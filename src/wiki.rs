const TIMEOUT_SECS: u64 = 10;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urlencoding_ascii() {
        assert_eq!(urlencoding("hello"), "hello");
    }

    #[test]
    fn urlencoding_spaces() {
        assert_eq!(urlencoding("hello world"), "hello+world");
    }

    #[test]
    fn urlencoding_special_chars() {
        assert_eq!(urlencoding("a&b=c"), "a%26b%3Dc");
    }

    #[test]
    fn urlencoding_unreserved() {
        assert_eq!(urlencoding("a-b_c.d~e"), "a-b_c.d~e");
    }

    #[test]
    fn urlencoding_unicode() {
        let encoded = urlencoding("日本語");
        assert!(encoded.contains('%'));
        assert!(!encoded.contains(' '));
    }

    #[test]
    fn get_first_page_valid() {
        let json: serde_json::Value = serde_json::json!({
            "query": {
                "pages": {
                    "12345": {
                        "title": "Test",
                        "extract": "Content"
                    }
                }
            }
        });
        let page = get_first_page(&json).unwrap();
        assert_eq!(page["title"], "Test");
    }

    #[test]
    fn get_first_page_missing() {
        let json: serde_json::Value = serde_json::json!({"batchcomplete": ""});
        assert!(get_first_page(&json).is_none());
    }
}
