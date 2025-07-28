use std::collections::HashMap;

#[derive(Clone)]
pub struct BengaliChar {
    pub bengali: String,
    pub is_consonant: bool,
    pub is_vowel: bool,
}

pub struct KeyMap {
    pub patterns: HashMap<String, BengaliChar>,
    pub vowel_diacritics: HashMap<String, String>,
}

impl KeyMap {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        let mut vowel_diacritics = HashMap::new();
        
        // Independent vowels (স্বরবর্ণ)
        patterns.extend([
            ("o".to_string(), BengaliChar { bengali: "অ".to_string(), is_consonant: false, is_vowel: true }),
            ("a".to_string(), BengaliChar { bengali: "আ".to_string(), is_consonant: false, is_vowel: true }),
            ("i".to_string(), BengaliChar { bengali: "ই".to_string(), is_consonant: false, is_vowel: true }),
            ("I".to_string(), BengaliChar { bengali: "ঈ".to_string(), is_consonant: false, is_vowel: true }),
            ("u".to_string(), BengaliChar { bengali: "উ".to_string(), is_consonant: false, is_vowel: true }),
            ("U".to_string(), BengaliChar { bengali: "ঊ".to_string(), is_consonant: false, is_vowel: true }),
            ("rri".to_string(), BengaliChar { bengali: "ঋ".to_string(), is_consonant: false, is_vowel: true }),
            ("e".to_string(), BengaliChar { bengali: "এ".to_string(), is_consonant: false, is_vowel: true }),
            ("oi".to_string(), BengaliChar { bengali: "ঐ".to_string(), is_consonant: false, is_vowel: true }),
            ("O".to_string(), BengaliChar { bengali: "ও".to_string(), is_consonant: false, is_vowel: true }),
            ("ou".to_string(), BengaliChar { bengali: "ঔ".to_string(), is_consonant: false, is_vowel: true }),
        ]);

        // Vowel diacritics (কার) - used after consonants
        vowel_diacritics.extend([
            ("a".to_string(), "া".to_string()),   // আ-কার
            ("i".to_string(), "ি".to_string()),   // ই-কার
            ("I".to_string(), "ী".to_string()),   // ঈ-কার
            ("u".to_string(), "ু".to_string()),   // উ-কার
            ("U".to_string(), "ূ".to_string()),   // ঊ-কার
            ("rri".to_string(), "ৃ".to_string()), // ঋ-কার
            ("e".to_string(), "ে".to_string()),   // এ-কার
            ("oi".to_string(), "ৈ".to_string()),  // ঐ-কার
            ("O".to_string(), "ো".to_string()),   // ও-কার
            ("ou".to_string(), "ৌ".to_string()),  // ঔ-কার
        ]);

