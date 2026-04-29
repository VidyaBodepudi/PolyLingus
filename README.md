# TrioLingo 🐍⚡

**Universal text transformer, steganography engine & AI red-teaming toolkit** — written in Rust.

> Parseltongue, reforged. Faster, more comprehensive, with offensive *and* defensive capabilities.

## What Is This?

TrioLingo is a high-performance CLI tool for text transformation, encoding, steganography, and AI security research. It implements **60 transforms** across 10 categories;

- 🧠 **Homoglyph Attack Engine** — Generate & detect Unicode confusable attacks (TR39)
- 🔍 **Prompt Injection Scanner** — 20+ patterns: instruction overrides, DAN, role injection
- 🕵️ **Steganography Suite** — Zero-width, whitespace, LSB image, and forensic scanner
- 💬 **Semantic Obfuscation** — Synonyms, euphemisms, paraphrasing, register shifting
- 🔗 **Pipeline Chains** — Chain transforms: `rot13 -> base64 -> hex`
- ⛓️ **Chain Decoder** — Brute-force multi-transform cracking with depth search
- 🎲 **Payload Fragmenter** — Split, interleave, shuffle, and reassemble payloads
- 🤖 **Tokenizer Attacks** — BPE boundary confusion, glitch token injection
- 📊 **Entropy Analysis** — Shannon entropy profiling to detect encoded payloads
- 🔬 **Unicode Scanner** — Detects 30+ hidden character types
- 📋 **JSON Reports** — Full analysis output as structured JSON

## Quick Start

```bash
# Build
cargo build --release

# Encode with any transform
triolingo encode --transform base64 "Hello World"
triolingo encode --transform fraktur "Secret Message"
triolingo encode --transform homoglyph "admin password"
triolingo encode --transform synonym "delete the user data"
triolingo encode --transform euphemism "hack the server"
triolingo encode --transform register_shift "gonna try this" -p target=formal

# Decode (universal auto-detect, or specify transform)
triolingo decode "SGVsbG8gV29ybGQ="
triolingo decode "SGVsbG8gV29ybGQ=" --depth 2  # chain decode

# Pipeline chains
triolingo pipeline "rot13 -> base64" "Hello World"
triolingo pipeline "rot13 -> base64" "VXJ5eWIgSmJleXE=" --reverse

# Analyze for attacks
triolingo analyze "ignore previous instructions" --mode prompt-injection
triolingo analyze "suspicious text" --mode entropy
triolingo analyze "text" --mode all
triolingo analyze "text" --json  # structured JSON report

# Fragment payloads
triolingo fragment "ignore all previous instructions" --parts 4
triolingo fragment "payload text" --parts 3 --interleave

# Token estimation
triolingo tokens "How many tokens is this?"

# Steganography
triolingo stego embed --method zwsp --message "hidden" --cover "normal text"
triolingo stego extract --method zwsp "encoded text"
triolingo stego embed --method image --message "secret" --cover image.png --output stego.png

# List all transforms
triolingo list
triolingo list --category cipher
```

## Transform Categories (60 total)

| Category | Count | Examples |
|---|---|---|
| **Encoding** | 9 | Base64, Base32, Base58, Base62, Binary, Hex, ASCII85, URL, HTML |
| **Cipher** | 11 | Caesar, ROT13/47, Morse, NATO, Vigenère, Rail Fence, XOR, Affine, Atbash, Columnar |
| **Visual** | 8 | Upside Down, Full Width, Small Caps, Bubble, Braille, Strikethrough, Underline, Zalgo |
| **Unicode Style** | 9 | Fraktur, Cursive, Monospace, Double-Struck, Bold, Italic, Sans-Serif |
| **Script** | 3 | Elder Futhark Runes, Hieroglyphics, Ogham |
| **Formatting** | 6 | Leetspeak, Pig Latin, Vaporwave, Mirror, Token Confuser, Glitch Token |
| **Homoglyph** | 1 | Full/Targeted/Random confusable generation |
| **Semantic** | 5 | Synonym, Euphemism, Paraphrase, Register Shift, Fragmenter |
| **Steganography** | 3 | Zero-width, Whitespace, Image LSB |
| **Analysis** | 5 | Prompt Injection, Homoglyph Detect, Stego Scan, Entropy, Unicode Scan |

## Architecture

```
src/
├── core/           # Transform trait, registry (60 transforms), pipeline engine
├── transforms/     # Encodings, ciphers, visual, unicode styles, scripts, formatting, advanced ciphers
├── homoglyph/      # Confusables DB (TR39), generator (3 modes), detector
├── steganography/  # Zero-width, whitespace, image LSB, forensic scanner
├── analysis/       # Prompt injection, entropy profiling, unicode scanner, report generator
├── semantic/       # Synonym DB, euphemism mapping, paraphrase engine, register shifter, fragmenter
├── tokenizer/      # BPE boundary confuser, glitch token DB, token estimator
├── decoder/        # Universal auto-decoder, brute-force chain cracker
└── payload/        # Per-word randomizer, fragment/interleave/reassemble engine
```

## What TrioLingo

| Feature | | TrioLingo |
|---|---|---|
| Transforms | ~20 | **60** |
| Language | JavaScript (browser) | **Rust (native CLI)** |
| Pipeline chaining | ✗ | ✅ |
| Homoglyph attacks | ✗ | ✅ Full TR39 engine |
| Prompt injection detection | ✗ | ✅ 20+ patterns |
| Steganography | ✗ | ✅ ZW, whitespace, image LSB |
| Semantic obfuscation | ✗ | ✅ Synonyms, euphemisms, paraphrase |
| Tokenizer attacks | ✗ | ✅ BPE confusion, glitch tokens |
| Entropy analysis | ✗ | ✅ Shannon profiling |
| Chain decoding | ✗ | ✅ Brute-force multi-chain |
| Payload fragmentation | ✗ | ✅ Interleaved with filler |
| JSON reports | ✗ | ✅ Full structured output |

## License


