<div align="center">

<img src="https://raw.githubusercontent.com/yudizaxay/vanity-address/main/assets/logo.svg" alt="Vanity Address logo" width="160" />

# vanity-address

**Fast, local, multi-chain vanity address generator**

Generate multi-chain keypairs whose public address matches your desired prefix, suffix, and/or contains pattern — entirely on your machine. No servers. No tracking. Keys never leave your device.

**Privacy-first:** keys are generated on your device only. No accounts. No telemetry. Audit the source before you trust any vanity tool.

<br />

[![CI](https://img.shields.io/github/actions/workflow/status/yudizaxay/vanity-address/ci.yml?style=for-the-badge&logo=githubactions&logoColor=white&label=CI)](https://github.com/yudizaxay/vanity-address/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/yudizaxay/vanity-address?style=for-the-badge&logo=github&label=Release)](https://github.com/yudizaxay/vanity-address/releases)
[![crates.io](https://img.shields.io/crates/v/vanity-address?style=for-the-badge&logo=rust&label=crates.io)](https://crates.io/crates/vanity-address)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)](https://github.com/yudizaxay/vanity-address/blob/main/LICENSE)
[![Security Policy](https://img.shields.io/badge/Security-Policy-green?style=for-the-badge&logo=github)](https://github.com/yudizaxay/vanity-address/blob/main/SECURITY.md)
[![Keys stay local](https://img.shields.io/badge/Keys-100%25%20local-success?style=for-the-badge)](#-security)
[![No network](https://img.shields.io/badge/Network-none-lightgrey?style=for-the-badge)](#-security)
[![Stars](https://img.shields.io/github/stars/yudizaxay/vanity-address?style=for-the-badge&logo=github&color=yellow)](https://github.com/yudizaxay/vanity-address/stargazers)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen?style=for-the-badge&logo=git&logoColor=white)](https://github.com/yudizaxay/vanity-address/blob/main/CONTRIBUTING.md)

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
| Contains + `*` wildcards    |   ✅   |    ✅    |
| Multi-pattern OR            |   ✅   |    ✅    |
| Case-insensitive mode       |   ✅   | ✅ (hex) |
| Exact case mode             |   ✅   |    —     |
| Parallel CPU grinding       |   ✅   |    ✅    |
| Live progress + ETA         |   ✅   |    ✅    |
| Multiple key export formats |   ✅   |    ✅    |
| Contract address vanity     |   —    | ✅ CREATE + CREATE2 |
| Token mint vanity           | ✅ SPL |    —     |
| Owner-only key files (`0600`) |  ✅   |    ✅    |
| 100% offline / local        |   ✅   |    ✅    |

**31 chains** + **contract & token-mint modes** · **CLI + desktop + npm SDK** · **MIT licensed** · **privacy-first** (keys never leave your machine)

> **New in 0.6.2:** impossible prefixes are rejected up front (no more grinding forever on e.g. Kaspa `ax`), exact difficulty estimates for every chain, and fixed address starts are added for you. **0.6.1:** vanity **contract addresses** with no factory (`--chain evm-contract`), vanity **Solana token mints** (`--chain sol-mint`), Hyperliquid + Sonic aliases, owner-only key files. See the [CHANGELOG](https://github.com/yudizaxay/vanity-address/blob/main/CHANGELOG.md).

---

## 🎬 Demo

<p align="center">
  <img src="https://raw.githubusercontent.com/yudizaxay/vanity-address/main/assets/demo-terminal.svg" alt="Vanity Address terminal demo — interactive menu, live grind progress, and match output" width="720" />
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/yudizaxay/vanity-address/main/assets/demo-desktop.svg" alt="Vanity Address desktop app — wizard summary and live grind progress" width="720" />
</p>

<p align="center">
  <sub>CLI interactive menu or <strong>Vanity Address</strong> desktop app — chain wizard → live benchmark → grind → export keys</sub>
</p>

---

## 📦 Install

**[Download from GitHub Releases](https://github.com/yudizaxay/vanity-address/releases/latest)** — no Rust or Node.js required.

| I want… | My computer | File |
| ------- | ----------- | ---- |
| **Desktop app** | Mac M1–M4 | `VanityAddress-*-Mac-AppleSilicon-Desktop.dmg` |
| **Desktop app** | Windows 10/11 | `VanityAddress-*-Windows-Desktop.exe` |
| **CLI** | Mac M1–M4 | `VanityAddress-*-Mac-AppleSilicon-CLI.tar.gz` |
| **CLI** | Mac Intel | `VanityAddress-*-Mac-Intel-CLI.tar.gz` |
| **CLI** | Windows | `VanityAddress-*-Windows-CLI.zip` |
| **CLI** | Linux | `VanityAddress-*-Linux-CLI.tar.gz` |

**Quick start (Linux):**

```bash
# Replace 0.6.2 if a newer release exists: https://github.com/yudizaxay/vanity-address/releases/latest
curl -LO https://github.com/yudizaxay/vanity-address/releases/download/v0.6.2/VanityAddress-0.6.2-Linux-CLI.tar.gz
tar xzf VanityAddress-0.6.2-Linux-CLI.tar.gz
./vanity-address
```

```bash
# Homebrew 6+: trust the tap once, then install (builds from source; requires Rust)
brew tap yudizaxay/tap && brew trust yudizaxay/tap && brew install vanity-address

# Or install via npm (pre-built binary; Node.js 18+)
npm install -g vanity-address
# npx vanity-address

# Or install the CLI from crates.io (requires Rust; first compile often takes 3–8 min)
cargo install vanity-address
```

> **Faster?** Skip compiling — download the pre-built CLI from [Releases](https://github.com/yudizaxay/vanity-address/releases/latest) (~30s).  
> **Remove later?** `cargo uninstall vanity-address` — see [Install guide → Uninstall](https://github.com/yudizaxay/vanity-address/blob/main/docs/INSTALL.md#uninstall).

> macOS may block unsigned apps — see [Install guide → Gatekeeper](https://github.com/yudizaxay/vanity-address/blob/main/docs/INSTALL.md#macos-gatekeeper-damaged-app).  
> Windows SmartScreen may warn — **More info → Run anyway**. Desktop app needs [WebView2](https://github.com/yudizaxay/vanity-address/blob/main/docs/INSTALL.md#windows--desktop-app-installer) (usually already on Windows 11).

📖 **Full guide:** [docs/INSTALL.md](https://github.com/yudizaxay/vanity-address/blob/main/docs/INSTALL.md) — per-platform steps, `.dmg` / Windows installer, checksums, Homebrew, crates.io, build from source

---

## 🚀 Usage

### Interactive mode (default)

```bash
vanity-address
```

Wizard flow: **chain → match kind → pattern → estimate → confirm → grind**.

### Direct mode

```bash
vanity-address --chain sol --suffix axay
vanity-address --chain evm --prefix dead --suffix beef -q
vanity-address --chain sol --contains pump --count 3
vanity-address --chain sol --suffix moon,pump,dao
vanity-address --chain btc-segwit --suffix cafe
vanity-address --chain robinhood --suffix dead
vanity-address verify --chain sol --address <addr> --key <hex-or-base58>
vanity-address --create2 --deployer 0x… --init-code-hash 0x… --prefix cafe
vanity-address --chain evm-contract --prefix cafe   # contract address (CREATE, deployer nonce 0)
vanity-address --chain sol-mint --suffix pump       # SPL token mint keypair
vanity-address --chain hyperliquid --prefix dead    # HyperEVM (also: sonic, base, monad, …)
```

### Common flags

| Flag              | Description                              |
| ----------------- | ---------------------------------------- |
| `--chain <ID>`    | `sol`, `evm`, `base`, `hyperliquid`, `btc-segwit`, `sei`, … or deploy mode `evm-contract` / `sol-mint` |
| `--prefix` / `--suffix` / `--contains` | Match start, end, or anywhere (`*` OK) |
| `--count N`       | Find N matches                           |
| `--json`          | Machine-readable output (scripts)        |
| `--save`          | Append keys to `vanity-results.txt` (owner-only `0600` on macOS/Linux) |
| `-q, --quiet`     | Minimal output for scripts               |

### Contract & token addresses

```bash
# EVM: grind a deployer key — its first deploy (nonce 0) lands on 0xcafe…
vanity-address --chain evm-contract --prefix cafe

# Solana: vanity token mint → save "Mint Keypair (JSON)" as mint.json
vanity-address --chain sol-mint --suffix pump
spl-token create-token mint.json
```

> For `evm-contract`, the deploy must be the deployer's **very first transaction** on that chain. Using a factory? Use `--create2` instead.

📖 **Full guide:** [docs/USAGE.md](https://github.com/yudizaxay/vanity-address/blob/main/docs/USAGE.md) — all chains, deploy modes, JSON schema, pattern rules, performance tips

---

## 📦 Use it in your own code

`npm install vanity-address` also gives you a programmatic API — no
subprocess, runs in-process via WebAssembly (Node uses multi-core workers by default):

```js
const { generateAddress, estimateDifficulty } = require("vanity-address");

console.log(estimateDifficulty({ chain: "evm", prefix: "abc", workers: 4 }));

const wallet = await generateAddress({ chain: "evm", prefix: "abc", workers: 4 });
console.log(wallet.address, wallet.privateKey);
```

Also: `generateAddresses`, `validatePattern`, `listChains`, `timeoutMs`, rich `onProgress`.

📖 **Full guide:** [docs/SDK.md](https://github.com/yudizaxay/vanity-address/blob/main/docs/SDK.md) — full API reference, workers, progress/ETA, supported chains

---

## 🏗 Architecture

```text
┌───────────────────┐   ┌─────────────────────┐
│  vanity-address   │   │    vanity-app       │
│      (CLI)        │   │  (Tauri desktop UI) │
├───────────────────┴───┴─────────────────────┤
│                vanity-core lib              │
│  ┌────────┐ ┌─────┐ ┌──────────┐ ┌───────┐  │
│  │ Solana │ │ EVM │ │ Bitcoin… │ │  +28  │  │
│  │Grinder │ │Grind│ │ Grinders │ │ more  │  │
│  └────────┘ └─────┘ └──────────┘ └───────┘  │
│              ChainGrinder trait             │
└─────────────────────────────────────────────┘
```

New chain = one file + trait implementation in `vanity-core/src/chains/`. CLI and desktop UI are thin frontends over the same engine — no duplicated chain logic.

---

## 🖥 Desktop App

**Vanity Address** (`vanity-app/`) — native Tauri UI wired directly to `vanity-core`.

```text
Home → Chain → Pattern → Summary → Grind → Result
```

| Feature | Desktop |
| ------- | ------- |
| 31 chains, live ETA, stop mid-grind | ✅ |
| Contract / token-mint modes | CLI only (for now) |
| Impractical-pattern warning | ✅ |
| Masked keys + reveal / copy / save | ✅ |

**Install:** download the Mac `.dmg` or Windows `.exe` from [Releases](https://github.com/yudizaxay/vanity-address/releases/latest) — see [docs/INSTALL.md](https://github.com/yudizaxay/vanity-address/blob/main/docs/INSTALL.md).

**Build from source:** `cd vanity-app && npm install && npm run tauri dev`

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
> | Saved keys   | `--save` files are owner-only (`0600`) on macOS/Linux; secrets are wiped from memory after use |
> | Open source  | Audit the code — trust, but verify                      |

Full policy: [SECURITY.md](https://github.com/yudizaxay/vanity-address/blob/main/SECURITY.md)

---

## 🤝 Contributing

Contributions welcome — new blockchains, bug fixes, features, docs, and tests.

Before opening a PR:

```bash
make check-ci
```

📖 **[CONTRIBUTING.md](https://github.com/yudizaxay/vanity-address/blob/main/CONTRIBUTING.md)** — add a chain, PR checklist, project structure

| Want to… | Start here |
|----------|------------|
| Add a blockchain | `vanity-core/src/chains/` + [contributing guide](https://github.com/yudizaxay/vanity-address/blob/main/CONTRIBUTING.md#adding-a-new-blockchain) |
| Fix a bug | Fork → branch → PR with repro steps |
| Propose a feature | [Open a feature request](https://github.com/yudizaxay/vanity-address/issues/new?template=feature_request.yml) |

---

## 📄 License

**[MIT License](https://github.com/yudizaxay/vanity-address/blob/main/LICENSE)** — free for personal and commercial use.

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
