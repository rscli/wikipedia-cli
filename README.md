# wikipedia-cli

A command-line tool to query Wikipedia with automatic language detection.

```
$ wiki rust
--- Rust ---

Rust is an iron oxide, a usually reddish-brown oxide formed by the reaction of iron and oxygen...

========================================
"rust" may also refer to:

  - Rust (programming language), a general purpose programming language focused on performance and memory safety
  - Rust (video game), a video game developed by Facepunch Studios
  - ...
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
wiki <query>
```

### Options

```
-l, --lang <code>  Specify language (e.g. en, zh, zh-cn, zh-tw, ja, ko, ru, ...)
-s, --search       Search mode: list top results instead of fetching article
-j, --json         Output as JSON (for piping to jq, scripts, etc.)
-h, --help         Print help information
-V, --version      Print version information
```

### Examples

```bash
# English
wiki rust
wiki "Rust (programming language)"
wiki "C++"

# Simplified Chinese
wiki 大语言模型

# Traditional Chinese
wiki 機器學習

# Japanese
wiki プログラミング言語

# Korean
wiki 인공지능

# Arabic
wiki الذكاء_الاصطناعي

# Russian
wiki "Искусственный интеллект"

# Emoji & Symbols
wiki 🦀
wiki 🐍
wiki ∞
wiki "E=mc²"

# Force language with -l
wiki -l zh rust              # query 'rust' on Chinese Wikipedia
wiki -l ja programming       # query on Japanese Wikipedia
wiki -l zh-tw machine learning  # query in Traditional Chinese

# Search mode
wiki --search rust           # list top search results
wiki -s programming          # short flag

# JSON output
wiki --json rust             # article as JSON
wiki --search --json rust    # search results as JSON
wiki -sj rust                # combine short flags
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

## Disambiguation

When a query is ambiguous, the tool shows the primary article followed by a list of alternative meanings:

```
$ wiki mercury
--- Mercury (planet) ---

Mercury is the first planet from the Sun and the smallest in the Solar System...

========================================
"Mercury" is ambiguous. Did you mean:

  - Mercury (planet), the closest planet to the Sun
  - Mercury (element), a chemical element
  - Mercury (mythology), a Roman deity
```

## License

MIT
