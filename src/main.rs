use std::env;
use std::io::{IsTerminal, Write};
use std::time::{Duration, Instant};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const TIMEOUT_SECS: u64 = 10;

struct Theme {
    title: &'static str,
    dim: &'static str,
    url: &'static str,
    warn: &'static str,
    bullet: &'static str,
    reset: &'static str,
}

const COLOR: Theme = Theme {
    title:  "\x1b[1;32m",
    dim:    "\x1b[2m",
    url:    "\x1b[36m",
    warn:   "\x1b[33m",
    bullet: "\x1b[33m",
    reset:  "\x1b[0m",
};

const PLAIN: Theme = Theme {
    title: "", dim: "", url: "", warn: "", bullet: "", reset: "",
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        print_help();
        std::process::exit(1);
    }

    let mut forced_lang: Option<&str> = None;
    let mut query_parts: Vec<&str> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-l" | "--lang" => {
                if i + 1 < args.len() {
                    forced_lang = Some(&args[i + 1]);
                    i += 2;
                } else {
                    eprintln!("Error: -l requires a language code (e.g. zh, ja, ko, en)");
                    std::process::exit(1);
                }
            }
            "-h" | "--help" => {
                print_help();
                return;
            }
            "-V" | "--version" => {
                println!("wiki {VERSION}");
                return;
            }
            other => {
                query_parts.push(other);
                i += 1;
            }
        }
    }

    let query = query_parts.join(" ");
    if query.is_empty() {
        print_help();
        std::process::exit(1);
    }

    let (detected_lang, detected_variant) = detect_language(&query);
    let (lang, variant): (&str, Option<&str>) = if let Some(l) = forced_lang {
        let variant = match l {
            "zh-cn" | "zh-hans" => Some("zh-cn"),
            "zh-tw" | "zh-hant" => Some("zh-tw"),
            _ => None,
        };
        if l.starts_with("zh") { ("zh", variant) } else { (l, variant) }
    } else {
        (detected_lang, detected_variant)
    };

    eprintln!(
        "[wiki] language: {lang}{}{}",
        variant.map(|v| format!(", variant: {v}")).unwrap_or_default(),
        if forced_lang.is_some() { " (manual)" } else { " (auto)" }
    );

    let t = if std::io::stdout().is_terminal() { &COLOR } else { &PLAIN };

    let client = reqwest::Client::builder()
        .user_agent(concat!("wiki/", env!("CARGO_PKG_VERSION")))
        .timeout(Duration::from_secs(TIMEOUT_SECS))
        .connect_timeout(Duration::from_secs(5))
        .build()
        .expect("Failed to build HTTP client");

    let variant_param = match variant {
        Some("zh-cn") => "&variant=zh-cn",
        Some("zh-tw") => "&variant=zh-tw",
        _ => "",
    };

    let start = Instant::now();

    let search_url = format!(
        "https://{lang}.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&srlimit=1&format=json{variant_param}",
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
        "https://{lang}.wikipedia.org/w/api.php?action=query&prop=extracts|pageprops&exintro&explaintext&titles={}&format=json&redirects=1{variant_param}",
        urlencoding(&title)
    );

    let Some(json) = fetch_json(&client, &extract_url).await else {
        eprintln!("Failed to fetch article.");
        std::process::exit(1);
    };

    let Some(page) = get_first_page(&json) else {
        eprintln!("No article found for '{query}'.");
        std::process::exit(1);
    };

    let is_disambiguation = page
        .get("pageprops")
        .and_then(|pp| pp.get("disambiguation"))
        .is_some();

    if is_disambiguation {
        handle_disambiguation(&client, lang, variant_param, &title, start, t).await;
    } else {
        let page_title = page.get("title").and_then(|t| t.as_str()).unwrap_or("Unknown");
        let extract = page.get("extract").and_then(|e| e.as_str()).unwrap_or("");

        if extract.is_empty() {
            println!("No article found for '{query}'.");
        } else {
            print_article(t, page_title, extract);
            print_footer(t, start, lang, page_title);
        }

        check_disambiguation_page(&client, lang, variant_param, &query, t).await;
    }
}

