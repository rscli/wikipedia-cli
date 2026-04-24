use std::io::Write;
use std::time::Instant;

pub struct Theme {
    pub title: &'static str,
    pub dim: &'static str,
    pub url: &'static str,
    pub reset: &'static str,
}

pub const COLOR: Theme = Theme {
    title: "\x1b[1;32m",
    dim: "\x1b[2m",
    url: "\x1b[36m",
    reset: "\x1b[0m",
};

pub const PLAIN: Theme = Theme {
    title: "",
    dim: "",
    url: "",
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
    -j, --json         Output as JSON (for piping to jq, scripts, etc.)
    -l, --lang <code>  Specify language (e.g. en, zh, zh-cn, zh-tw, ja, ko, ru, ...)
    -h, --help         Print help information
    -V, --version      Print version information

EXAMPLES:
    wiki rust                    # query article
    wiki \"rust lang\"             # multi-word query
    wiki --json rust             # article as JSON
    wiki -l zh rust              # query on Chinese Wikipedia
    wiki -l ja programming       # query on Japanese Wikipedia

SUPPORTED LANGUAGES:
    Auto-detected by script: English, Chinese (Simplified/Traditional),
    Japanese, Korean, Arabic, Russian, Hindi, Thai, Hebrew, Greek,
    Tamil, Bengali, Telugu, Turkish, Vietnamese
"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_width_ascii() {
        assert_eq!(display_width("hello"), 5);
    }

    #[test]
    fn display_width_cjk() {
        assert_eq!(display_width("日本"), 4);
    }

    #[test]
    fn display_width_mixed() {
        assert_eq!(display_width("hello世界"), 9);
    }

    #[test]
    fn display_width_empty() {
        assert_eq!(display_width(""), 0);
    }
}
