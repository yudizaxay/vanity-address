# Usage guide

How to run **vanity-address** in interactive mode, direct CLI, and JSON automation.

---

## Interactive mode (default)

Just run — no flags needed:

```bash
vanity-address
```

```
╔══════════════════════════════════════════╗
║         vanity-address  v0.3.7           ║
╚══════════════════════════════════════════╝

  [1]  Start a new grind
  [2]  Help & pattern rules
  [3]  Exit

  Choose option [1-3]:
```

The wizard walks you through: **chain → prefix/suffix → pattern → estimate → confirm → grind**.

---

## Direct mode (power users / scripts)

```bash
vanity-address --chain sol --suffix axay
vanity-address --chain evm --prefix dead --suffix beef -q
```

---

## Chain examples

### Solana

```bash
vanity-address --chain sol --suffix axay
vanity-address --chain sol --prefix DeFi
vanity-address --chain sol --prefix DeFi --suffix axay
vanity-address --chain sol --suffix axay --exact
```

Example output:

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
vanity-address --chain evm --suffix beef
vanity-address --chain evm --prefix dead
vanity-address --chain evm --prefix dead --suffix beef
```

```bash
$ vanity-address --chain evm --prefix dead --suffix beef

  Chain     EVM (Ethereum)
  Target    starting with '0xdead' and ending with 'beef'
  Expected  ~1.1T attempts (average)
  ...
```

### Other chains

Use `--chain` with any supported ID:

`ada`, `algo`, `aptos`, `btc`, `cosmos`, `doge`, `dot`, `evm`, `fil`, `hedera`, `icp`, `kaspa`, `ltc`, `near`, `osmo`, `sol`, `sui`, `ton`, `trx`, `xlm`, `xrp`, `xtz`

---

## JSON output (automation)

Use `--json` with `--prefix` and/or `--suffix` for machine-readable stdout (errors go to stderr as JSON):

```bash
vanity-address --chain sol --suffix ax --json --no-benchmark --force
```

Example success payload:

```json
{
  "version": "0.3.4",
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

> **Security:** JSON includes private keys on stdout. Use only in trusted environments. See [SECURITY.md](../SECURITY.md).

Combine with `--save` / `--output` to persist keys; `saved_to` is included in the JSON when saving.

---

## CLI reference

| Flag                 | Description                                        | Default |
| -------------------- | -------------------------------------------------- | ------- |
| `--chain <ID>`       | Blockchain (see [chain examples](#other-chains))   | `sol`   |
| `--prefix <PATTERN>` | Address must start with pattern                    | —       |
| `--suffix <PATTERN>` | Address must end with pattern                      | —       |
| `--exact`            | Exact casing (base58 chains)                       | off     |
| `--save`             | Append match (incl. private keys) to file          | off     |
| `--output <PATH>`    | Custom save file (with `--save` or interactive)    | `vanity-results.txt` |
| `--no-benchmark`     | Skip 2s speed calibration warm-up                  | off     |
| `--force`            | Allow impractical patterns in CLI mode             | off     |
| `--json`             | Machine-readable JSON on stdout (direct mode)      | off     |
| `-q, --quiet`        | Minimal plain-text output (script-friendly)        | off     |
| `--threads <N>`      | Override worker threads                            | auto    |
| `-h, --help`         | Show help                                          | —       |
| `-V, --version`      | Show version                                       | —       |

### Pattern rules

- **Base58 chains** (Solana, Bitcoin, Litecoin, Dogecoin, Tron, Ripple, Stellar, Tezos, Polkadot): no `0`, `O`, `I`, `l` (where applicable); Ripple uses its own alphabet
- **Bech32** (Cosmos, Osmosis, Kaspa, Cardano): charset `qpzry9x8gf2tvdw0s3jn54khce6mua7l` (Kaspa uses `kaspa:` separator; Cardano `addr1…`)
- **Base32** (Algorand, Filecoin, ICP): Algorand uppercase; Filecoin `f1…`; ICP principals (dashes optional in pattern)
- **Base64url** (TON): `UQ…` Wallet V4R2 non-bounceable
- **Hex chains** (EVM, Aptos, Sui, NEAR, Hedera pubkey): `0-9`, `a-f`; EVM/Aptos/Sui accept optional `0x` prefix

---

## Performance

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
