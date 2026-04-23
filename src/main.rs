use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: wikipedia-cli <query>");
        std::process::exit(1);
    }

    let query = args.join(" ");
    let url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&prop=extracts&exintro&explaintext&titles={}&format=json&redirects=1",
        urlencoding(&query)
    );

    let client = reqwest::Client::builder()
        .user_agent("wikipedia-cli/0.1.0")
        .build()
        .expect("Failed to build HTTP client");

    let resp = match client.get(&url).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Request failed: {e}");
            std::process::exit(1);
        }
    };

    let text = match resp.text().await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to read response: {e}");
            std::process::exit(1);
        }
    };

    let json: serde_json::Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to parse response: {e}");
            std::process::exit(1);
        }
    };

    let Some(pages) = json.get("query").and_then(|q| q.get("pages")).and_then(|p| p.as_object()) else {
        eprintln!("No results found for '{query}'.");
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

fn urlencoding(s: &str) -> String {
    let mut result = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(b as char);
            }
            b' ' => result.push('+'),
            _ => {
                result.push('%');
                result.push_str(&format!("{b:02X}"));
            }
        }
    }
    result
}
