use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers, KeyEventKind};
use std::io::{stdout, Write};
use std::collections::HashMap;

fn main() {
    let keyboard = BengaliKeyboard::new();
    
    println!("Bengali Phonetic Keyboard (Avro Layout)");
    println!("=======================================");
    println!("Type phonetically to get Bengali characters");
    println!("Press Esc to exit, Ctrl+L to clear screen");
    println!("Examples: 'ami' → 'আমি', 'bangla' → 'বাংলা'");
    println!();

    let mut input_buffer = String::new();
    let mut output_buffer = String::new();

    loop {
        if let Ok(Event::Key(KeyEvent { 
            code, 
            modifiers, 
            kind: KeyEventKind::Press,
            .. 
        })) = read() {
            match (code, modifiers) {
                (KeyCode::Esc, _) => {
                    println!("\nExiting Bengali Keyboard.");
                    break;
                }
                (KeyCode::Char('l'), KeyModifiers::CONTROL) => {
                    print!("\x1B[2J\x1B[1;1H"); // Clear screen
                    println!("Bengali Phonetic Keyboard - Screen Cleared");
                    input_buffer.clear();
                    output_buffer.clear();
                    stdout().flush().unwrap();
                }
                (KeyCode::Char(ch), _) => {
                    input_buffer.push(ch);
                    process_input(&mut input_buffer, &mut output_buffer, &keyboard);
                }
                (KeyCode::Enter, _) => {
                    // Process any remaining input before newline
                    if !input_buffer.is_empty() {
                        output_buffer.push_str(&input_buffer);
                        clear_and_print(&output_buffer, input_buffer.len());
                        input_buffer.clear();
                    }
                    print_and_flush("\n");
                    output_buffer.clear();
                }
                (KeyCode::Backspace, _) => {
                    if !input_buffer.is_empty() {
                        input_buffer.pop();
                        // Reprocess the buffer
                        let temp_output = output_buffer.clone();
                        output_buffer.clear();
                        process_input(&mut input_buffer, &mut output_buffer, &keyboard);
                        
                        // Clear previous output and print new
                        clear_line();
                        print_and_flush(&output_buffer);
                        if !input_buffer.is_empty() {
                            print_and_flush(&input_buffer);
                        }
                    } else if !output_buffer.is_empty() {
                        // Remove last character from output
                        output_buffer.pop();
                        clear_line();
                        print_and_flush(&output_buffer);
                    }
                }
                (KeyCode::Tab, _) => {
                    // Process any remaining input before tab
                    if !input_buffer.is_empty() {
                        output_buffer.push_str(&input_buffer);
                        input_buffer.clear();
                    }
                    output_buffer.push_str("    ");
                    clear_line();
                    print_and_flush(&output_buffer);
                }
                _ => {}
            }
        }
    }
}

fn process_input(input_buffer: &mut String, output_buffer: &mut String, keyboard: &BengaliKeyboard) {
    let mut temp_input = input_buffer.clone();
    let mut temp_output = output_buffer.clone();
    
    // Try to find matches and convert them
    while !temp_input.is_empty() {
        let mut matched = false;
        
        // Try to find the longest match starting from the beginning
        for len in (1..=temp_input.len()).rev() {
            let substr = &temp_input[..len];
            if let Some(bengali) = keyboard.find_exact_match(substr) {
                temp_output.push_str(&bengali);
                temp_input = temp_input[len..].to_string();
                matched = true;
                break;
            }
        }
        
        // If no match found, keep the first character as is
        if !matched {
            let first_char = temp_input.chars().next().unwrap();
            temp_output.push(first_char);
            temp_input = temp_input[first_char.len_utf8()..].to_string();
        }
    }
    
    // Clear the line and print the new output + remaining input
    clear_line();
    print_and_flush(&temp_output);
    if !temp_input.is_empty() {
        print_and_flush(&temp_input);
    } else {
        // Update buffers only when input is fully processed
        *output_buffer = temp_output;
        input_buffer.clear();
    }
}

fn clear_line() {
    print!("\r\x1B[K"); // Move to beginning of line and clear it
    stdout().flush().unwrap();
}

fn clear_and_print(text: &str, input_len: usize) {
    // Clear the current line
    for _ in 0..input_len {
        print!("\x08 \x08");
    }
    print_and_flush(text);
}

struct BengaliKeyboard {
    patterns: HashMap<String, String>,
}

impl BengaliKeyboard {
    fn new() -> Self {
        let mut patterns = HashMap::new();
        
        // Add all patterns
        Self::add_complex_letters(&mut patterns);
        Self::add_consonants(&mut patterns);
        Self::add_vowels(&mut patterns);
        Self::add_others(&mut patterns);
        
        Self { patterns }
    }
    