        // Consonants (ব্যঞ্জনবর্ণ)
        patterns.extend([
            ("kh".to_string(), BengaliChar { bengali: "খ".to_string(), is_consonant: true, is_vowel: false }),
            ("k".to_string(), BengaliChar { bengali: "ক".to_string(), is_consonant: true, is_vowel: false }),
            ("gh".to_string(), BengaliChar { bengali: "ঘ".to_string(), is_consonant: true, is_vowel: false }),
            ("g".to_string(), BengaliChar { bengali: "গ".to_string(), is_consonant: true, is_vowel: false }),
            ("Ng".to_string(), BengaliChar { bengali: "ঙ".to_string(), is_consonant: true, is_vowel: false }),
            ("ch".to_string(), BengaliChar { bengali: "ছ".to_string(), is_consonant: true, is_vowel: false }),
            ("C".to_string(), BengaliChar { bengali: "ছ".to_string(), is_consonant: true, is_vowel: false }),
            ("c".to_string(), BengaliChar { bengali: "চ".to_string(), is_consonant: true, is_vowel: false }),
            ("jh".to_string(), BengaliChar { bengali: "ঝ".to_string(), is_consonant: true, is_vowel: false }),
            ("j".to_string(), BengaliChar { bengali: "জ".to_string(), is_consonant: true, is_vowel: false }),
            ("Y".to_string(), BengaliChar { bengali: "ঞ".to_string(), is_consonant: true, is_vowel: false }),
            ("Th".to_string(), BengaliChar { bengali: "ঠ".to_string(), is_consonant: true, is_vowel: false }),
            ("T".to_string(), BengaliChar { bengali: "ট".to_string(), is_consonant: true, is_vowel: false }),
            ("Dh".to_string(), BengaliChar { bengali: "ঢ".to_string(), is_consonant: true, is_vowel: false }),
            ("D".to_string(), BengaliChar { bengali: "ড".to_string(), is_consonant: true, is_vowel: false }),
            ("N".to_string(), BengaliChar { bengali: "ণ".to_string(), is_consonant: true, is_vowel: false }),
            ("th".to_string(), BengaliChar { bengali: "থ".to_string(), is_consonant: true, is_vowel: false }),
            ("t".to_string(), BengaliChar { bengali: "ত".to_string(), is_consonant: true, is_vowel: false }),
            ("dh".to_string(), BengaliChar { bengali: "ধ".to_string(), is_consonant: true, is_vowel: false }),
            ("d".to_string(), BengaliChar { bengali: "দ".to_string(), is_consonant: true, is_vowel: false }),
            ("n".to_string(), BengaliChar { bengali: "ন".to_string(), is_consonant: true, is_vowel: false }),
            ("ph".to_string(), BengaliChar { bengali: "ফ".to_string(), is_consonant: true, is_vowel: false }),
            ("f".to_string(), BengaliChar { bengali: "ফ".to_string(), is_consonant: true, is_vowel: false }),
            ("p".to_string(), BengaliChar { bengali: "প".to_string(), is_consonant: true, is_vowel: false }),
            ("bh".to_string(), BengaliChar { bengali: "ভ".to_string(), is_consonant: true, is_vowel: false }),
            ("v".to_string(), BengaliChar { bengali: "ভ".to_string(), is_consonant: true, is_vowel: false }),
            ("b".to_string(), BengaliChar { bengali: "ব".to_string(), is_consonant: true, is_vowel: false }),
            ("m".to_string(), BengaliChar { bengali: "ম".to_string(), is_consonant: true, is_vowel: false }),
            ("z".to_string(), BengaliChar { bengali: "য".to_string(), is_consonant: true, is_vowel: false }),
            ("r".to_string(), BengaliChar { bengali: "র".to_string(), is_consonant: true, is_vowel: false }),
            ("l".to_string(), BengaliChar { bengali: "ল".to_string(), is_consonant: true, is_vowel: false }),
            ("Sh".to_string(), BengaliChar { bengali: "ষ".to_string(), is_consonant: true, is_vowel: false }),
            ("sh".to_string(), BengaliChar { bengali: "শ".to_string(), is_consonant: true, is_vowel: false }),
            ("S".to_string(), BengaliChar { bengali: "শ".to_string(), is_consonant: true, is_vowel: false }),
            ("s".to_string(), BengaliChar { bengali: "স".to_string(), is_consonant: true, is_vowel: false }),
            ("h".to_string(), BengaliChar { bengali: "হ".to_string(), is_consonant: true, is_vowel: false }),
            ("Rh".to_string(), BengaliChar { bengali: "ঢ়".to_string(), is_consonant: true, is_vowel: false }),
            ("R".to_string(), BengaliChar { bengali: "ড়".to_string(), is_consonant: true, is_vowel: false }),
            ("y".to_string(), BengaliChar { bengali: "য়".to_string(), is_consonant: true, is_vowel: false }),
            (".t".to_string(), BengaliChar { bengali: "ৎ".to_string(), is_consonant: true, is_vowel: false }),
            ("ng".to_string(), BengaliChar { bengali: "ং".to_string(), is_consonant: false, is_vowel: false }),
            (":".to_string(), BengaliChar { bengali: "ঃ".to_string(), is_consonant: false, is_vowel: false }),
            ("H".to_string(), BengaliChar { bengali: "ঃ".to_string(), is_consonant: false, is_vowel: false }),
            (".n".to_string(), BengaliChar { bengali: "ঁ".to_string(), is_consonant: false, is_vowel: false }),
        ]);

