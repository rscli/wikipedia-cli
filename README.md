# wk

A command-line tool to query Wikipedia with automatic language detection.

```
$ wk rust
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
# Binary at ./target/release/wk
```

## Usage

```
wk <query>
```

### Options

```
-l, --lang <code>  Specify language (e.g. en, zh, zh-cn, zh-tw, ja, ko, ru, ...)
-h, --help         Print help information
-V, --version      Print version information
```

### Examples

```bash
# English
wk rust
wk "Rust (programming language)"
wk "C++"

# Simplified Chinese
wk 大语言模型

# Traditional Chinese
wk 機器學習

# Japanese
wk プログラミング言語

# Korean
wk 인공지능

# Arabic
wk الذكاء_الاصطناعي

# Russian
wk "Искусственный интеллект"

# Emoji & Symbols
wk 🦀
wk 🐍
wk ∞
wk "E=mc²"

# Force language with -l
wk -l zh rust              # query 'rust' on Chinese Wikipedia
wk -l ja programming       # query on Japanese Wikipedia
wk -l zh-tw machine learning  # query in Traditional Chinese
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
$ wk mercury
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
