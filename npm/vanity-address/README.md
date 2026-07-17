<div align="center">

<img src="https://raw.githubusercontent.com/yudizaxay/vanity-address/main/assets/logo.svg" alt="Vanity Address logo" width="120" />

# vanity-address

**Generate custom crypto wallet addresses that start or end with YOUR word — Solana, Ethereum, Bitcoin + 10 more chains.**

[![npm](https://img.shields.io/npm/v/vanity-address?style=flat-square&logo=npm&color=cb3837)](https://www.npmjs.com/package/vanity-address)
[![downloads](https://img.shields.io/npm/dm/vanity-address?style=flat-square&color=blue)](https://www.npmjs.com/package/vanity-address)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](https://github.com/yudizaxay/vanity-address/blob/main/LICENSE)
[![GitHub](https://img.shields.io/github/stars/yudizaxay/vanity-address?style=flat-square&logo=github)](https://github.com/yudizaxay/vanity-address)

**100% offline · keys never leave your machine · no telemetry · open source**

</div>

---

## What is a vanity address?

A normal crypto wallet address is random gibberish:

```text
7xKpN4qwRnLB2fGhy9uMcVTAeWDs38ZjQaU5vXrE6bYt
```

A **vanity address** is a real, fully working wallet address that contains a word **you** choose — at the start or the end:

```text
7xKpN4qwRnLB2fGhy9uMcVTAeWDs38ZjQaU5vXrmoon   ← ends with "moon"
DeFiN4qwRnLB2fGhy9uMcVTAeWDs38ZjQaU5vXrE6bY   ← starts with "DeFi"
0xdead...cafe                                  ← Ethereum with dead / cafe
```

### Why would I want one?

- **Branding** — projects, DAOs and creators use recognizable deposit addresses (`...pump`, `...dao`, `...shop`)
- **Personal identity** — your name or handle in your wallet address
- **Easy verification** — you (and your users) can spot the right address at a glance and avoid copy-paste scams

### How does this tool make one?

There is no shortcut in cryptography — the tool rapidly generates **millions of random keypairs per second** on your CPU and keeps the first one whose address matches your pattern. That keypair is 100% standard: import the private key into Phantom, MetaMask, or any normal wallet and use it like any other account.

Everything runs **locally on your machine**. Nothing is sent anywhere.

---

## Quick start

```bash
# No install needed — runs instantly
npx vanity-address
```

This opens an interactive wizard: **pick a chain → type your word → see the time estimate → grind → get your keys.**

Or install globally:

```bash
npm install -g vanity-address
vanity-address
```

> Requires **Node.js 18+**. Works on macOS (Intel & Apple Silicon), Linux x64, Windows x64.

---

## Examples

```bash
# Solana address ending in "moon"
npx vanity-address --chain sol --suffix moon

# Ethereum address starting with 0xdead
npx vanity-address --chain evm --prefix dead

# Both prefix AND suffix
npx vanity-address --chain sol --prefix Cool --suffix xyz

# Save the keys to a file (vanity-results.txt)
npx vanity-address --chain sol --suffix moon --save

# JSON output for scripts / automation
npx vanity-address --chain sol --suffix ax --json --no-benchmark
```

Example output:

```text
  Chain     Solana
  Target    ending with 'moon'
  Expected  ~656M attempts (average)

⠋ 12.4M keys | 2,198,421 keys/s | ~5 min remaining

 Match found!

  Address   7xKpN4qwRnLB2fGhy9uMcVTAeWDs38ZjQaU5vXrmoon
  Time      142.31s

 Private Keys
  Never share these with anyone.
```

> ⏱ **Tip:** short patterns (3–4 characters) take seconds-to-minutes. Every extra character multiplies the time by ~30–60×. The tool shows an honest estimate **before** starting, so you can decide.

---

## Supported chains (13)

| Chain | `--chain` | Address style | Works with |
| ----- | --------- | ------------- | ---------- |
| Solana | `sol` | base58 | Phantom, Solflare |
| Ethereum + all EVM | `evm` | `0x` hex | MetaMask, Rabby |
| Bitcoin | `btc` | base58 (P2PKH) | Electrum, Sparrow |
| Litecoin | `ltc` | base58 | — |
| Dogecoin | `doge` | base58 | — |
| Tron | `trx` | base58 (`T…`) | TronLink |
| Cosmos | `cosmos` | bech32 (`cosmos1…`) | Keplr |
| Osmosis | `osmo` | bech32 (`osmo1…`) | Keplr |
| Ripple | `xrp` | base58 (`r…`) | — |
| Stellar | `xlm` | strkey (`G…`) | — |
| Aptos | `aptos` | `0x` hex | Petra |
| Sui | `sui` | `0x` hex | Sui Wallet |
| NEAR | `near` | hex implicit | — |

---

## All flags

| Flag | What it does | 
| ---- | ------------ |
| `--chain <ID>` | Which blockchain (see table above) |
| `--prefix <WORD>` | Address must **start** with this |
| `--suffix <WORD>` | Address must **end** with this |
| `--exact` | Match upper/lowercase exactly |
| `--save` | Save keys to `vanity-results.txt` |
| `--output <PATH>` | Custom file for saved keys |
| `--json` | Machine-readable output |
| `--threads <N>` | Limit CPU threads used |
| `--no-benchmark` | Skip the 2s speed warm-up |
| `-q, --quiet` | Minimal output |

Full guide with pattern rules per chain: [docs/USAGE.md](https://github.com/yudizaxay/vanity-address/blob/main/docs/USAGE.md)

---

## Is it safe?

| Question | Answer |
| -------- | ------ |
| Does it send my keys anywhere? | **No.** The tool makes zero network connections. |
| Are the wallets real? | Yes — standard keypairs, importable into Phantom / MetaMask / etc. |
| Is it open source? | Yes, MIT licensed — [read the code](https://github.com/yudizaxay/vanity-address) |
| Telemetry / analytics? | None. |

⚠️ **Golden rule:** whoever has the private key controls the funds. Never share it, and protect `vanity-results.txt` if you use `--save`.

---

## How the npm package works

This package doesn't compile anything on your machine. It ships a **pre-built native binary** (written in Rust) for your platform via `optionalDependencies` — the same approach used by esbuild and Biome:

| Platform package | For |
| ---------------- | --- |
| `vanity-address-darwin-arm64` | macOS Apple Silicon (M1–M4) |
| `vanity-address-darwin-x64` | macOS Intel |
| `vanity-address-linux-x64` | Linux x86_64 |
| `vanity-address-win32-x64` | Windows x64 |

npm installs only the one matching your OS. No `postinstall` scripts. Binaries are built by [GitHub Actions](https://github.com/yudizaxay/vanity-address/actions) and are identical to the [GitHub Release](https://github.com/yudizaxay/vanity-address/releases) assets.

**Speed:** grinds on all CPU cores in parallel — millions of keys per second on modern hardware.

---

## Prefer another install method?

| Method | Command |
| ------ | ------- |
| Homebrew (macOS/Linux) | `brew tap yudizaxay/tap && brew trust yudizaxay/tap && brew install vanity-address` |
| Cargo (Rust) | `cargo install vanity-address` |
| Direct download | [GitHub Releases](https://github.com/yudizaxay/vanity-address/releases/latest) |
| **Desktop app (GUI)** | [.dmg / .exe downloads](https://github.com/yudizaxay/vanity-address/releases/latest) — same engine, point-and-click |

---

## Links

- 📖 [Full documentation](https://github.com/yudizaxay/vanity-address#readme)
- 🚀 [Usage guide (all chains + JSON)](https://github.com/yudizaxay/vanity-address/blob/main/docs/USAGE.md)
- 📦 [Install guide](https://github.com/yudizaxay/vanity-address/blob/main/docs/INSTALL.md)
- 🐛 [Report an issue](https://github.com/yudizaxay/vanity-address/issues)
- 🔐 [Security policy](https://github.com/yudizaxay/vanity-address/blob/main/SECURITY.md)

---

<div align="center">

**MIT © [yudizaxay](https://github.com/yudizaxay)**

If this saved you time, [star the repo ⭐](https://github.com/yudizaxay/vanity-address)

</div>