        // Complex letters (conjuncts)
        patterns.extend([
            ("kSh".to_string(), BengaliChar { bengali: "ক্ষ".to_string(), is_consonant: true, is_vowel: false }),
            ("kkh".to_string(), BengaliChar { bengali: "ক্ষ".to_string(), is_consonant: true, is_vowel: false }),
            ("jY".to_string(), BengaliChar { bengali: "জ্ঞ".to_string(), is_consonant: true, is_vowel: false }),
            ("gg".to_string(), BengaliChar { bengali: "জ্ঞ".to_string(), is_consonant: true, is_vowel: false }),
            ("kk".to_string(), BengaliChar { bengali: "ক্ক".to_string(), is_consonant: true, is_vowel: false }),
            ("kT".to_string(), BengaliChar { bengali: "ক্ট".to_string(), is_consonant: true, is_vowel: false }),
            ("kt".to_string(), BengaliChar { bengali: "ক্ত".to_string(), is_consonant: true, is_vowel: false }),
            ("kw".to_string(), BengaliChar { bengali: "ক্ব".to_string(), is_consonant: true, is_vowel: false }),
            ("km".to_string(), BengaliChar { bengali: "ক্ম".to_string(), is_consonant: true, is_vowel: false }),
            ("kl".to_string(), BengaliChar { bengali: "ক্ল".to_string(), is_consonant: true, is_vowel: false }),
            ("ks".to_string(), BengaliChar { bengali: "ক্স".to_string(), is_consonant: true, is_vowel: false }),
            ("tt".to_string(), BengaliChar { bengali: "ত্ত".to_string(), is_consonant: true, is_vowel: false }),
            ("tn".to_string(), BengaliChar { bengali: "ত্ন".to_string(), is_consonant: true, is_vowel: false }),
            ("tw".to_string(), BengaliChar { bengali: "ত্ব".to_string(), is_consonant: true, is_vowel: false }),
            ("tm".to_string(), BengaliChar { bengali: "ত্ম".to_string(), is_consonant: true, is_vowel: false }),
            ("dd".to_string(), BengaliChar { bengali: "দ্দ".to_string(), is_consonant: true, is_vowel: false }),
            ("dw".to_string(), BengaliChar { bengali: "দ্ব".to_string(), is_consonant: true, is_vowel: false }),
            ("dm".to_string(), BengaliChar { bengali: "দ্ম".to_string(), is_consonant: true, is_vowel: false }),
            ("nn".to_string(), BengaliChar { bengali: "ন্ন".to_string(), is_consonant: true, is_vowel: false }),
            ("nt".to_string(), BengaliChar { bengali: "ন্ত".to_string(), is_consonant: true, is_vowel: false }),
            ("nd".to_string(), BengaliChar { bengali: "ন্দ".to_string(), is_consonant: true, is_vowel: false }),
            ("nw".to_string(), BengaliChar { bengali: "ন্ব".to_string(), is_consonant: true, is_vowel: false }),
            ("nm".to_string(), BengaliChar { bengali: "ন্ম".to_string(), is_consonant: true, is_vowel: false }),
            ("pp".to_string(), BengaliChar { bengali: "প্প".to_string(), is_consonant: true, is_vowel: false }),
            ("pt".to_string(), BengaliChar { bengali: "প্ত".to_string(), is_consonant: true, is_vowel: false }),
            ("pl".to_string(), BengaliChar { bengali: "প্ল".to_string(), is_consonant: true, is_vowel: false }),
            ("bb".to_string(), BengaliChar { bengali: "ব্ব".to_string(), is_consonant: true, is_vowel: false }),
            ("bd".to_string(), BengaliChar { bengali: "ব্দ".to_string(), is_consonant: true, is_vowel: false }),
            ("bl".to_string(), BengaliChar { bengali: "ব্ল".to_string(), is_consonant: true, is_vowel: false }),
            ("mm".to_string(), BengaliChar { bengali: "ম্ম".to_string(), is_consonant: true, is_vowel: false }),
            ("mp".to_string(), BengaliChar { bengali: "ম্প".to_string(), is_consonant: true, is_vowel: false }),
            ("mb".to_string(), BengaliChar { bengali: "ম্ব".to_string(), is_consonant: true, is_vowel: false }),
            ("ml".to_string(), BengaliChar { bengali: "ম্ল".to_string(), is_consonant: true, is_vowel: false }),
            ("ll".to_string(), BengaliChar { bengali: "ল্ল".to_string(), is_consonant: true, is_vowel: false }),
            ("lk".to_string(), BengaliChar { bengali: "ল্ক".to_string(), is_consonant: true, is_vowel: false }),
            ("lg".to_string(), BengaliChar { bengali: "ল্গ".to_string(), is_consonant: true, is_vowel: false }),
            ("lp".to_string(), BengaliChar { bengali: "ল্প".to_string(), is_consonant: true, is_vowel: false }),
            ("lw".to_string(), BengaliChar { bengali: "ল্ব".to_string(), is_consonant: true, is_vowel: false }),
            ("lm".to_string(), BengaliChar { bengali: "ল্ম".to_string(), is_consonant: true, is_vowel: false }),
            ("sk".to_string(), BengaliChar { bengali: "স্ক".to_string(), is_consonant: true, is_vowel: false }),
            ("st".to_string(), BengaliChar { bengali: "স্ত".to_string(), is_consonant: true, is_vowel: false }),
            ("sn".to_string(), BengaliChar { bengali: "স্ন".to_string(), is_consonant: true, is_vowel: false }),
            ("sp".to_string(), BengaliChar { bengali: "স্প".to_string(), is_consonant: true, is_vowel: false }),
            ("sw".to_string(), BengaliChar { bengali: "স্ব".to_string(), is_consonant: true, is_vowel: false }),
            ("sm".to_string(), BengaliChar { bengali: "স্ম".to_string(), is_consonant: true, is_vowel: false }),
            ("sl".to_string(), BengaliChar { bengali: "স্ল".to_string(), is_consonant: true, is_vowel: false }),
        ]);

