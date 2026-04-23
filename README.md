# wikipedia-cli

A command-line tool to query Wikipedia with automatic language detection and hacker-style terminal output.

```
$ wiki rust
┌─ Rust ────────

Rust is an iron oxide, a usually reddish-brown oxide formed by the reaction of iron and oxygen...

└─ 342ms · https://en.wikipedia.org/wiki/Rust

════════════════════════════════════════
"rust" may also refer to:

  ▸ Rust (programming language), a general purpose programming language
  ▸ Rust (video game), a video game developed by Facepunch Studios
  ▸ ...
```

## Install

```bash
cargo install --git https://github.com/rustq/wikipedia-cli
```

Or build from source:

```bash
git clone https://github.com/rustq/wikipedia-cli.git
cd wikipedia-cli
cargo build --release
# Binary at ./target/release/wiki
```

## Usage

```
wiki [OPTIONS] <query>
```

### Options

```
-g, --get          Get article summary (default)
-s, --search       Search mode: list top results instead of fetching article
-j, --json         Output as JSON (for piping to jq, scripts, etc.)
-l, --lang <code>  Specify language (e.g. en, zh, zh-cn, zh-tw, ja, ko, ru, ...)
-h, --help         Print help information
-V, --version      Print version information
```

### Examples

```bash
# Get article (default)
wiki rust
wiki --get rust              # same as above, explicit
wiki "Rust (programming language)"
wiki "C++"

# Search mode
wiki --search rust           # list top search results
wiki -s "programming language"

# JSON output
wiki --json rust             # article as JSON
wiki --search --json rust    # search results as JSON

# Multi-language (auto-detected)
wiki 大语言模型              # Simplified Chinese
wiki 機器學習                # Traditional Chinese
wiki プログラミング言語       # Japanese
wiki 인공지능                # Korean
wiki "Искусственный интеллект"  # Russian
wiki الذكاء_الاصطناعي        # Arabic

# Emoji & Symbols
wiki 🦀
wiki 🐍
wiki ∞
wiki "E=mc²"

# Force language with -l
wiki -l zh rust              # query 'rust' on Chinese Wikipedia
wiki -l ja programming       # query on Japanese Wikipedia
wiki -l zh-tw machine learning  # query in Traditional Chinese
```

## Language Detection

Language is auto-detected by analyzing Unicode script ranges in the query:

| Script | Language | Wikipedia |
|--------|----------|-----------|
| Latin | English | en.wikipedia.org |
| CJK (simplified) | Chinese | zh.wikipedia.org (zh-cn) |
| CJK (traditional) | Chinese | zh.wikipedia.org (zh-tw) |
| Hiragana / Katakana | Japanese | ja.wikipedia.org |
| Hangul | Korean | ko.wikipedia.org |
| Arabic | Arabic | ar.wikipedia.org |
| Cyrillic | Russian | ru.wikipedia.org |
| Devanagari | Hindi | hi.wikipedia.org |
| Thai | Thai | th.wikipedia.org |
| Hebrew | Hebrew | he.wikipedia.org |
| Greek | Greek | el.wikipedia.org |
| Tamil | Tamil | ta.wikipedia.org |
| Bengali | Bengali | bn.wikipedia.org |
| Telugu | Telugu | te.wikipedia.org |
| Turkish markers | Turkish | tr.wikipedia.org |
| Vietnamese markers | Vietnamese | vi.wikipedia.org |

## Search Mode

List top search results without fetching full articles:

```
$ wiki --search rust
┌─ search: rust ──────────────────────

  ▸ [1] Rust
    Rust is an iron oxide, a usually reddish-brown oxide...
  ▸ [2] Rust (programming language)
    Rust is a general-purpose programming language...
  ▸ [3] Rust (video game)
    Rust is a multiplayer survival video game...

└─ 10 results · 245ms
```

## JSON Output

Machine-readable output for piping to `jq`, scripts, or other tools:

```
$ wiki --json rust
{
  "title": "Rust",
  "extract": "Rust is an iron oxide...",
  "url": "https://en.wikipedia.org/wiki/Rust",
  "language": "en",
  "time_ms": 342
}

$ wiki --search --json rust
{
  "query": "rust",
  "language": "en",
  "time_ms": 198,
  "results": [
    { "title": "Rust", "snippet": "...", "url": "..." },
    ...
  ]
}
```

## Disambiguation

When a query is ambiguous, the tool shows the primary article followed by a list of alternative meanings. Disambiguation messages are localized to the query language.

```
$ wiki mercury
┌─ Mercury (planet) ──────────────

Mercury is the first planet from the Sun and the smallest in the Solar System...

└─ 523ms · https://en.wikipedia.org/wiki/Mercury_(planet)

════════════════════════════════════════
"Mercury" is ambiguous. Did you mean:

  ▸ Mercury (planet), the closest planet to the Sun
  ▸ Mercury (element), a chemical element
  ▸ Mercury (mythology), a Roman deity
```

## Features

- ANSI colored output with hacker-style box drawing (auto-disabled when piped)
- Single API call for article fetch (search + extract merged via generator API)
- Zero-dependency language detection via Unicode script analysis
- Optimized release binary (LTO, strip, panic=abort, opt-level=z)

## License

MIT
