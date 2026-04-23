use std::time::Instant;

use crate::output::*;

const TIMEOUT_SECS: u64 = 10;

pub async fn do_search(
    client: &reqwest::Client,
    lang: &str,
    variant_param: &str,
    query: &str,
    start: Instant,
    t: &Theme,
    json_mode: bool,
) {
    let url = format!(
        "https://{lang}.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&srlimit=10&format=json{variant_param}",
        urlencoding(query)
    );

    let Some(json) = fetch_json(client, &url).await else {
        if json_mode {
            println!("[]");
        } else {
            eprintln!("No results found for '{query}'.");
        }
        return;
    };

    let results = json
        .get("query")
        .and_then(|q| q.get("search"))
        .and_then(|s| s.as_array());

    let Some(results) = results else {
        if json_mode {
            println!("[]");
        } else {
            eprintln!("No results found for '{query}'.");
        }
        return;
    };

    if json_mode {
        let items: Vec<serde_json::Value> = results
            .iter()
            .map(|r| {
                let title = r.get("title").and_then(|t| t.as_str()).unwrap_or("");
                let snippet = r.get("snippet").and_then(|s| s.as_str()).unwrap_or("");
                let snippet = strip_html_tags(snippet);
                let url = format!(
                    "https://{lang}.wikipedia.org/wiki/{}",
                    title.replace(' ', "_")
                );
                serde_json::json!({ "title": title, "snippet": snippet, "url": url })
            })
            .collect();
        let obj = serde_json::json!({
            "query": query,
            "language": lang,
            "time_ms": start.elapsed().as_millis() as u64,
            "results": items,
        });
        print_json(&obj);
    } else {
        let bar = "─".repeat(display_width(query) + 12);
        println!("{}┌─ search: {} {}{}", t.title, query, bar, t.reset);
        println!();
        for (i, r) in results.iter().enumerate() {
            let title = r.get("title").and_then(|t| t.as_str()).unwrap_or("");
            let snippet = r.get("snippet").and_then(|s| s.as_str()).unwrap_or("");
            let snippet = strip_html_tags(snippet);
            println!(
                "  {}▸{} {}[{}]{} {}",
                t.bullet,
                t.reset,
                t.title,
                i + 1,
                t.reset,
                title
            );
            if !snippet.is_empty() {
                println!("    {}{}{}", t.dim, snippet, t.reset);
            }
        }
        let elapsed = start.elapsed();
        let time = if elapsed.as_secs() >= 1 {
            format!("{:.2}s", elapsed.as_secs_f64())
        } else {
            format!("{}ms", elapsed.as_millis())
        };
        println!(
            "\n{}└─ {} results · {time}{}",
            t.dim,
            results.len(),
            t.reset
        );
    }
}

pub async fn check_disambiguation_page(
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

    let Some(json) = fetch_json(client, &url).await else {
        return;
    };
    let Some(page) = get_first_page(&json) else {
        return;
    };

    if page
        .get("pageprops")
        .and_then(|pp| pp.get("disambiguation"))
        .is_none()
    {
        return;
    }

    let extract = page.get("extract").and_then(|e| e.as_str()).unwrap_or("");
    let suggestions = filter_disambiguation_lines(extract);

    if !suggestions.is_empty() {
        let (also, _) = disambig_labels(lang);
        print_disambig(t, also, query, &suggestions);
    }
}

pub async fn handle_disambiguation(
    client: &reqwest::Client,
    lang: &str,
    variant_param: &str,
    title: &str,
    start: Instant,
    t: &Theme,
    json_mode: bool,
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

    let first_link = suggestions.iter().find_map(|line| {
        let name = line.split(',').next().unwrap_or(line).trim();
        if name.len() >= 2 {
            Some(name)
        } else {
            None
        }
    });

    if json_mode {
        let mut first_extract = String::new();
        let mut first_title = String::new();

        if let Some(first) = first_link {
            let url = format!(
                "https://{lang}.wikipedia.org/w/api.php?action=query&prop=extracts&exintro&explaintext&titles={}&format=json&redirects=1{variant_param}",
                urlencoding(first)
            );
            if let Some(json) = fetch_json(client, &url).await {
                if let Some(p) = get_first_page(&json) {
                    first_title = p
                        .get("title")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    first_extract = p
                        .get("extract")
                        .and_then(|e| e.as_str())
                        .unwrap_or("")
                        .to_string();
                }
            }
        }

        let obj = serde_json::json!({
            "disambiguation": true,
            "query": title,
            "primary": { "title": first_title, "extract": first_extract },
            "suggestions": suggestions,
            "language": lang,
            "time_ms": start.elapsed().as_millis() as u64,
        });
        print_json(&obj);
    } else {
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
            let (_, ambig) = disambig_labels(lang);
            print_disambig(t, ambig, title, &suggestions);
        }
    }
}

pub async fn fetch_json(client: &reqwest::Client, url: &str) -> Option<serde_json::Value> {
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

pub fn get_first_page(json: &serde_json::Value) -> Option<&serde_json::Value> {
    json.get("query")?
        .get("pages")?
        .as_object()?
        .values()
        .next()
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

fn strip_html_tags(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(c),
            _ => {}
        }
    }
    result
}

const HEX: &[u8; 16] = b"0123456789ABCDEF";

pub fn urlencoding(s: &str) -> String {
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