fn print_article(t: &Theme, title: &str, extract: &str) {
    let bar = "─".repeat(title.len() + 2);
    println!("{}┌─ {} {}{}",  t.title, title, bar, t.reset);
    println!();
    println!("{extract}");
}

fn print_footer(t: &Theme, start: Instant, lang: &str, title: &str) {
    let elapsed = start.elapsed();
    let time = if elapsed.as_secs() >= 1 {
        format!("{:.2}s", elapsed.as_secs_f64())
    } else {
        format!("{}ms", elapsed.as_millis())
    };
    let out = std::io::stdout();
    let mut out = out.lock();
    let _ = write!(out, "\n{}└─ {time} {}{} · {}https://{lang}.wikipedia.org/wiki/", t.dim, t.reset, t.dim, t.url);
    for b in title.bytes() {
        let _ = out.write_all(if b == b' ' { b"_" } else { std::slice::from_ref(&b) });
    }
    let _ = writeln!(out, "{}", t.reset);
}

fn print_disambig(t: &Theme, header: &str, query: &str, suggestions: &[&str]) {
    let out = std::io::stdout();
    let mut out = out.lock();
    let _ = writeln!(out, "\n{}════════════════════════════════════════{}", t.dim, t.reset);
    let _ = writeln!(out, "{}\"{}\" {}{}", t.warn, query, header, t.reset);
    let _ = writeln!(out);
    for s in suggestions {
        let _ = writeln!(out, "  {}▸{} {s}", t.bullet, t.reset);
    }
}

fn get_first_page(json: &serde_json::Value) -> Option<&serde_json::Value> {
    json.get("query")?.get("pages")?.as_object()?.values().next()
}

fn filter_disambiguation_lines(text: &str) -> Vec<&str> {
    text.lines()
        .map(|l| l.trim())
        .filter(|l| {
            !l.is_empty()
                && !l.starts_with("==")
                && !l.contains("may also refer to")
                && !l.contains("most commonly refers to")
                && !l.contains("may refer to")
                && !l.starts_with("All pages with")
        })
        .collect()
}

async fn check_disambiguation_page(
    client: &reqwest::Client,
    lang: &str,
    variant_param: &str,
    query: &str,
    t: &Theme,
) {
    let disambig_title = format!("{query} (disambiguation)");
    let url = format!(
        "https://{lang}.wikipedia.org/w/api.php?action=query&prop=extracts|pageprops&explaintext&titles={}&format=json&redirects=1{variant_param}",
        urlencoding(&disambig_title)
    );

    let Some(json) = fetch_json(client, &url).await else { return };
    let Some(page) = get_first_page(&json) else { return };

    if page.get("pageprops").and_then(|pp| pp.get("disambiguation")).is_none() { return }

    let extract = page.get("extract").and_then(|e| e.as_str()).unwrap_or("");
    let suggestions = filter_disambiguation_lines(extract);

    if !suggestions.is_empty() {
        print_disambig(t, "may also refer to:", query, &suggestions);
    }
}

