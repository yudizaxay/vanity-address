<div align="center">

<img src="assets/logo.svg" alt="Vanity Address logo" width="160" />

# vanity-address

**Fast, local, multi-chain vanity address generator**

Generate multi-chain keypairs whose public address matches your desired prefix and/or suffix — entirely on your machine. No servers. No tracking. Keys never leave your device.

<br />

<!-- Tech stack badges -->

![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=for-the-badge&logo=rust&logoColor=white)
![Solana](https://img.shields.io/badge/Solana-Supported-9945FF?style=for-the-badge&logo=solana&logoColor=white)
![Ethereum](https://img.shields.io/badge/EVM-Supported-3C3C3D?style=for-the-badge&logo=ethereum&logoColor=white)
![Bitcoin](https://img.shields.io/badge/Bitcoin-Supported-F7931A?style=for-the-badge&logo=bitcoin&logoColor=white)
![13+ Chains](https://img.shields.io/badge/Chains-13+-blue?style=for-the-badge)
![Rayon](https://img.shields.io/badge/Rayon-Parallel-DEA584?style=for-the-badge&logo=rust&logoColor=white)
![Clap](https://img.shields.io/badge/Clap-CLI-00C853?style=for-the-badge)
![Tauri](https://img.shields.io/badge/Tauri-Desktop-FFC131?style=for-the-badge&logo=tauri&logoColor=white)

<br />

<!-- Project status badges -->

[![CI](https://img.shields.io/github/actions/workflow/status/yudizaxay/vanity-address/ci.yml?style=for-the-badge&logo=githubactions&logoColor=white&label=CI)](https://github.com/yudizaxay/vanity-address/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/yudizaxay/vanity-address?style=for-the-badge&logo=github&label=Release)](https://github.com/yudizaxay/vanity-address/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)](LICENSE)
[![Issues](https://img.shields.io/github/issues/yudizaxay/vanity-address?style=for-the-badge&logo=github)](https://github.com/yudizaxay/vanity-address/issues)
[![Stars](https://img.shields.io/github/stars/yudizaxay/vanity-address?style=for-the-badge&logo=github&color=yellow)](https://github.com/yudizaxay/vanity-address/stargazers)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen?style=for-the-badge&logo=git&logoColor=white)](CONTRIBUTING.md)

<br />

[Features](#-features) ·
[Demo](#-demo) ·
[Install](#-install) ·
[Usage](#-usage) ·
[Architecture](#-architecture) ·
[Desktop App](#-desktop-app) ·
[Security](#-security) ·
[Contributing](#-contributing)

</div>

---

## ✨ Features

| Feature                     | Solana |   EVM    |
| --------------------------- | :----: | :------: |
| Prefix matching             |   ✅   |    ✅    |
| Suffix matching             |   ✅   |    ✅    |
| Case-insensitive mode       |   ✅   | ✅ (hex) |
| Exact case mode             |   ✅   |    —     |
| Parallel CPU grinding       |   ✅   |    ✅    |
| Live progress + ETA         |   ✅   |    ✅    |
| Multiple key export formats |   ✅   |    ✅    |
| 100% offline / local        |   ✅   |    ✅    |

<details>
<summary><strong>Why developers choose vanity-address</strong></summary>

<br />

|                      |                                                                             |
| -------------------- | --------------------------------------------------------------------------- |
| 🔒 **Privacy-first** | Keys generated locally — zero network calls                                 |
| ⚡ **Blazing fast**  | Multi-core parallel grinding via [rayon](https://github.com/rayon-rs/rayon) |
| 🔌 **Extensible**    | `ChainGrinder` trait — add chains without touching core                     |
| 🖥️ **Great CLI**     | Colors, spinner, ETA, quiet mode for scripts                                |
| 🪟 **Desktop app**   | **Vanity Address** — Tauri UI with the same wizard flow as the CLI          |
| 📦 **Open source**   | MIT licensed, contributions welcome                                         |
| 🛡️ **Transparent**   | Full source code — audit before you trust                                   |

</details>

---

## 🎬 Demo

<p align="center">
  <img src="assets/demo-terminal.svg" alt="Vanity Address terminal demo — interactive menu, live grind progress, and match output" width="720" />
</p>

<p align="center">
  <img src="assets/demo-desktop.svg" alt="Vanity Address desktop app — wizard summary and live grind progress" width="720" />
</p>

<p align="center">
  <sub>CLI interactive menu or <strong>Vanity Address</strong> desktop app — chain wizard → live benchmark → grind → export keys</sub>
</p>

### Solana

```bash
$ vanity-address --chain sol --suffix axay

vanity-address
Local multi-chain vanity address generator

  Chain     Solana
  Target    ending with 'axay'
  Mode      any case
  Expected  ~656.4M attempts (average)
  Hint      Base58 characters only. Invalid: 0, O, I, l

⠋ 12.4M keys | 2,198,421 keys/s | ~5 min remaining

 Match found!

  Address   7xKp...Qaxay
  Time      142.31s
  Attempts  312,847,291

 Private Keys
  Never share these with anyone.
```

### EVM (Ethereum)

```bash
$ vanity-address --chain evm --prefix dead --suffix beef

  Chain     EVM (Ethereum)
  Target    starting with '0xdead' and ending with 'beef'
  Expected  ~1.1T attempts (average)
  ...
```

---

## 📦 Install

### Pre-built binaries (recommended)

Download the latest release for your platform — no Rust toolchain required:

| Platform | CLI archive | Desktop |
| -------- | ----------- | ------- |
| **Linux** (x86_64) | [`*-linux-x86_64.tar.gz`](https://github.com/yudizaxay/vanity-address/releases/latest) | build from source |
| **macOS** (Apple Silicon) | [`*-macos-arm64.tar.gz`](https://github.com/yudizaxay/vanity-address/releases/latest) | [`*-macos-arm64-desktop.tar.gz`](https://github.com/yudizaxay/vanity-address/releases/latest) (`.dmg` inside) |
| **macOS** (Intel) | [`*-macos-x86_64.tar.gz`](https://github.com/yudizaxay/vanity-address/releases/latest) | build from source |
| **Windows** (x86_64) | [`*-windows-x86_64.zip`](https://github.com/yudizaxay/vanity-address/releases/latest) | build from source |

```bash
# Linux example (check Releases page for exact version)
curl -LO https://github.com/yudizaxay/vanity-address/releases/download/v0.3.0/vanity-address-0.3.0-linux-x86_64.tar.gz
tar xzf vanity-address-0.3.0-linux-x86_64.tar.gz
cd vanity-address-0.3.0-linux-x86_64
./vanity-address
```

```bash
# macOS (Apple Silicon) CLI
curl -LO https://github.com/yudizaxay/vanity-address/releases/download/v0.3.0/vanity-address-0.3.0-macos-arm64.tar.gz
tar xzf vanity-address-0.3.0-macos-arm64.tar.gz
cd vanity-address-0.3.0-macos-arm64
./vanity-address
```

```bash
# Windows (PowerShell) — extract zip, run vanity-address.exe
```

Each CLI archive includes the binary, `README.md`, `LICENSE`, `SECURITY.md`, and a `.sha256` checksum file.

Verify downloads:

```bash
shasum -a 256 -c vanity-address-*-linux-x86_64.tar.gz.sha256
```

### Homebrew (macOS / Linux)

```bash
brew install --build-from-source ./Formula/vanity-address.rb
```

See [RELEASING.md](RELEASING.md) for tap setup and formula hash updates.

### crates.io

```bash
cargo install vanity-address
```

Requires Rust 1.70+. Publishes `vanity-address` + `vanity-core` — see [RELEASING.md](RELEASING.md#publishing-to-cratesio).

### Build from source

**Requirements:** [Rust](https://rustup.rs/) 1.70+

```bash
git clone https://github.com/yudizaxay/vanity-address.git
cd vanity-address
cargo build --release
```

Binary:

```text
target/release/vanity-address
```

**Install globally:**

```bash
cargo install --path vanity-address
```

---

## 🚀 Usage

### Interactive mode (default)

Just run — no flags needed:

```bash
./target/release/vanity-address
```

```
╔══════════════════════════════════════════╗
║         vanity-address  v0.3.0           ║
╚══════════════════════════════════════════╝

  [1]  Start a new grind
  [2]  Help & pattern rules
  [3]  Exit

  Choose option [1-3]:
```

The wizard walks you through: **chain → prefix/suffix → pattern → estimate → confirm → grind**.

### Direct mode (power users / scripts)

```bash
vanity-address --chain sol --suffix axay
vanity-address --chain evm --prefix dead --suffix beef -q
```

### JSON output (automation)

Use `--json` with `--prefix` and/or `--suffix` for machine-readable stdout (errors go to stderr as JSON):

```bash
vanity-address --chain sol --suffix ax --json --no-benchmark --force
```

Example success payload:

```json
{
  "version": "0.3.0",
  "chain": "sol",
  "chain_name": "Solana",
  "pattern": {
    "prefix": "",
    "suffix": "ax",
    "description": "ending with 'ax'",
    "ignore_case": true
  },
  "address": "7xKp…Qax",
  "exports": [{ "label": "Secret Key (base58)", "value": "…" }],
  "stats": { "attempts": 1200, "elapsed_secs": 0.04, "keys_per_sec": 30000.0 },
  "measured_keys_per_sec": 2100000.0
}
```

> **Security:** JSON includes private keys on stdout. Use only in trusted environments. See [SECURITY.md](SECURITY.md).

Combine with `--save` / `--output` to persist keys; `saved_to` is included in the JSON when saving.

### Solana

```bash
vanity-address --chain sol --suffix axay
vanity-address --chain sol --prefix DeFi
vanity-address --chain sol --prefix DeFi --suffix axay
vanity-address --chain sol --suffix axay --exact
```

### EVM

```bash
vanity-address --chain evm --suffix beef
vanity-address --chain evm --prefix dead
vanity-address --chain evm --prefix dead --suffix beef
```

### CLI reference

| Flag                 | Description                                        | Default |
| -------------------- | -------------------------------------------------- | ------- |
| `--chain <ID>`       | Blockchain: `sol`, `evm`, `btc`, `ltc`, `doge`, `trx`, `cosmos`, `osmo`, `xrp`, `xlm`, `aptos`, `sui`, `near` | `sol`   |
| `--prefix <PATTERN>` | Address must start with pattern                    | —       |
| `--suffix <PATTERN>` | Address must end with pattern                      | —       |
| `--exact`            | Exact casing (base58 chains)                       | off     |
| `--save`             | Append match (incl. private keys) to `vanity-results.txt` | off |
| `--output <PATH>`    | Custom save file (with `--save` or interactive save) | `vanity-results.txt` |
| `--no-benchmark`     | Skip 2s speed calibration warm-up before grinding | off |
| `--force`            | Allow impractical patterns in CLI mode (blocked by default) | off |
| `--json`             | Machine-readable JSON on stdout (direct mode)      | off     |
| `-q, --quiet`        | Minimal plain-text output (script-friendly)        | off     |
| `--threads <N>`      | Override worker threads (auto-detected by default) | auto    |
| `-h, --help`         | Show help                                          | —       |
| `-V, --version`      | Show version                                       | —       |

> **Pattern rules**
>
> - **Base58 chains** (Solana, Bitcoin, Litecoin, Dogecoin, Tron, Ripple, Stellar): no `0`, `O`, `I`, `l` (Solana alphabet); Ripple uses its own alphabet
> - **Bech32** (Cosmos, Osmosis): `qpzry9x8gf2tvdw0s3jn54khce6mua7l` or full address with `cosmos1` / `osmo1` prefix
> - **Hex chains** (EVM, Aptos, Sui, NEAR): `0-9`, `a-f`; EVM/Aptos/Sui accept optional `0x` prefix

---

## 🏗 Architecture

```text
┌───────────────────┐   ┌───────────────────┐
│  vanity-address    │   │    vanity-app      │
│      (CLI)         │   │  (Tauri desktop UI)│
├───────────────────┴───┴───────────────────┤
│                vanity-core lib              │
│  ┌────────┐ ┌─────┐ ┌──────────┐ ┌───────┐ │
│  │ Solana │ │ EVM │ │ Bitcoin… │ │  +10  │ │
│  │Grinder │ │Grind│ │ Grinders │ │ more  │ │
│  └────────┘ └─────┘ └──────────┘ └───────┘ │
│              ChainGrinder trait             │
└─────────────────────────────────────────────┘
```

New chain = one file + trait implementation in `vanity-core/src/chains/`. Both the CLI and the desktop UI are thin frontends over the same `vanity-core` grinding engine — no chain logic is duplicated.

---

## 🖥 Desktop App

**Vanity Address** is the native desktop UI (`vanity-app/`), built with [Tauri 2](https://tauri.app/) and wired directly to `vanity-core` — no duplicated chain logic.

### Wizard flow (same as CLI)

```text
Home → Chain → Pattern (suffix/prefix/both) → Summary → Grind → Result
         ↑ Help              ↑ system info + warnings    ↑ stop anytime
```

| Feature | Desktop |
| ------- | ------- |
| 13 chains | ✅ |
| Live estimate + difficulty | ✅ |
| Impractical-pattern warning + double confirm | ✅ |
| 2s speed benchmark before grind | ✅ |
| Stop mid-grind | ✅ |
| Masked keys + reveal / copy all | ✅ |
| Native save dialog | ✅ |

### Requirements

- [Rust](https://rustup.rs/) 1.70+ (same as CLI)
- [Node.js](https://nodejs.org/) 18+
- macOS or Linux for `tauri build` (see [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/))

### Run / build

```bash
cd vanity-app
npm install
npm run tauri dev      # development (hot reload)
npm run tauri build    # native .app / .dmg (macOS) or .deb / .AppImage (Linux)
```

Output lands in `vanity-app/src-tauri/target/release/bundle/`.

Grinding runs on a background thread; keys stay masked until you click **Reveal**, and **Save** opens a native file picker (nothing is written automatically).

---

## ⚡ Performance

On startup, **vanity-address** (CLI and desktop app) probes your system (CPU cores, RAM) and runs a **2-second benchmark** when grinding starts for an honest ETA:

| Signal              | Behavior                                                                       |
| ------------------- | ------------------------------------------------------------------------------ |
| CPU cores           | Uses all cores on 1–2 core machines; reserves 1 core for OS on larger machines |
| Physical vs logical | Avoids oversubscribing physical cores when hyperthreading is present           |
| Available RAM       | Reduces workers on low-memory systems to prevent pressure                      |
| Progress interval   | Scales with worker count to keep UI smooth without overhead                    |

```bash
# Auto-detect (recommended)
vanity-address --chain sol --suffix axay

# Manual override
vanity-address --chain sol --suffix axay --threads 4
```

Example startup:

```text
  System    8 logical / 8 physical · 16.0 GB RAM (12.0 GB free) · 7 workers · memory: comfortable
  Chain     Solana
  Target    ending with 'axay'
```

| Pattern length | Solana (case-insensitive) | EVM (hex)     |
| -------------- | ------------------------- | ------------- |
| 4 chars        | ~7M attempts              | ~65K attempts |
| 6 chars        | ~656M attempts            | ~16M attempts |
| 8 chars        | ~58B attempts             | ~4B attempts  |

Longer patterns = exponentially harder. Start short, verify, then go longer.

---

## 🔐 Security

> **⚠️ Read before generating keys**
>
> | Rule         | Detail                                                  |
> | ------------ | ------------------------------------------------------- |
> | Local only   | Keys are generated on **your machine**                  |
> | No network   | This tool **never connects to the internet**            |
> | Never share  | **Do not** share private keys with anyone               |
> | Verify first | Always double-check the address before sending funds    |
> | Open source  | Audit the code — trust, but verify                      |
> | Risk         | Vanity grinding is probabilistic — use at your own risk |

---

## 🗺 Roadmap

- [x] Solana suffix / prefix grinding
- [x] EVM (Ethereum) support
- [x] Multi-chain `ChainGrinder` architecture
- [x] Polished CLI (colors, progress, `--chain`)
- [x] Auto system detection (CPU + memory tuned thread pool)
- [x] Desktop UI ([Tauri](https://tauri.app/)) — `vanity-app/`
- [x] Bitcoin, Litecoin, Dogecoin, Tron, Cosmos, Osmosis, Ripple, Stellar, Aptos, Sui, NEAR
- [ ] Cardano, TON (complex address formats)
- [ ] Regex patterns for power users

---

## 🤝 Contributing

We love contributions — **new blockchains, bug fixes, features, docs, and tests** are all welcome!

📖 **[CONTRIBUTING.md](CONTRIBUTING.md)** — full guide (add a chain, PR checklist, code style)

| Want to… | Start here |
|----------|------------|
| Add a blockchain | `vanity-core/src/chains/` + [contributing guide](CONTRIBUTING.md#adding-a-new-blockchain) |
| Fix a bug / UX issue | Fork → branch → PR with repro steps |
| Propose a feature | [Open a feature request](https://github.com/yudizaxay/vanity-address/issues/new?template=feature_request.yml) for big changes |

### Development checks

Before opening a PR, run the same checks as [CI](.github/workflows/ci.yml):

```bash
make check        # fmt + test + clippy + frontend build
make check-ci     # same, but fmt-check only (what CI runs)
```

Or manually:

```bash
# Format Rust (apply fixes)
cargo fmt --all

# Format check only — what CI runs (no file changes)
cargo fmt --all -- --check

# Tests
cargo test -p vanity-core -p vanity-address
cargo test -p vanity-app

# Lint
cargo clippy -p vanity-core -p vanity-address -- -D warnings
cargo clippy -p vanity-app -- -D warnings

# Desktop frontend (TypeScript + Vite)
cd vanity-app && npm ci && npm run build && cd ..
```

**One-liner** (quick pre-push):

```bash
cargo fmt --all && cargo test && cargo clippy -- -D warnings && cd vanity-app && npm ci && npm run build && cd ..
```

See [SECURITY.md](SECURITY.md) before reporting vulnerabilities publicly.

```bash
git checkout -b feat/my-feature
# … make changes, then run make check …
```

PRs use the [pull request template](.github/PULL_REQUEST_TEMPLATE.md) — fill it in so reviewers can merge faster.

1. Fork the repo
2. Create your branch (`feat/…`, `fix/…`, `docs/…`)
3. Commit with a clear message
4. Push and open a PR

---

## 📄 License

This project is licensed under the **[MIT License](LICENSE)** — free for personal and commercial use.

---

<div align="center">

**Built with 🦀 Rust** · Keys stay on your machine

<br />

[![GitHub](https://img.shields.io/badge/GitHub-yudizaxay%2Fvanity--address-181717?style=flat-square&logo=github)](https://github.com/yudizaxay/vanity-address)
[![Report Bug](https://img.shields.io/badge/Report-Bug-red?style=flat-square&logo=github)](https://github.com/yudizaxay/vanity-address/issues/new?template=bug_report.yml)
[![Request Feature](https://img.shields.io/badge/Request-Feature-blue?style=flat-square&logo=github)](https://github.com/yudizaxay/vanity-address/issues/new?template=feature_request.yml)

<br />

<sub>If this project helped you, consider giving it a ⭐ on GitHub!</sub>

</div>
