#![windows_subsystem = "windows"]

mod keymap;

use keymap::KeyMap;
use std::sync::Arc;
use parking_lot::Mutex;
use once_cell::sync::Lazy;
use winapi::um::winuser::*;
use winapi::um::shellapi::*;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::shared::windef::*;
use winapi::shared::minwindef::*;
use std::ptr;
use std::mem;

// Global state
static KEYBOARD_STATE: Lazy<Arc<Mutex<KeyboardState>>> = Lazy::new(|| {
    Arc::new(Mutex::new(KeyboardState::new()))
});

static BENGALI_KEYBOARD: Lazy<BengaliKeyboard> = Lazy::new(|| BengaliKeyboard::new());

const WM_TRAYICON: u32 = WM_USER + 1;
const ID_TOGGLE: u32 = 1001;
const ID_EXIT: u32 = 1002;
const TOGGLE_KEY: u32 = VK_F10 as u32;

struct KeyboardState {
    enabled: bool,
    input_buffer: String,
    last_bengali_output: String,
}

impl KeyboardState {
    fn new() -> Self {
        Self {
            enabled: false,
            input_buffer: String::new(),
            last_bengali_output: String::new(),
        }
    }
}

struct BengaliKeyboard {
    keymap: KeyMap,
}

impl BengaliKeyboard {
    fn new() -> Self {
        Self {
            keymap: KeyMap::new(),
        }
    }

    fn convert_text(&self, input: &str) -> String {
        let mut result = String::new();
        let mut i = 0;
        let chars: Vec<char> = input.chars().collect();
        let mut last_was_consonant = false;
        
        while i < chars.len() {
            let mut found_match = false;
            let mut longest_match = None;
            let mut longest_len = 0;
            
            // Try to find the longest matching pattern starting from position i
            for (pattern, bengali_char) in &self.keymap.patterns {
                let pattern_chars: Vec<char> = pattern.chars().collect();
                
                // Check if pattern matches at current position
                if i + pattern_chars.len() <= chars.len() {
                    let slice: String = chars[i..i + pattern_chars.len()].iter().collect();
                    if slice == *pattern && pattern_chars.len() > longest_len {
                        longest_match = Some((pattern, bengali_char, pattern_chars.len()));
                        longest_len = pattern_chars.len();
                    }
                }
            }
            
            if let Some((pattern, bengali_char, len)) = longest_match {
                if bengali_char.is_vowel && last_was_consonant {
                    if let Some(diacritic) = self.keymap.vowel_diacritics.get(pattern) {
                        result.push_str(diacritic);
                    } else {
                        result.push_str(&bengali_char.bengali);
                    }
                } else {
                    result.push_str(&bengali_char.bengali);
                }
                
                last_was_consonant = bengali_char.is_consonant;
                i += len;
                found_match = true;
            }
            
            if !found_match {
                result.push(chars[i]);
                last_was_consonant = false;
                i += 1;
            }
        }
        
        result
    }
}