async fn handle_disambiguation(
    client: &reqwest::Client,
    lang: &str,
    variant_param: &str,
    title: &str,
    start: Instant,
    t: &Theme,
) {
    let full_url = format!(
        "https://{lang}.wikipedia.org/w/api.php?action=query&prop=extracts&explaintext&titles={}&format=json&redirects=1{variant_param}",
        urlencoding(title)
    );

    let extract = match fetch_json(client, &full_url).await {
        Some(json) => get_first_page(&json)
            .and_then(|p| p.get("extract"))
            .and_then(|e| e.as_str())
            .unwrap_or("")
            .to_string(),
        None => String::new(),
    };

    let suggestions = filter_disambiguation_lines(&extract);

    let first_link = suggestions.iter()
        .find_map(|line| {
            let name = line.split(',').next().unwrap_or(line).trim();
            if name.len() >= 2 { Some(name) } else { None }
        });

    if let Some(first) = first_link {
        let url = format!(
            "https://{lang}.wikipedia.org/w/api.php?action=query&prop=extracts&exintro&explaintext&titles={}&format=json&redirects=1{variant_param}",
            urlencoding(first)
        );

        if let Some(json) = fetch_json(client, &url).await {
            if let Some(p) = get_first_page(&json) {
                let pt = p.get("title").and_then(|v| v.as_str()).unwrap_or("Unknown");
                let ext = p.get("extract").and_then(|e| e.as_str()).unwrap_or("");
                if !ext.is_empty() {
                    print_article(t, pt, ext);
                    print_footer(t, start, lang, pt);
                }
            }
        }
    }

    if !suggestions.is_empty() {
        print_disambig(t, "is ambiguous. Did you mean:", title, &suggestions);
    }
}

fn print_help() {
    print!("\
wiki {VERSION}
Query Wikipedia from the command line with automatic language detection.

USAGE:
    wiki <query>

OPTIONS:
    -l, --lang <code>  Specify language (e.g. en, zh, zh-cn, zh-tw, ja, ko, ru, ...)
    -h, --help         Print help information
    -V, --version      Print version information

EXAMPLES:
    wiki rust
    wiki 大语言模型
    wiki プログラミング言語
    wiki 인공지능
    wiki -l zh rust              # query 'rust' on Chinese Wikipedia
    wiki -l ja programming       # query 'programming' on Japanese Wikipedia
    wiki -l zh-tw machine learning  # query in Traditional Chinese

SUPPORTED LANGUAGES:
    Auto-detected by script: English, Chinese (Simplified/Traditional),
    Japanese, Korean, Arabic, Russian, Hindi, Thai, Hebrew, Greek,
    Tamil, Bengali, Telugu, Turkish, Vietnamese
");
}

fn detect_language(text: &str) -> (&'static str, Option<&'static str>) {
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

    if japanese_score > 0 { return ("ja", None); }
    if korean_score > 0 { return ("ko", None); }

    if cjk_score > 0 {
        return if traditional_score > simplified_score {
            ("zh", Some("zh-tw"))
        } else {
            ("zh", Some("zh-cn"))
        };
    }

    let scores: [_; 11] = [
        (arabic_score, "ar"), (cyrillic_score, "ru"), (devanagari_score, "hi"),
        (thai_score, "th"), (hebrew_score, "he"), (greek_score, "el"),
        (tamil_score, "ta"), (bengali_score, "bn"), (telugu_score, "te"),
        (turkish_score, "tr"), (vietnamese_score, "vi"),
    ];

    if let Some((_, lang)) = scores.iter().filter(|(s, _)| *s > 0).max_by_key(|(s, _)| *s) {
        return (lang, None);
    }

    ("en", None)
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
    let resp = match client.get(url).send().await {
        Ok(r) => r,
        Err(e) if e.is_timeout() => {
            eprintln!("[wiki] Error: request timed out after {TIMEOUT_SECS}s. Please check your network and try again.");
            std::process::exit(1);
        }
        Err(e) if e.is_connect() => {
            eprintln!("[wiki] Error: connection failed. Please check your network.");
            std::process::exit(1);
        }
        Err(_) => return None,
    };
    let text = resp.text().await.ok()?;
    serde_json::from_str(&text).ok()
}

const HEX: &[u8; 16] = b"0123456789ABCDEF";

fn urlencoding(s: &str) -> String {
    let mut result = String::with_capacity(s.len() * 2);
    for &b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(b as char);
            }
            b' ' => result.push('+'),
            _ => {
                result.push('%');
                result.push(HEX[(b >> 4) as usize] as char);
                result.push(HEX[(b & 0x0F) as usize] as char);
            }
        }
    }
    result
}
