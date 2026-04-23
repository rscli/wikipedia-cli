use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: wikipedia-cli <query>");
        std::process::exit(1);
    }

    let query = args.join(" ");

    let client = reqwest::Client::builder()
        .user_agent("wikipedia-cli/0.1.0")
        .build()
        .expect("Failed to build HTTP client");

    let has_cjk = query.chars().any(|c| {
        matches!(c,
            '\u{4E00}'..='\u{9FFF}' |
            '\u{3400}'..='\u{4DBF}' |
            '\u{3000}'..='\u{303F}' |
            '\u{30A0}'..='\u{30FF}' |
            '\u{AC00}'..='\u{D7AF}'
        )
    });
    let lang = if has_cjk { "zh" } else { "en" };

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
