# wikipedia-cli

A command-line tool to query Wikipedia.

```
$ wiki "quantum computing"
[wiki] language: en (auto)
┌─ Quantum computing ───────────────────

A quantum computer is a real or theoretical computer that exploits quantum phenomena like superposition and entanglement in an essential way. It is widely believed that a quantum computer could perform some calculations exponentially faster than any classical computer. For example, a large-scale quantum computer could break some widely used encryption schemes and aid physicists in performing physical simulations. However, current hardware implementations of quantum computation are largely experimental and only suitable for specialized tasks.
The basic unit of information in quantum computing, the qubit (or "quantum bit"), serves the same function as the bit in ordinary or "classical" computing. However, unlike a classical bit, which can be in one of two states (a binary), a qubit can exist in a linear combination of two states known as a quantum superposition. The result of measuring a qubit is one of the two states given by a probabilistic rule. If a quantum computer manipulates the qubit in a particular way, wave interference effects amplify the probability of the desired measurement result. The design of quantum algorithms involves creating procedures that allow a quantum computer to perform this amplification.
Quantum computers are not yet practical for real-world applications. Physically engineering high-quality qubits has proven to be challenging. If a physical qubit is not sufficiently isolated from its environment, it suffers from quantum decoherence, introducing noise into calculations. National governments have invested heavily in experimental research aimed at developing scalable qubits with longer coherence times and lower error rates. Example implementations include superconductors (which isolate an electrical current by eliminating electrical resistance) and ion traps (which confine a single atomic particle using electromagnetic fields). Researchers have claimed, and are widely believed to be correct, that certain quantum devices can outperform classical computers on narrowly defined tasks, a milestone referred to as quantum advantage or quantum supremacy. These tasks are not necessarily useful for real-world applications. As a result, current demonstrations are best understood as scientific milestones rather than evidence of broad near-term deployment.



└─ 309ms  · https://en.wikipedia.org/wiki/Quantum_computing
```

```
$ wiki "量子计算机的工作原理是什么"
[wiki] language: zh, variant: zh-cn (auto)
┌─ 不确定性原理 ──────────────

在量子力学里，不确定性原理（uncertainty principle，又译测不准原理）表明，粒子的位置与动量不可同时被确定，位置的不确定性越小，则动量的不确定性越大，反之亦然。对于不同的案例，不确定性的内涵也不一样，它可以是观察者对于某种数量的信息的缺乏程度，也可以是对于某种数量的测量误差大小，或者是一个系综的类似制备的系统所具有的统计学扩散数值。
维尔纳·海森堡于1925年发表论文《论量子理论运动学与力学的物理内涵》(On the quantum-theoretical reinterpretation of kinematical and mechanical relationships)给出这原理的原本启发式论述，希望能够成功地定性分析与表述简单量子实验的物理性质。这原理又称为“海森堡不确定性原理”。同年稍后，厄尔·肯纳德严格地数学表述出位置与动量的不确定性关系式。两年后，霍华德·罗伯森又将肯纳德的关系式加以推广。
类似的不确定性关系式也存在于能量和时间、角动量和角度等物理量之间。由于不确定性原理是量子力学的基要理论，很多一般实验都时常会涉及到关于它的一些问题。有些实验会特别检验这原理或类似的原理。例如，检验发生于超导系统或量子光学系统的“数字－相位不确定性原理”。对于不确定性原理的相关研究可以用来发展引力波干涉仪所需要的低噪声科技。

└─ 331ms  · https://zh.wikipedia.org/wiki/不确定性原理
```

## Install

### Homebrew

```bash
brew install rscli/tap/wiki
```

### Cargo

```bash
cargo install --git https://github.com/rscli/wikipedia-cli
```

### Build from source

```bash
git clone https://github.com/rscli/wikipedia-cli.git
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

Language is auto-detected from the query's Unicode script. Supports English, Chinese (Simplified/Traditional), Japanese, Korean, Arabic, Russian, Hindi, Thai, Hebrew, Greek, Tamil, Bengali, Telugu, Turkish, and Vietnamese. Use `-l` to override.

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