fn main() {
    unsafe {
        let h_instance = GetModuleHandleW(ptr::null());
        
        let class_name = wide_string("BengaliKeyboardClass");
        let wc = WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: ptr::null_mut(),
            hCursor: LoadCursorW(ptr::null_mut(), IDC_ARROW),
            hbrBackground: ptr::null_mut(),
            lpszMenuName: ptr::null(),
            lpszClassName: class_name.as_ptr(),
        };

        RegisterClassW(&wc);

        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            wide_string("Bengali Keyboard").as_ptr(),
            0,
            0, 0, 0, 0,
            ptr::null_mut(),
            ptr::null_mut(),
            h_instance,
            ptr::null_mut(),
        );

        let hook = SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(keyboard_hook_proc),
            h_instance,
            0,
        );

        if hook.is_null() {
            return;
        }

        create_tray_icon(hwnd);

        let mut msg = mem::zeroed();
        while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        UnhookWindowsHookEx(hook);
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_TRAYICON => {
            if lparam as UINT == WM_RBUTTONUP {
                show_context_menu(hwnd);
            }
            0
        }
        WM_COMMAND => {
            match LOWORD(wparam as u32) as u32 {
                ID_TOGGLE => {
                    toggle_keyboard();
                    update_tray_icon(hwnd);
                }
                ID_EXIT => {
                    PostQuitMessage(0);
                }
                _ => {}
            }
            0
        }
        WM_DESTROY => {
            remove_tray_icon(hwnd);
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

unsafe extern "system" fn keyboard_hook_proc(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if code >= 0 {
        let kbd_struct = *(lparam as *const KBDLLHOOKSTRUCT);
        let vk_code = kbd_struct.vkCode;
        
        // Check for toggle key (F10)
        if wparam == WM_KEYDOWN as usize && vk_code == TOGGLE_KEY {
            toggle_keyboard();
            return 1;
        }
        
        // Check for Ctrl key combinations
        let ctrl_pressed = (GetAsyncKeyState(VK_CONTROL) & 0x8000u16 as i16) != 0;
        if ctrl_pressed && wparam == WM_KEYDOWN as usize {
            match vk_code {
                0x43 => return 0, // Ctrl+C
                0x56 => return 0, // Ctrl+V
                0x41 => return 0, // Ctrl+A
                0x58 => return 0, // Ctrl+X
                0x5A => return 0, // Ctrl+Z
                0x59 => return 0, // Ctrl+Y
                _ => {}
            }
        }
        
        let state = KEYBOARD_STATE.lock();
        if state.enabled && wparam == WM_KEYDOWN as usize {
            if let Some(ch) = vk_to_char(vk_code) {
                drop(state);
                if process_character(ch) {
                    return 1;
                }
            }
        }
    }
    
    CallNextHookEx(ptr::null_mut(), code, wparam, lparam)
}

fn process_character(ch: char) -> bool {
    let mut state = KEYBOARD_STATE.lock();
    
    if ch == '\x08' { // Backspace
        if !state.input_buffer.is_empty() {
            state.input_buffer.pop();
            
            if state.input_buffer.is_empty() {
                // Clear the last Bengali output
                let chars_to_remove = state.last_bengali_output.chars().count();
                state.last_bengali_output.clear();
                
                drop(state);
                
                unsafe {
                    for _ in 0..chars_to_remove {
                        send_backspace();
                    }
                }
                return true;
            } else {
                // Recalculate Bengali text for remaining buffer
                let new_bengali = BENGALI_KEYBOARD.convert_text(&state.input_buffer);
                let chars_to_remove = state.last_bengali_output.chars().count();
                state.last_bengali_output = new_bengali.clone();
                
                drop(state);
                
                unsafe {
                    // Remove previous Bengali text
                    for _ in 0..chars_to_remove {
                        send_backspace();
                    }
                    
                    // Send the new Bengali text
                    if !new_bengali.is_empty() {
                        send_unicode_text(&new_bengali);
                    }
                }
                return true;
            }
        }
        return false;
    } else if ch == ' ' || ch == '\n' || ch == '\t' {
        // Word boundary - process current word and allow the space/newline/tab
        if !state.input_buffer.is_empty() {
            let word = state.input_buffer.clone();
            let bengali_word = BENGALI_KEYBOARD.convert_text(&word);
            
            // Clear the buffer
            state.input_buffer.clear();
            state.last_bengali_output.clear();
            
            // If we have a valid Bengali conversion, send it
            if !bengali_word.is_empty() && bengali_word != word {
                drop(state);
                
                unsafe {
                    // Remove the English word
                    for _ in 0..word.len() {
                        send_backspace();
                    }
                    
                    // Send the Bengali word
                    send_unicode_text(&bengali_word);
                }
                return true; // Suppress the space/newline/tab
            }
        }
        return false; // Allow the space/newline/tab
    } else if ch.is_ascii_alphabetic() || ch.is_ascii_digit() || ".:$_".contains(ch) {
        // Add character to buffer
        state.input_buffer.push(ch);
        
        // Convert the entire buffer to Bengali
        let new_bengali = BENGALI_KEYBOARD.convert_text(&state.input_buffer);
        
        // Check if we have a valid Bengali conversion
        if !new_bengali.is_empty() && new_bengali != state.input_buffer {
            let chars_to_remove = state.last_bengali_output.chars().count();
            let input_buffer_len = state.input_buffer.len();
            state.last_bengali_output = new_bengali.clone();
            
            drop(state);
            
            unsafe {
                // Remove previous Bengali text
                for _ in 0..chars_to_remove {
                    send_backspace();
                }
                
                // Remove the English characters that were just typed
                for _ in 0..input_buffer_len {
                    send_backspace();
                }
                
                // Send the Bengali text
                send_unicode_text(&new_bengali);
            }
            
            return true;
        }
        
        return false;
    } else {
        // Non-matching character, clear buffer
        if !state.input_buffer.is_empty() {
            state.input_buffer.clear();
            state.last_bengali_output.clear();
        }
    }
    
    false
}

unsafe fn send_backspace() {
    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: mem::zeroed(),
    };
    
    *input.u.ki_mut() = KEYBDINPUT {
        wVk: VK_BACK as u16,
        wScan: 0,
        dwFlags: 0,
        time: 0,
        dwExtraInfo: 0,
    };
    
    SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
    
    input.u.ki_mut().dwFlags = KEYEVENTF_KEYUP;
    SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
}

