# wikipedia-cli

A command-line tool to query Wikipedia.

```
$ wiki anthropic

[wiki] language: en (auto)
┌─ Anthropic ───────────
Anthropic PBC is an American artificial intelligence (AI) company headquartered in San Francisco. It has developed a range of large language models (LLMs) named Claude and focuses on AI safety.
Anthropic was founded in 2021 by former members of OpenAI, including siblings Daniela Amodei and Dario Amodei, who are president and CEO, respectively. The company is privately held and, as of February 2026, had an estimated value of $380 billion. 
└─ 179ms  · https://en.wikipedia.org/wiki/Anthropic
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

options

```
-g, --get          Get article summary (default)
-s, --search       Search mode: list top results instead of fetching article
-j, --json         Output as JSON (for piping to jq, scripts, etc.)
-l, --lang <code>  Specify language (e.g. en, zh, zh-cn, zh-tw, ja, ko, ru, ...)
-h, --help         Print help information
-V, --version      Print version information
```


## Features

- ANSI colored output with hacker-style box drawing (auto-disabled when piped)
- Single API call for article fetch (search + extract merged via generator API)
- Zero-dependency language detection via Unicode script analysis
- Optimized release binary (LTO, strip, panic=abort, opt-level=z)

## License

MIT
