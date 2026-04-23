mod lang;
mod output;
mod wiki;

use std::env;
use std::io::IsTerminal;
use std::time::{Duration, Instant};

use output::{print_article, print_footer, print_help, print_json_article, COLOR, PLAIN};
use wiki::{
    check_disambiguation_page, do_search, fetch_json, get_first_page, handle_disambiguation,
    urlencoding,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        print_help();
        std::process::exit(1);
    }

    let mut forced_lang: Option<&str> = None;
    let mut search_mode = false;
    let mut json_mode = false;
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
            "-g" | "--get" => {
                i += 1;
            }
            "-s" | "--search" => {
                search_mode = true;
                i += 1;
            }
            "-j" | "--json" => {
                json_mode = true;
                i += 1;
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

    let (detected_lang, detected_variant) = lang::detect_language(&query);
    let (lang, variant): (&str, Option<&str>) = if let Some(l) = forced_lang {
        let variant = match l {
            "zh-cn" | "zh-hans" => Some("zh-cn"),
            "zh-tw" | "zh-hant" => Some("zh-tw"),
            _ => None,
        };
        if l.starts_with("zh") {
            ("zh", variant)
        } else {
            (l, variant)
        }
    } else {
        (detected_lang, detected_variant)
    };

    if !json_mode {
        let mode = if forced_lang.is_some() {
            "manual"
        } else {
            "auto"
        };
        match variant {
            Some(v) => eprintln!("[wiki] language: {lang}, variant: {v} ({mode})"),
            None => eprintln!("[wiki] language: {lang} ({mode})"),
        };
    }

    let t = if json_mode {
        &PLAIN
    } else if std::io::stdout().is_terminal() {
        &COLOR
    } else {
        &PLAIN
    };

    let client = reqwest::Client::builder()
        .user_agent(concat!("wiki/", env!("CARGO_PKG_VERSION")))
        .timeout(Duration::from_secs(10))
        .connect_timeout(Duration::from_secs(5))
        .build()
        .expect("Failed to build HTTP client");

    let variant_param = match variant {
        Some("zh-cn") => "&variant=zh-cn",
        Some("zh-tw") => "&variant=zh-tw",
        _ => "",
    };

    let start = Instant::now();

    if search_mode {
        do_search(&client, lang, variant_param, &query, start, t, json_mode).await;
        return;
    }

    let url = format!(
        "https://{lang}.wikipedia.org/w/api.php?action=query&generator=search&gsrsearch={}&gsrlimit=1&prop=extracts|pageprops&exintro&explaintext&format=json&redirects=1{variant_param}",
        urlencoding(&query)
    );

    let Some(json) = fetch_json(&client, &url).await else {
        if json_mode {
            println!("{{\"error\":\"No results found\"}}");
        } else {
            eprintln!("No results found for '{query}'.");
        }
        std::process::exit(1);
    };

    let Some(page) = get_first_page(&json) else {
        if json_mode {
            println!("{{\"error\":\"No results found\"}}");
        } else {
            eprintln!("No results found for '{query}'.");
        }
        std::process::exit(1);
    };

    let title = page
        .get("title")
        .and_then(|t| t.as_str())
        .unwrap_or("Unknown");
    let is_disambiguation = page
        .get("pageprops")
        .and_then(|pp| pp.get("disambiguation"))
        .is_some();

    if is_disambiguation {
        handle_disambiguation(&client, lang, variant_param, title, start, t, json_mode).await;
    } else {
        let extract = page.get("extract").and_then(|e| e.as_str()).unwrap_or("");

        if extract.is_empty() {
            if json_mode {
                println!("{{\"error\":\"No article found\"}}");
            } else {
                println!("No article found for '{query}'.");
            }
        } else if json_mode {
            print_json_article(lang, title, extract, start);
        } else {
            print_article(t, title, extract);
            print_footer(t, start, lang, title);
        }

        if !json_mode {
            check_disambiguation_page(&client, lang, variant_param, &query, t).await;
        }
    }
}
