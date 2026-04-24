pub fn detect_language(text: &str) -> (&'static str, Option<&'static str>) {
    let mut japanese_score = 0u32;
    let mut korean_score = 0u32;
    let mut cjk_score = 0u32;
    let mut simplified_score = 0u32;
    let mut traditional_score = 0u32;
    let mut arabic_score = 0u32;
    let mut cyrillic_score = 0u32;
    let mut devanagari_score = 0u32;
    let mut thai_score = 0u32;
    let mut hebrew_score = 0u32;
    let mut greek_score = 0u32;
    let mut tamil_score = 0u32;
    let mut bengali_score = 0u32;
    let mut telugu_score = 0u32;
    let mut turkish_score = 0u32;
    let mut vietnamese_score = 0u32;

    for c in text.chars() {
        match c {
            '\u{3040}'..='\u{309F}' | '\u{30A0}'..='\u{30FF}' | '\u{31F0}'..='\u{31FF}' => {
                japanese_score += 1;
            }
            '\u{AC00}'..='\u{D7AF}' | '\u{1100}'..='\u{11FF}' | '\u{3130}'..='\u{318F}' => {
                korean_score += 1;
            }
            '\u{4E00}'..='\u{9FFF}' | '\u{3400}'..='\u{4DBF}' | '\u{F900}'..='\u{FAFF}' => {
                cjk_score += 1;
                if is_simplified_indicator(c) {
                    simplified_score += 1;
                } else if is_traditional_indicator(c) {
                    traditional_score += 1;
                }
            }
            '\u{0600}'..='\u{06FF}' | '\u{0750}'..='\u{077F}' | '\u{FB50}'..='\u{FDFF}' => {
                arabic_score += 1;
            }
            '\u{0400}'..='\u{04FF}' | '\u{0500}'..='\u{052F}' => {
                cyrillic_score += 1;
            }
            '\u{0900}'..='\u{097F}' => devanagari_score += 1,
            '\u{0E00}'..='\u{0E7F}' => thai_score += 1,
            '\u{0590}'..='\u{05FF}' | '\u{FB1D}'..='\u{FB4F}' => hebrew_score += 1,
            '\u{0370}'..='\u{03FF}' | '\u{1F00}'..='\u{1FFF}' => greek_score += 1,
            '\u{0B80}'..='\u{0BFF}' => tamil_score += 1,
            '\u{0980}'..='\u{09FF}' => bengali_score += 1,
            '\u{0C00}'..='\u{0C7F}' => telugu_score += 1,
            'ğ' | 'Ğ' | 'ş' | 'Ş' | 'ı' | 'İ' => turkish_score += 1,
            'ă' | 'Ă' | 'đ' | 'Đ' | 'ơ' | 'Ơ' | 'ư' | 'Ư' => vietnamese_score += 1,
            _ => {}
        }
    }

    if japanese_score > 0 {
        return ("ja", None);
    }
    if korean_score > 0 {
        return ("ko", None);
    }

    if cjk_score > 0 {
        return if traditional_score > simplified_score {
            ("zh", Some("zh-tw"))
        } else {
            ("zh", Some("zh-cn"))
        };
    }

    let scores: [_; 11] = [
        (arabic_score, "ar"),
        (cyrillic_score, "ru"),
        (devanagari_score, "hi"),
        (thai_score, "th"),
        (hebrew_score, "he"),
        (greek_score, "el"),
        (tamil_score, "ta"),
        (bengali_score, "bn"),
        (telugu_score, "te"),
        (turkish_score, "tr"),
        (vietnamese_score, "vi"),
    ];

    if let Some((_, lang)) = scores
        .iter()
        .filter(|(s, _)| *s > 0)
        .max_by_key(|(s, _)| *s)
    {
        return (lang, None);
    }

    ("en", None)
}

fn is_simplified_indicator(c: char) -> bool {
    matches!(
        c,
        '么' | '个'
            | '们'
            | '这'
            | '国'
            | '对'
            | '说'
            | '时'
            | '会'
            | '学'
            | '将'
            | '从'
            | '还'
            | '进'
            | '过'
            | '动'
            | '与'
            | '长'
            | '发'
            | '开'
            | '问'
            | '关'
            | '没'
            | '车'
            | '让'
            | '经'
            | '头'
            | '点'
            | '运'
            | '实'
            | '东'
            | '业'
            | '变'
            | '节'
            | '万'
            | '达'
            | '岁'
            | '华'
            | '写'
            | '号'
            | '厂'
            | '币'
            | '飞'
            | '机'
            | '尽'
            | '脑'
            | '冲'
            | '齐'
            | '网'
            | '讯'
    )
}

fn is_traditional_indicator(c: char) -> bool {
    matches!(
        c,
        '們' | '這'
            | '國'
            | '對'
            | '說'
            | '時'
            | '會'
            | '學'
            | '將'
            | '從'
            | '還'
            | '進'
            | '過'
            | '動'
            | '與'
            | '長'
            | '發'
            | '開'
            | '問'
            | '關'
            | '沒'
            | '車'
            | '讓'
            | '經'
            | '頭'
            | '點'
            | '運'
            | '實'
            | '東'
            | '業'
            | '變'
            | '節'
            | '萬'
            | '達'
            | '歲'
            | '華'
            | '寫'
            | '號'
            | '廠'
            | '幣'
            | '飛'
            | '機'
            | '盡'
            | '腦'
            | '衝'
            | '齊'
            | '網'
            | '訊'
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_text() {
        assert_eq!(detect_language("rust"), ("en", None));
        assert_eq!(detect_language("hello world"), ("en", None));
    }

    #[test]
    fn japanese_hiragana() {
        assert_eq!(detect_language("こんにちは"), ("ja", None));
    }

    #[test]
    fn japanese_katakana() {
        assert_eq!(detect_language("プログラミング"), ("ja", None));
    }

    #[test]
    fn korean() {
        assert_eq!(detect_language("한국어"), ("ko", None));
    }

    #[test]
    fn simplified_chinese() {
        let (lang, variant) = detect_language("这个国家");
        assert_eq!(lang, "zh");
        assert_eq!(variant, Some("zh-cn"));
    }

    #[test]
    fn traditional_chinese() {
        let (lang, variant) = detect_language("這個國家");
        assert_eq!(lang, "zh");
        assert_eq!(variant, Some("zh-tw"));
    }

    #[test]
    fn arabic() {
        assert_eq!(detect_language("مرحبا").0, "ar");
    }

    #[test]
    fn russian() {
        assert_eq!(detect_language("программирование").0, "ru");
    }

    #[test]
    fn hindi() {
        assert_eq!(detect_language("नमस्ते").0, "hi");
    }

    #[test]
    fn thai() {
        assert_eq!(detect_language("สวัสดี").0, "th");
    }

    #[test]
    fn hebrew() {
        assert_eq!(detect_language("שלום").0, "he");
    }

    #[test]
    fn greek() {
        assert_eq!(detect_language("ελληνικά").0, "el");
    }

    #[test]
    fn mixed_english_stays_english() {
        assert_eq!(detect_language("Rust programming language"), ("en", None));
    }

    #[test]
    fn empty_string() {
        assert_eq!(detect_language(""), ("en", None));
    }
}
