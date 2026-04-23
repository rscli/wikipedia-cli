use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: wikipedia-cli <query>");
        std::process::exit(1);
    }

    let query = args.join(" ");
    let lang = detect_language(&query);

    eprintln!("[wikipedia-cli] detected language: {lang}");

    let client = reqwest::Client::builder()
        .user_agent("wikipedia-cli/0.1.0")
        .build()
        .expect("Failed to build HTTP client");

    let search_url = format!(
        "https://{lang}.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&srlimit=1&format=json",
        urlencoding(&query)
    );

    let title = match fetch_json(&client, &search_url).await {
        Some(json) => {
            json.get("query")
                .and_then(|q| q.get("search"))
                .and_then(|s| s.as_array())
                .and_then(|arr| arr.first())
                .and_then(|item| item.get("title"))
                .and_then(|t| t.as_str())
                .map(|s| s.to_string())
        }
        None => None,
    };

    let Some(title) = title else {
        eprintln!("No results found for '{query}'.");
        std::process::exit(1);
    };

    let extract_url = format!(
        "https://{lang}.wikipedia.org/w/api.php?action=query&prop=extracts&exintro&explaintext&titles={}&format=json&redirects=1",
        urlencoding(&title)
    );

    let Some(json) = fetch_json(&client, &extract_url).await else {
        eprintln!("Failed to fetch article.");
        std::process::exit(1);
    };

    let Some(pages) = json.get("query").and_then(|q| q.get("pages")).and_then(|p| p.as_object()) else {
        eprintln!("No article found for '{query}'.");
        std::process::exit(1);
    };

    for page in pages.values() {
        let title = page.get("title").and_then(|t| t.as_str()).unwrap_or("Unknown");
        let extract = page.get("extract").and_then(|e| e.as_str()).unwrap_or("");

        if extract.is_empty() {
            println!("No article found for '{query}'.");
        } else {
            println!("--- {title} ---\n");
            println!("{extract}");
        }
    }
}

fn detect_language(text: &str) -> &'static str {
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
            // Japanese: Hiragana + Katakana
            '\u{3040}'..='\u{309F}' | '\u{30A0}'..='\u{30FF}' | '\u{31F0}'..='\u{31FF}' => {
                japanese_score += 1;
            }
            // Korean: Hangul Syllables + Jamo
            '\u{AC00}'..='\u{D7AF}' | '\u{1100}'..='\u{11FF}' | '\u{3130}'..='\u{318F}' => {
                korean_score += 1;
            }
            // CJK Unified Ideographs (shared by Chinese/Japanese/Korean)
            '\u{4E00}'..='\u{9FFF}' | '\u{3400}'..='\u{4DBF}' | '\u{F900}'..='\u{FAFF}' => {
                cjk_score += 1;
                if is_simplified_indicator(c) {
                    simplified_score += 1;
                } else if is_traditional_indicator(c) {
                    traditional_score += 1;
                }
            }
            // Arabic
            '\u{0600}'..='\u{06FF}' | '\u{0750}'..='\u{077F}' | '\u{FB50}'..='\u{FDFF}' => {
                arabic_score += 1;
            }
            // Cyrillic
            '\u{0400}'..='\u{04FF}' | '\u{0500}'..='\u{052F}' => {
                cyrillic_score += 1;
            }
            // Devanagari (Hindi, Marathi, Sanskrit, Nepali)
            '\u{0900}'..='\u{097F}' => {
                devanagari_score += 1;
            }
            // Thai
            '\u{0E00}'..='\u{0E7F}' => {
                thai_score += 1;
            }
            // Hebrew
            '\u{0590}'..='\u{05FF}' | '\u{FB1D}'..='\u{FB4F}' => {
                hebrew_score += 1;
            }
            // Greek
            '\u{0370}'..='\u{03FF}' | '\u{1F00}'..='\u{1FFF}' => {
                greek_score += 1;
            }
            // Tamil
            '\u{0B80}'..='\u{0BFF}' => {
                tamil_score += 1;
            }
            // Bengali
            '\u{0980}'..='\u{09FF}' => {
                bengali_score += 1;
            }
            // Telugu
            '\u{0C00}'..='\u{0C7F}' => {
                telugu_score += 1;
            }
            // Turkish-specific Latin characters
            'ğ' | 'Ğ' | 'ş' | 'Ş' | 'ı' | 'İ' => {
                turkish_score += 1;
            }
            // Vietnamese-specific diacritics
            'ă' | 'Ă' | 'đ' | 'Đ' | 'ơ' | 'Ơ' | 'ư' | 'Ư' => {
                vietnamese_score += 1;
            }
            _ => {}
        }
    }

    // Japanese-specific kana takes priority
    if japanese_score > 0 {
        return "ja";
    }

    // Korean Hangul takes priority
    if korean_score > 0 {
        return "ko";
    }

    // CJK characters without Japanese/Korean markers → Chinese
    if cjk_score > 0 {
        return if traditional_score > simplified_score {
            "zh-yue" // Cantonese/Traditional → use yue or zh
        } else {
            "zh"
        };
    }

    let scores = [
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

    if let Some((_, lang)) = scores.iter().filter(|(s, _)| *s > 0).max_by_key(|(s, _)| *s) {
        return lang;
    }

    "en"
}

fn is_simplified_indicator(c: char) -> bool {
    matches!(c,
        '么' | '个' | '们' | '这' | '国' | '对' | '说' | '时' | '会' | '学' |
        '将' | '从' | '还' | '进' | '过' | '动' | '与' | '长' | '发' | '开' |
        '问' | '关' | '没' | '车' | '让' | '经' | '头' | '点' | '运' | '实' |
        '东' | '业' | '变' | '节' | '万' | '达' | '岁' | '华' | '写' | '号' |
        '厂' | '币' | '飞' | '机' | '尽' | '脑' | '冲' | '齐' | '网' | '讯'
    )
}

fn is_traditional_indicator(c: char) -> bool {
    matches!(c,
        '們' | '這' | '國' | '對' | '說' | '時' | '會' | '學' | '將' | '從' |
        '還' | '進' | '過' | '動' | '與' | '長' | '發' | '開' | '問' | '關' |
        '沒' | '車' | '讓' | '經' | '頭' | '點' | '運' | '實' | '東' | '業' |
        '變' | '節' | '萬' | '達' | '歲' | '華' | '寫' | '號' | '廠' | '幣' |
        '飛' | '機' | '盡' | '腦' | '衝' | '齊' | '網' | '訊'
    )
}

async fn fetch_json(client: &reqwest::Client, url: &str) -> Option<serde_json::Value> {
    let resp = client.get(url).send().await.ok()?;
    let text = resp.text().await.ok()?;
    serde_json::from_str(&text).ok()
}

fn urlencoding(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '~') {
            result.push(c);
        } else if c == ' ' {
            result.push('+');
        } else {
            for b in c.to_string().bytes() {
                result.push('%');
                result.push_str(&format!("{b:02X}"));
            }
        }
    }
    result
}
