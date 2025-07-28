fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico");
        res.set("ProductName", "Bengali Phonetic Keyboard");
        res.set("FileDescription", "System-wide Bengali Input Method");
        res.set("CompanyName", "Your Name");
        res.compile().unwrap();
    }
}