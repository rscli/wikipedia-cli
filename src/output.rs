use std::io::Write;
use std::time::Instant;

pub struct Theme {
    pub title: &'static str,
    pub dim: &'static str,
    pub url: &'static str,
    pub warn: &'static str,
    pub bullet: &'static str,
    pub reset: &'static str,
}

pub const COLOR: Theme = Theme {
    title: "\x1b[1;32m",
    dim: "\x1b[2m",
    url: "\x1b[36m",
    warn: "\x1b[33m",
    bullet: "\x1b[33m",
    reset: "\x1b[0m",
};

pub const PLAIN: Theme = Theme {
    title: "",
    dim: "",
    url: "",
    warn: "",
    bullet: "",
    reset: "",
};

pub fn display_width(s: &str) -> usize {
    s.chars()
        .map(|c| match c as u32 {
            0x1100..=0x115F
            | 0x2E80..=0x303E
            | 0x3040..=0x33BF
            | 0x3400..=0x4DBF
            | 0x4E00..=0x9FFF
            | 0xA000..=0xA4CF
            | 0xAC00..=0xD7AF
            | 0xF900..=0xFAFF
            | 0xFE30..=0xFE6F
            | 0xFF01..=0xFF60
            | 0xFFE0..=0xFFE6
            | 0x20000..=0x2FA1F => 2,
            _ => 1,
        })
        .sum()
}

pub fn print_article(t: &Theme, title: &str, extract: &str) {
    let bar = "─".repeat(display_width(title) + 2);
    println!("{}┌─ {} {}{}", t.title, title, bar, t.reset);
    println!();
    println!("{extract}");
}

pub fn print_footer(t: &Theme, start: Instant, lang: &str, title: &str) {
    let elapsed = start.elapsed();
    let time = if elapsed.as_secs() >= 1 {
        format!("{:.2}s", elapsed.as_secs_f64())
    } else {
        format!("{}ms", elapsed.as_millis())
    };
    let out = std::io::stdout();
    let mut out = out.lock();
    let _ = write!(
        out,
        "\n{}└─ {time} {}{} · {}https://{lang}.wikipedia.org/wiki/",
        t.dim, t.reset, t.dim, t.url
    );
    for b in title.bytes() {
        let _ = out.write_all(if b == b' ' {
            b"_"
        } else {
            std::slice::from_ref(&b)
        });
    }
    let _ = writeln!(out, "{}", t.reset);
}

pub fn disambig_labels(lang: &str) -> (&'static str, &'static str) {
    match lang {
        "zh" => ("也可以指：", "是一个消歧义词条。您是否在找："),
        "ja" => (
            "は以下を指す場合もあります：",
            "は曖昧さ回避です。もしかして：",
        ),
        "ko" => (
            "은(는) 다음을 가리킬 수도 있습니다:",
            "은(는) 동음이의어입니다. 찾으시는 것은:",
        ),
        "ar" => ("قد تشير أيضًا إلى:", "صفحة توضيح. هل تقصد:"),
        "ru" => (
            "может также означать:",
            "— страница значений. Возможно, вы имели в виду:",
        ),
        "hi" => ("यह भी हो सकता है:", "एक बहुविकल्पी पृष्ठ है। क्या आप ढूंढ रहे हैं:"),
        "th" => ("อาจหมายถึง:", "เป็นหน้าแก้ความกำกวม คุณหมายถึง:"),
        "he" => (":עשוי גם להתייחס ל", ":דף פירושונים. האם התכוונת ל"),
        "el" => (
            "μπορεί επίσης να αναφέρεται σε:",
            "είναι σελίδα αποσαφήνισης. Εννοούσατε:",
        ),
        "vi" => (
            "cũng có thể là:",
            "là trang định hướng. Có phải bạn muốn tìm:",
        ),
        "tr" => (
            "ayrıca şu anlamlara gelebilir:",
            "bir anlam ayrımı sayfasıdır. Aradığınız:",
        ),
        _ => ("may also refer to:", "is ambiguous. Did you mean:"),
    }
}

pub fn print_disambig(t: &Theme, header: &str, query: &str, suggestions: &[&str]) {
    let out = std::io::stdout();
    let mut out = out.lock();
    let _ = writeln!(
        out,
        "\n{}════════════════════════════════════════{}",
        t.dim, t.reset
    );
    let _ = writeln!(out, "{}\"{}\" {}{}", t.warn, query, header, t.reset);
    let _ = writeln!(out);
    for s in suggestions {
        let _ = writeln!(out, "  {}▸{} {s}", t.bullet, t.reset);
    }
}

pub fn print_json(obj: &serde_json::Value) {
    let out = std::io::stdout();
    let mut out = out.lock();
    let _ = serde_json::to_writer_pretty(&mut out, obj);
    let _ = writeln!(out);
}

pub fn print_json_article(lang: &str, title: &str, extract: &str, start: Instant) {
    let url = format!(
        "https://{lang}.wikipedia.org/wiki/{}",
        title.replace(' ', "_")
    );
    let obj = serde_json::json!({
        "title": title,
        "extract": extract,
        "url": url,
        "language": lang,
        "time_ms": start.elapsed().as_millis() as u64,
    });
    print_json(&obj);
}

pub fn print_help() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    print!(
        "\
wiki {VERSION}
Query Wikipedia from the command line with automatic language detection.

USAGE:
    wiki [OPTIONS] <query>

OPTIONS:
    -g, --get          Get article summary (default)
    -s, --search       Search mode: list top results instead of fetching article
    -j, --json         Output as JSON (for piping to jq, scripts, etc.)
    -l, --lang <code>  Specify language (e.g. en, zh, zh-cn, zh-tw, ja, ko, ru, ...)
    -h, --help         Print help information
    -V, --version      Print version information

EXAMPLES:
    wiki rust                    # open article (default)
    wiki --get rust              # same as above, explicit
    wiki --search rust           # list search results
    wiki --json rust             # article as JSON
    wiki --search --json rust    # search results as JSON
    wiki -l zh rust              # query 'rust' on Chinese Wikipedia
    wiki -l ja programming       # query 'programming' on Japanese Wikipedia

SUPPORTED LANGUAGES:
    Auto-detected by script: English, Chinese (Simplified/Traditional),
    Japanese, Korean, Arabic, Russian, Hindi, Thai, Hebrew, Greek,
    Tamil, Bengali, Telugu, Turkish, Vietnamese
"
    );
}