        // Numbers and others
        patterns.extend([
            ("0".to_string(), BengaliChar { bengali: "০".to_string(), is_consonant: false, is_vowel: false }),
            ("1".to_string(), BengaliChar { bengali: "১".to_string(), is_consonant: false, is_vowel: false }),
            ("2".to_string(), BengaliChar { bengali: "২".to_string(), is_consonant: false, is_vowel: false }),
            ("3".to_string(), BengaliChar { bengali: "৩".to_string(), is_consonant: false, is_vowel: false }),
            ("4".to_string(), BengaliChar { bengali: "৪".to_string(), is_consonant: false, is_vowel: false }),
            ("5".to_string(), BengaliChar { bengali: "৫".to_string(), is_consonant: false, is_vowel: false }),
            ("6".to_string(), BengaliChar { bengali: "৬".to_string(), is_consonant: false, is_vowel: false }),
            ("7".to_string(), BengaliChar { bengali: "৭".to_string(), is_consonant: false, is_vowel: false }),
            ("8".to_string(), BengaliChar { bengali: "৮".to_string(), is_consonant: false, is_vowel: false }),
            ("9".to_string(), BengaliChar { bengali: "৯".to_string(), is_consonant: false, is_vowel: false }),
            (".".to_string(), BengaliChar { bengali: "।".to_string(), is_consonant: false, is_vowel: false }),
            ("$".to_string(), BengaliChar { bengali: "৳".to_string(), is_consonant: false, is_vowel: false }),
            ("aya".to_string(), BengaliChar { bengali: "অ্যা".to_string(), is_consonant: false, is_vowel: false }),
        ]);

        Self { patterns, vowel_diacritics }
    }
}