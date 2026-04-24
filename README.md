# wikipedia-cli

A command-line tool to query Wikipedia.

```
$ wiki linux
[wiki] language: en (auto)
┌─ Linux ────────
Linux is a family of open-source Unix-like operating systems ...

└─ 179ms  · https://en.wikipedia.org/wiki/Linux
```

```
$ wiki "rust lang"
[wiki] language: en (auto)
┌─ Rust (programming language) ─────────────────────────────
Rust is a general-purpose programming language ...

└─ 191ms  · https://en.wikipedia.org/wiki/Rust_(programming_language)
```

## Installation

via homebrew

```bash
brew install rscli/tap/wiki
```

via cargo

```bash
cargo install --git https://github.com/rscli/wikipedia-cli
```

## Usage

```
wiki [OPTIONS] <query>
```

### Options

```
-j, --json         Output as JSON (for piping to jq, scripts, etc.)
-l, --lang <code>  Specify language (e.g. en, zh, zh-cn, zh-tw, ja, ko, ru, ...)
-h, --help         Print help information
-V, --version      Print version information
```

### Examples

```bash
wiki rust                    # query article
wiki "rust lang"             # multi-word query
wiki --json rust             # article as JSON
wiki -l zh rust              # query on Chinese Wikipedia
wiki -l ja programming       # query on Japanese Wikipedia
```

### Language auto-detection

```bash
wiki 人工智能               # → Chinese Wikipedia
wiki プログラミング言語      # → Japanese Wikipedia
wiki 인공지능               # → Korean Wikipedia
wiki Искусственный интеллект # → Russian Wikipedia
```

## License

MIT
