<div align="center">

<img src="https://raw.githubusercontent.com/yudizaxay/vanity-address/main/assets/logo.svg" alt="Vanity Address logo" width="120" />

# vanity-address

**Fast, local, multi-chain vanity address generator — right from `npx`**

[![npm](https://img.shields.io/npm/v/vanity-address?style=flat-square&logo=npm&color=cb3837)](https://www.npmjs.com/package/vanity-address)
[![downloads](https://img.shields.io/npm/dm/vanity-address?style=flat-square&color=blue)](https://www.npmjs.com/package/vanity-address)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](https://github.com/yudizaxay/vanity-address/blob/main/LICENSE)
[![GitHub](https://img.shields.io/github/stars/yudizaxay/vanity-address?style=flat-square&logo=github)](https://github.com/yudizaxay/vanity-address)

Generate cryptocurrency keypairs whose public address starts or ends with the pattern you want — **100% offline, on your machine**. No servers, no tracking, keys never leave your device.

</div>

---

## Quick start

```bash
# Run instantly (no install)
npx vanity-address

# Or install globally
npm install -g vanity-address
vanity-address
```

Running without flags opens an **interactive wizard**: pick a chain → enter prefix/suffix → see a realistic time estimate → grind.

## Examples

```bash
# Solana address ending in "moon"
npx vanity-address --chain sol --suffix moon

# Ethereum address starting with 0xdead
npx vanity-address --chain evm --prefix dead

# Bitcoin address, save keys to file
npx vanity-address --chain btc --prefix 1Cool --save

# Machine-readable JSON output (for scripts)
npx vanity-address --chain sol --suffix ax --json --no-benchmark
```

## Supported chains (13)

| Chain | ID | Address format |
| ----- | -- | -------------- |
| Solana | `sol` | base58 (Phantom, Solflare) |
| Ethereum / EVM | `evm` | 0x hex (MetaMask, all EVM chains) |
| Bitcoin | `btc` | base58 P2PKH |
| Litecoin | `ltc` | base58 P2PKH |
| Dogecoin | `doge` | base58 |
| Tron | `trx` | base58 (T…) |
| Cosmos | `cosmos` | bech32 (ATOM) |
| Osmosis | `osmo` | bech32 |
| Ripple | `xrp` | base58 (r…) |
| Stellar | `xlm` | strkey (G…) |
| Aptos | `aptos` | 0x hex |
| Sui | `sui` | 0x hex |
| NEAR | `near` | hex implicit account |

## Key flags

| Flag | Description |
| ---- | ----------- |
| `--chain <ID>` | Blockchain (see table above) |
| `--prefix <PATTERN>` | Address must start with pattern |
| `--suffix <PATTERN>` | Address must end with pattern |
| `--exact` | Case-sensitive matching (base58 chains) |
| `--save` | Append keys to `vanity-results.txt` |
| `--json` | Machine-readable output for scripts |
| `--threads <N>` | Limit CPU threads |
| `-q, --quiet` | Minimal output |

Full usage guide: [docs/USAGE.md](https://github.com/yudizaxay/vanity-address/blob/main/docs/USAGE.md)

## How it works

This npm package is a lightweight wrapper — **no Rust toolchain or compilation needed**. It ships a pre-built native binary for your platform via `optionalDependencies` (the same pattern used by esbuild and Biome):

| Package | Platform |
| ------- | -------- |
| `vanity-address-darwin-arm64` | macOS Apple Silicon (M1–M4) |
| `vanity-address-darwin-x64` | macOS Intel |
| `vanity-address-linux-x64` | Linux x86_64 |
| `vanity-address-win32-x64` | Windows x64 |

npm automatically installs only the binary matching your OS. No `postinstall` scripts. Binaries are built from source by [GitHub Actions](https://github.com/yudizaxay/vanity-address/actions) and identical to [GitHub Release](https://github.com/yudizaxay/vanity-address/releases) assets.

**Performance:** parallel grinding across all CPU cores (rayon), with a live benchmark for honest ETAs — millions of keys/second on modern hardware.

## Security

- 🔒 **Keys are generated locally** — this tool never connects to the internet
- 🚫 No telemetry, no accounts, no tracking
- ⚠️ **Never share your private keys.** Anyone with the key controls the funds
- ⚠️ If you use `--save`, protect `vanity-results.txt` — it contains private keys
- 🔍 Open source (MIT) — [audit the code](https://github.com/yudizaxay/vanity-address) before trusting any vanity tool

## Requirements

- **Node.js 18+**
- macOS (Intel / Apple Silicon), Linux x64, or Windows x64

## Other install methods

| Method | Command |
| ------ | ------- |
| Homebrew | `brew tap yudizaxay/tap && brew trust yudizaxay/tap && brew install vanity-address` |
| Cargo (Rust) | `cargo install vanity-address` |
| Pre-built binaries | [GitHub Releases](https://github.com/yudizaxay/vanity-address/releases/latest) |
| Desktop app (GUI) | [Download .dmg / .exe](https://github.com/yudizaxay/vanity-address/releases/latest) |

## Links

- 📖 [Documentation](https://github.com/yudizaxay/vanity-address#readme)
- 🚀 [Usage guide](https://github.com/yudizaxay/vanity-address/blob/main/docs/USAGE.md)
- 📦 [Install guide](https://github.com/yudizaxay/vanity-address/blob/main/docs/INSTALL.md)
- 🐛 [Report issues](https://github.com/yudizaxay/vanity-address/issues)
- 🔐 [Security policy](https://github.com/yudizaxay/vanity-address/blob/main/SECURITY.md)

---

<div align="center">

**MIT © [yudizaxay](https://github.com/yudizaxay)**

If this tool saved you time, consider [starring the repo ⭐](https://github.com/yudizaxay/vanity-address)

</div>
