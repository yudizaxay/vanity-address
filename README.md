# vanity-orbt

Fast, local Solana vanity address generator. Grind keypairs until the public address matches your desired prefix and/or suffix — all on your machine, with no network calls.

## Security

> **Warning**
> - Private keys are generated **locally on your machine**
> - Never share your private key with anyone
> - This tool does not connect to the internet
> - Always verify the address before sending funds
> - Use at your own risk

## Install

Requires [Rust](https://rustup.rs/) 1.70+.

```bash
git clone https://github.com/YOUR_USERNAME/vanity-orbt.git
cd vanity-orbt
cargo build --release
```

Binary: `target/release/vanity-orbt`

## Usage

```bash
# Suffix only (default: orbt)
vanity-orbt --suffix orbt

# Prefix only
vanity-orbt --prefix ABC

# Prefix + suffix
vanity-orbt --prefix DeFi --suffix orbt

# Exact case (Solana addresses are case-sensitive in exact mode)
vanity-orbt --suffix ORBT --exact

# Help
vanity-orbt --help
```

### Flags

| Flag | Description |
|------|-------------|
| `--suffix <PATTERN>` | Address must end with this pattern |
| `--prefix <PATTERN>` | Address must start with this pattern |
| `--exact` | Require exact casing (default: case-insensitive) |

Patterns must use base58 characters. The characters `0`, `O`, `I`, and `l` are invalid — they never appear in Solana addresses.

## Output formats

On success, the tool prints:

- **Public key** — the vanity address
- **Private key (hex)** — raw secret bytes
- **Private key (base58)** — import into Phantom, Solflare, etc.
- **Keypair (JSON)** — `solana-cli` format

## Performance

Uses [rayon](https://github.com/rayon-rs/rayon) for parallel CPU grinding. Speed depends on your CPU core count. Longer patterns take exponentially longer — a 6-character suffix averages ~58^6 attempts.

## Roadmap

- [x] Solana suffix grinding
- [x] Prefix support
- [x] Case-insensitive / exact case
- [x] CLI flags (clap)
- [ ] EVM (Ethereum) support
- [ ] Desktop UI (Tauri)

## License

MIT