unsafe fn send_unicode_text(text: &str) {
    for ch in text.chars() {
        let mut input = INPUT {
            type_: INPUT_KEYBOARD,
            u: mem::zeroed(),
        };
        
        *input.u.ki_mut() = KEYBDINPUT {
            wVk: 0,
            wScan: ch as u16,
            dwFlags: KEYEVENTF_UNICODE,
            time: 0,
            dwExtraInfo: 0,
        };
        
        SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
        
        input.u.ki_mut().dwFlags = KEYEVENTF_UNICODE | KEYEVENTF_KEYUP;
        SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
    }
}



unsafe fn create_tray_icon(hwnd: HWND) {
    let mut nid: NOTIFYICONDATAW = mem::zeroed();
    nid.cbSize = mem::size_of::<NOTIFYICONDATAW>() as u32;
    nid.hWnd = hwnd;
    nid.uID = 1;
    nid.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;
    nid.uCallbackMessage = WM_TRAYICON;
    
    // Use default icon for now
    nid.hIcon = LoadIconW(ptr::null_mut(), IDI_APPLICATION);
    
    let state = KEYBOARD_STATE.lock();
    let tooltip = if state.enabled {
        wide_string("Bengali Keyboard - Enabled (F10 to toggle)")
    } else {
        wide_string("Bengali Keyboard - Disabled (F10 to toggle)")
    };
    
    for (i, &ch) in tooltip.iter().take(127).enumerate() {
        nid.szTip[i] = ch;
    }
    
    Shell_NotifyIconW(NIM_ADD, &mut nid);
}

unsafe fn update_tray_icon(hwnd: HWND) {
    let mut nid: NOTIFYICONDATAW = mem::zeroed();
    nid.cbSize = mem::size_of::<NOTIFYICONDATAW>() as u32;
    nid.hWnd = hwnd;
    nid.uID = 1;
    nid.uFlags = NIF_ICON | NIF_TIP;
    
    let state = KEYBOARD_STATE.lock();
    
    // Use default icon for now
    nid.hIcon = LoadIconW(ptr::null_mut(), IDI_APPLICATION);
    
    let tooltip = if state.enabled {
        wide_string("Bengali Keyboard - Enabled (F10 to toggle)")
    } else {
        wide_string("Bengali Keyboard - Disabled (F10 to toggle)")
    };
    
    for (i, &ch) in tooltip.iter().take(127).enumerate() {
        nid.szTip[i] = ch;
    }
    
    Shell_NotifyIconW(NIM_MODIFY, &mut nid);
}

unsafe fn remove_tray_icon(hwnd: HWND) {
    let mut nid: NOTIFYICONDATAW = mem::zeroed();
    nid.cbSize = mem::size_of::<NOTIFYICONDATAW>() as u32;
    nid.hWnd = hwnd;
    nid.uID = 1;
    
    Shell_NotifyIconW(NIM_DELETE, &mut nid);
}

unsafe fn show_context_menu(hwnd: HWND) {
    let hmenu = CreatePopupMenu();
    let state = KEYBOARD_STATE.lock();
    
    let toggle_text = if state.enabled {
        wide_string("Disable Bengali Keyboard")
    } else {
        wide_string("Enable Bengali Keyboard")
    };
    drop(state);
    
    AppendMenuW(hmenu, MF_STRING, ID_TOGGLE as usize, toggle_text.as_ptr());
    AppendMenuW(hmenu, MF_SEPARATOR, 0, ptr::null());
    AppendMenuW(hmenu, MF_STRING, ID_EXIT as usize, wide_string("Exit").as_ptr());
    
    let mut pt = POINT { x: 0, y: 0 };
    GetCursorPos(&mut pt);
    
    SetForegroundWindow(hwnd);
    TrackPopupMenu(
        hmenu,
        TPM_RIGHTBUTTON,
        pt.x,
        pt.y,
        0,
        hwnd,
        ptr::null(),
    );
    
    DestroyMenu(hmenu);
}

fn toggle_keyboard() {
    let mut state = KEYBOARD_STATE.lock();
    state.enabled = !state.enabled;
    state.input_buffer.clear();
    state.last_bengali_output.clear();
}

fn vk_to_char(vk_code: u32) -> Option<char> {
    match vk_code {
        0x41..=0x5A => Some((vk_code - 0x41 + b'a' as u32) as u8 as char),
        0x30..=0x39 => Some((vk_code - 0x30 + b'0' as u32) as u8 as char),
        0x08 => Some('\x08'),
        0x20 => Some(' '),
        0x0D => Some('\n'),
        0x09 => Some('\t'),
        0xBE => Some('.'),
        0xBA => Some(':'),
        0xBD => Some('_'),
        _ => None,
    }
}

fn wide_string(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}