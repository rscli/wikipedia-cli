![wikipedia-cli-image](https://github.com/user-attachments/assets/7f97fcba-6601-4467-b431-af0b5b6ad762)

# Wikipedia CLI

A command-line tool to query wikipedia

## Installation

### via homebrew

```bash
brew install rscli/tap/wiki
```

### via cargo

```bash
cargo install --git https://github.com/rscli/wikipedia-cli
```

## Usage

```
wiki [OPTIONS] <query>
```

### options

```
-j, --json         Output as JSON (for piping to jq, scripts, etc.)
-l, --lang <code>  Specify language (e.g. en, zh, zh-cn, zh-tw, ja, ko, ru, ...)
-h, --help         Print help information
-V, --version      Print version information
```

### examples

```bash
wiki rust                    # query article
wiki "rust lang"             # multi-word query
wiki --json rust             # article as JSON
wiki -l zh rust              # query on Chinese Wikipedia
wiki -l ja programming       # query on Japanese Wikipedia
```

### language auto-detection

```bash
wiki 人工智能                 # → Chinese Wikipedia
wiki プログラミング言語         # → Japanese Wikipedia
wiki 인공지능                   # → Korean Wikipedia
wiki Искусственный интеллект  # → Russian Wikipedia
```

## License

MIT