    fn add_vowels(patterns: &mut HashMap<String, String>) {
        patterns.extend([
            ("o".to_string(), "অ".to_string()),
            ("a".to_string(), "আ".to_string()),
            ("i".to_string(), "ই".to_string()),
            ("I".to_string(), "ঈ".to_string()),
            ("u".to_string(), "উ".to_string()),
            ("U".to_string(), "ঊ".to_string()),
            ("rri".to_string(), "ঋ".to_string()),
            ("e".to_string(), "এ".to_string()),
            ("oi".to_string(), "ঐ".to_string()),
            ("O".to_string(), "ও".to_string()),
            ("ou".to_string(), "ঔ".to_string()),
        ]);
    }
    
    fn add_consonants(patterns: &mut HashMap<String, String>) {
        patterns.extend([
            ("kh".to_string(), "খ".to_string()),
            ("k".to_string(), "ক".to_string()),
            ("gh".to_string(), "ঘ".to_string()),
            ("g".to_string(), "গ".to_string()),
            ("Ng".to_string(), "ঙ".to_string()),
            ("ch".to_string(), "ছ".to_string()),
            ("C".to_string(), "ছ".to_string()),
            ("c".to_string(), "চ".to_string()),
            ("jh".to_string(), "ঝ".to_string()),
            ("j".to_string(), "জ".to_string()),
            ("Y".to_string(), "ঞ".to_string()),
            ("Th".to_string(), "ঠ".to_string()),
            ("T".to_string(), "ট".to_string()),
            ("Dh".to_string(), "ঢ".to_string()),
            ("D".to_string(), "ড".to_string()),
            ("N".to_string(), "ণ".to_string()),
            ("th".to_string(), "থ".to_string()),
            ("t".to_string(), "ত".to_string()),
            ("dh".to_string(), "ধ".to_string()),
            ("d".to_string(), "দ".to_string()),
            ("n".to_string(), "ন".to_string()),
            ("ph".to_string(), "ফ".to_string()),
            ("f".to_string(), "ফ".to_string()),
            ("p".to_string(), "প".to_string()),
            ("bh".to_string(), "ভ".to_string()),
            ("v".to_string(), "ভ".to_string()),
            ("b".to_string(), "ব".to_string()),
            ("m".to_string(), "ম".to_string()),
            ("z".to_string(), "য".to_string()),
            ("r".to_string(), "র".to_string()),
            ("l".to_string(), "ল".to_string()),
            ("Sh".to_string(), "ষ".to_string()),
            ("sh".to_string(), "শ".to_string()),
            ("S".to_string(), "শ".to_string()),
            ("s".to_string(), "স".to_string()),
            ("h".to_string(), "হ".to_string()),
            ("Rh".to_string(), "ঢ়".to_string()),
            ("R".to_string(), "ড়".to_string()),
            ("y".to_string(), "য়".to_string()),
            (".t".to_string(), "ৎ".to_string()),
            ("ng".to_string(), "ং".to_string()),
            (":".to_string(), "ঃ".to_string()),
            ("H".to_string(), "ঃ".to_string()),
            (".n".to_string(), "ঁ".to_string()),
        ]);
    }
    
    fn add_complex_letters(patterns: &mut HashMap<String, String>) {
        patterns.extend([
            // Complex letters (keeping all from your specification)
            ("kShN".to_string(), "ক্ষ্ণ".to_string()),
            ("kShw".to_string(), "ক্ষ্ব".to_string()),
            ("kShm".to_string(), "ক্ষ্ম".to_string()),
            ("kSh".to_string(), "ক্ষ".to_string()),
            ("kkh".to_string(), "ক্ষ".to_string()),
            ("NgkSh".to_string(), "ঙ্ক্ষ".to_string()),
            ("Ngkt".to_string(), "ঙ্ক্ত".to_string()),
            ("Ngkh".to_string(), "ঙ্খ".to_string()),
            ("Nggh".to_string(), "ঙ্ঘ".to_string()),
            ("Ngk".to_string(), "ঙ্ক".to_string()),
            ("Ngg".to_string(), "ঙ্গ".to_string()),
            ("Ngm".to_string(), "ঙ্ম".to_string()),
            ("cCw".to_string(), "চ্ছ্ব".to_string()),
            ("cC".to_string(), "চ্ছ".to_string()),
            ("cY".to_string(), "চ্ঞ".to_string()),
            ("cw".to_string(), "চ্ব".to_string()),
            ("cc".to_string(), "চ্চ".to_string()),
            ("jjw".to_string(), "জ্জ্ব".to_string()),
            ("jjh".to_string(), "জ্ঝ".to_string()),
            ("jY".to_string(), "জ্ঞ".to_string()),
            ("gg".to_string(), "জ্ঞ".to_string()),
            ("jw".to_string(), "জ্ব".to_string()),
            ("jj".to_string(), "জ্জ".to_string()),
            ("Yjh".to_string(), "ঞ্ঝ".to_string()),
            ("Yc".to_string(), "ঞ্চ".to_string()),
            ("nc".to_string(), "ঞ্চ".to_string()),
            ("YC".to_string(), "ঞ্ছ".to_string()),
            ("nC".to_string(), "ঞ্ছ".to_string()),
            ("Yj".to_string(), "ঞ্জ".to_string()),
            ("nj".to_string(), "ঞ্জ".to_string()),
            ("TT".to_string(), "ট্ট".to_string()),
            ("Tw".to_string(), "ট্ব".to_string()),
            ("Tm".to_string(), "ট্ম".to_string()),
            ("DD".to_string(), "ড্ড".to_string()),
            ("Dw".to_string(), "ড্ব".to_string()),
            ("NTh".to_string(), "ণ্ঠ".to_string()),
            ("NDh".to_string(), "ণ্ঢ".to_string()),
            ("NT".to_string(), "ণ্ট".to_string()),
            ("ND".to_string(), "ণ্ড".to_string()),
            ("NN".to_string(), "ণ্ণ".to_string()),
            ("Nw".to_string(), "ণ্ব".to_string()),
            ("Nm".to_string(), "ণ্ম".to_string()),
            ("ttw".to_string(), "ত্ত্ব".to_string()),
            ("tth".to_string(), "ত্থ".to_string()),
            ("tk".to_string(), "ৎক".to_string()),
            ("tt".to_string(), "ত্ত".to_string()),
            ("tn".to_string(), "ত্ন".to_string()),
            ("tw".to_string(), "ত্ব".to_string()),
            ("tm".to_string(), "ত্ম".to_string()),
            ("tl".to_string(), "ৎল".to_string()),
            ("ts".to_string(), "ৎস".to_string()),
            ("thw".to_string(), "থ্ব".to_string()),
            ("ddw".to_string(), "দ্দ্ব".to_string()),
            ("ddh".to_string(), "দ্ধ".to_string()),
            ("dgh".to_string(), "দ্ঘ".to_string()),
            ("dg".to_string(), "দ্গ".to_string()),
            ("dd".to_string(), "দ্দ".to_string()),
            ("dw".to_string(), "দ্ব".to_string()),
            ("dv".to_string(), "দ্ভ".to_string()),
            ("dm".to_string(), "দ্ম".to_string()),
            ("dhn".to_string(), "ধ্ন".to_string()),
            ("dhw".to_string(), "ধ্ব".to_string()),
            ("dhm".to_string(), "ধ্ম".to_string()),
            ("nTh".to_string(), "ন্ঠ".to_string()),
            ("ntw".to_string(), "ন্ত্ব".to_string()),
            ("ndw".to_string(), "ন্দ্ব".to_string()),
            ("nT".to_string(), "ন্ট".to_string()),
            ("nD".to_string(), "ন্ড".to_string()),
            ("nt".to_string(), "ন্ত".to_string()),
            ("nth".to_string(), "ন্থ".to_string()),
            ("nd".to_string(), "ন্দ".to_string()),
            ("ndh".to_string(), "ন্ধ".to_string()),
            ("nn".to_string(), "ন্ন".to_string()),
            ("nw".to_string(), "ন্ব".to_string()),
            ("nm".to_string(), "ন্ম".to_string()),
            ("pT".to_string(), "প্ট".to_string()),
            ("pt".to_string(), "প্ত".to_string()),
            ("pn".to_string(), "প্ন".to_string()),
            ("pp".to_string(), "প্প".to_string()),
            ("pl".to_string(), "প্ল".to_string()),
            ("ps".to_string(), "প্স".to_string()),
            ("fl".to_string(), "ফ্ল".to_string()),
            ("bj".to_string(), "ব্জ".to_string()),
            ("bd".to_string(), "ব্দ".to_string()),
            ("bdh".to_string(), "ব্ধ".to_string()),
            ("bb".to_string(), "ব্ব".to_string()),
            ("bl".to_string(), "ব্ল".to_string()),
            ("vw".to_string(), "ভ্ব".to_string()),
            ("mn".to_string(), "ম্ন".to_string()),
            ("mp".to_string(), "ম্প".to_string()),
            ("mf".to_string(), "ম্ফ".to_string()),
            ("mb".to_string(), "ম্ব".to_string()),
            ("mv".to_string(), "ম্ভ".to_string()),
            ("mm".to_string(), "ম্ম".to_string()),
            ("ml".to_string(), "ম্ল".to_string()),
            ("lk".to_string(), "ল্ক".to_string()),
            ("lg".to_string(), "ল্গ".to_string()),
            ("lT".to_string(), "ল্ট".to_string()),
            ("lD".to_string(), "ল্ড".to_string()),
            ("lp".to_string(), "ল্প".to_string()),
            ("lf".to_string(), "ল্ফ".to_string()),
            ("lw".to_string(), "ল্ব".to_string()),
            ("lv".to_string(), "ল্ভ".to_string()),
            ("lm".to_string(), "ল্ম".to_string()),
            ("ll".to_string(), "ল্ল".to_string()),
            ("shc".to_string(), "শ্চ".to_string()),
            ("shC".to_string(), "শ্ছ".to_string()),
            ("shn".to_string(), "শ্ন".to_string()),
            ("shw".to_string(), "শ্ব".to_string()),
            ("shm".to_string(), "শ্ম".to_string()),
            ("shl".to_string(), "শ্ল".to_string()),
            ("ShTh".to_string(), "ষ্ঠ".to_string()),
            ("Shk".to_string(), "ষ্ক".to_string()),
            ("ShT".to_string(), "ষ্ট".to_string()),
            ("ShN".to_string(), "ষ্ণ".to_string()),
            ("Shp".to_string(), "ষ্প".to_string()),
            ("Shf".to_string(), "ষ্ফ".to_string()),
            ("Shw".to_string(), "ষ্ব".to_string()),
            ("Shm".to_string(), "ষ্ম".to_string()),
            ("skh".to_string(), "স্খ".to_string()),
            ("spl".to_string(), "স্প্ল".to_string()),
            ("stw".to_string(), "স্ত্ব".to_string()),
            ("sth".to_string(), "স্থ".to_string()),
            ("sk".to_string(), "স্ক".to_string()),
            ("sT".to_string(), "স্ট".to_string()),
            ("st".to_string(), "স্ত".to_string()),
            ("sn".to_string(), "স্ন".to_string()),
            ("sp".to_string(), "স্প".to_string()),
            ("sf".to_string(), "স্ফ".to_string()),
            ("sw".to_string(), "স্ব".to_string()),
            ("sm".to_string(), "স্ম".to_string()),
            ("sl".to_string(), "স্ল".to_string()),
            ("hN".to_string(), "হ্ণ".to_string()),
            ("hn".to_string(), "হ্ন".to_string()),
            ("hw".to_string(), "হ্ব".to_string()),
            ("hm".to_string(), "হ্ম".to_string()),
            ("hl".to_string(), "হ্ল".to_string()),
            ("kk".to_string(), "ক্ক".to_string()),
            ("kT".to_string(), "ক্ট".to_string()),
            ("kt".to_string(), "ক্ত".to_string()),
            ("kw".to_string(), "ক্ব".to_string()),
            ("km".to_string(), "ক্ম".to_string()),
            ("kl".to_string(), "ক্ল".to_string()),
            ("ks".to_string(), "ক্স".to_string()),
            ("gN".to_string(), "গ্ণ".to_string()),
            ("gdh".to_string(), "গ্ধ".to_string()),
            ("gn".to_string(), "গ্ন".to_string()),
            ("gw".to_string(), "গ্ব".to_string()),
            ("gm".to_string(), "গ্ম".to_string()),
            ("gl".to_string(), "গ্ল".to_string()),
            ("ghn".to_string(), "ঘ্ন".to_string()),
        ]);
    }
    
    fn add_others(patterns: &mut HashMap<String, String>) {
        patterns.extend([
            ("aya".to_string(), "অ্যা".to_string()),
            (".".to_string(), "।".to_string()),
            ("_".to_string(), "্".to_string()),
            ("$".to_string(), "৳".to_string()),
            ("0".to_string(), "০".to_string()),
            ("1".to_string(), "১".to_string()),
            ("2".to_string(), "২".to_string()),
            ("3".to_string(), "৩".to_string()),
            ("4".to_string(), "৪".to_string()),
            ("5".to_string(), "৫".to_string()),
            ("6".to_string(), "৬".to_string()),
            ("7".to_string(), "৭".to_string()),
            ("8".to_string(), "৮".to_string()),
            ("9".to_string(), "৯".to_string()),
        ]);
    }
    
    fn find_exact_match(&self, input: &str) -> Option<String> {
        self.patterns.get(input).cloned()
    }
}

fn print_and_flush(s: &str) {
    print!("{}", s);
    stdout().flush().unwrap();
}