# Usage guide

How to run **vanity-address** in interactive mode, direct CLI, and JSON automation.

---

> Per-chain address formats, valid characters and time estimates: [vanity-address.online/chains](https://vanity-address.online/chains/) · [difficulty calculator](https://vanity-address.online/vanity-address-difficulty/).

## Interactive mode (default)

Just run — no flags needed:

```bash
vanity-address
```

```
╔══════════════════════════════════════════╗
║         vanity-address  v0.8.0           ║
╚══════════════════════════════════════════╝

  [1]  Start a new grind
  [2]  Help & pattern rules
  [3]  Exit

  Choose option [1-3]:
```

The wizard walks you through: **chain → match kind → pattern → estimate → confirm → grind**.

---

## Direct mode (power users / scripts)

```bash
vanity-address --chain sol --suffix axay
vanity-address --chain evm --prefix dead --suffix face -q
vanity-address --chain sol --contains pump
vanity-address --chain sol --suffix moon,pump,dao --count 3
vanity-address --chain btc-segwit --suffix cafe
vanity-address --chain robinhood --suffix dead
vanity-address verify --chain evm --address 0x… --key 0x…
vanity-address --create2 --deployer 0x… --init-code-hash 0x… --prefix cafe
vanity-address --chain evm-contract --prefix cafe
vanity-address --chain trx-contract --prefix Cafe
vanity-address --chain sol-mint --suffix pump
vanity-address --chain hyperliquid --prefix dead
vanity-address --chain rvn --prefix RLove
vanity-address --puzzle 6 --prefix 1Love
```

---

## Deploy modes (contract + token addresses)

**Where:** CLI interactive menu (digits after A–Z wallets), desktop **Deploy / mint** section, npm SDK (`listDeployModes()` / `chain: "…"`), and the web generator **Address | Contract** header toggle.

Interactive CLI: digit keys pick `evm-contract`, `trx-contract`, `sol-mint`. CREATE2 needs factory params — use `--create2 --deployer … --init-code-hash …` (or the desktop CREATE2 fields / SDK options).

### EVM contract address (`evm-contract`)

Grinds a **deployer key** whose first deployment (nonce 0, plain `CREATE`) lands on a matching contract address.

```bash
vanity-address --chain evm-contract --prefix cafe
vanity-address verify --chain evm-contract --address 0xcafe… --key <deployer-private-key>
```

Output: contract address + deployer private key + deployer address. Fund the deployer and make the deploy its **very first transaction** on that chain — any earlier transaction bumps the nonce and the address changes. Works on every EVM chain (same address on each, as long as nonce 0 is the deploy).

Use `--create2` instead when you deploy through a factory and already know its init code hash.

### Tron contract address (`trx-contract`)

Same CREATE / nonce-0 math as EVM, but the address is Tron’s base58 form (`T…`). The deployer key is still a 32-byte secp256k1 hex key (same as `--chain trx`).

```bash
vanity-address --chain trx-contract --prefix Cafe
vanity-address verify --chain trx-contract --address TCafe… --key <deployer-private-key>
```

### EVM CREATE2 salt (`--create2`)

Grinds a **salt** so `CREATE2(deployer, salt, initCodeHash)` matches your pattern. You already know the factory/deployer and the keccak256 of the init code.

```bash
vanity-address --create2 --deployer 0xYourFactory… --init-code-hash 0x… --prefix cafe
```

Output: contract address + salt (hex). Not a private key — use the salt in your factory call.

### Solana token mint (`sol-mint`)

Same ed25519 math as a wallet, exported for token creation:

```bash
vanity-address --chain sol-mint --suffix pump
# save the "Mint Keypair (JSON)" line as mint.json, then:
spl-token create-token mint.json            # or: spl-token create-token --program-2022 mint.json
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
vanity-address --chain evm --suffix face
vanity-address --chain evm --prefix dead
vanity-address --chain evm --prefix dead --suffix face
```

```bash
$ vanity-address --chain evm --prefix dead --suffix face

  Chain     EVM (Ethereum)
  Target    starting with '0xdead' and ending with 'face'
  Expected  ~1.1T attempts (average)
  ...
```

### Other chains

Use `--chain` with any supported ID:

Wallet menu (A–Z + Bitcoin-family forks): `ada`, `algo`, `aptos`, `btc`, `btg`, `btc-segwit`, `btc-taproot`, `cdn`, `cosmos`, `dash`, `dgb`, `doge`, `dot`, `dydx`, `emc2`, `erd`, `evm`, `fil`, `hedera`, `icp`, `inj`, `kaspa`, `ksm`, `ltc`, `mona`, `near`, `nvc`, `osmo`, `ppc`, `qtum`, `rvn`, `sei`, `sol`, `sui`, `sys`, `tia`, `ton`, `trx`, `via`, `vtc`, `xlm`, `xpm`, `xrp`, `xtz`

EVM aliases (same math as `evm`): `base`, `arb`, `op`, `polygon`, `robinhood`, `hyperliquid` / `hyperevm`, `sonic`, `monad`, …

Bitcoin-family P2PKH bytes and puzzle mode: [BITCOIN_FORKS.md](BITCOIN_FORKS.md).

### Bitcoin puzzle mode

```bash
vanity-address --puzzle 6 --prefix 1Love
vanity-address --puzzle 20 --puzzle-start 0000000000000000000000000000000000000000000000000000000000000000 --prefix 1
```

`--puzzle N` searches a `2^N`-sized private-key space (high bits fixed, usually zero). BTC P2PKH only for now.

### Split-key (Bitcoin-family P2PKH)

Alice keeps a secret and shares only a public key. Bob grinds a **part**; Alice combines offline into the final spending key.

```bash
# 1) Alice generates (share only public_key_hex; --json before the subcommand)
vanity-address split-key gen --chain btc
vanity-address --json split-key gen --chain btc

# 2) Bob grinds against Alice's pubkey (-P)
vanity-address --chain btc -P 04… --prefix Love

# 3) Alice combines her secret + Bob's PrivkeyPart
vanity-address split-key combine --alice <hex-or-wif> --part <hex-or-wif> --chain btc
```

Works with any registered Bitcoin-family `--chain` (`btc`, `ltc`, `doge`, `dash`, `rvn`, …). Mutually exclusive with `--puzzle` / `--create2`.

Threat model / safety habits: [SECURITY-MODEL.md](SECURITY-MODEL.md).

### OpenCL GPU (`--gpu`)

**EVM** is GPU-accelerated on the **CLI and desktop app** (~90× the CPU on an Apple M4; other GPUs untested). **Solana** GPU is experimental and currently slower than CPU, so `--gpu auto` only picks the GPU for EVM — use `--gpu require` to force Solana. The web generator and npm WASM SDK stay CPU-only (no GPU, puzzle, or split-key). Matches are always re-derived on CPU before export.

```bash
vanity-address gpu devices
vanity-address gpu bench --chain evm --seconds 2
vanity-address --chain evm --prefix cafe --gpu auto
vanity-address --chain evm --prefix cafe --gpu require   # error if no OpenCL device
```

Benchmarks: [BENCHMARKS.md](BENCHMARKS.md).

---

## JSON output (automation)

Use `--json` with `--prefix` and/or `--suffix` for machine-readable stdout (errors go to stderr as JSON):

```bash
vanity-address --chain sol --suffix ax --json --no-benchmark --force
```

Example success payload:

```json
{
  "version": "0.8.0",
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

| Flag                 | Description                                                                              | Default              |
| -------------------- | ---------------------------------------------------------------------------------------- | -------------------- |
| `--chain <ID>`       | Blockchain, alias (`base`, `hyperliquid`, …) or deploy mode (`evm-contract`, `trx-contract`, `sol-mint`) | `sol`                |
| `--prefix <PATTERN>` | Address must start with pattern (`*` OK)                                                 | —                    |
| `--suffix <PATTERN>` | Address must end with pattern (`*` OK; comma = OR)                                       | —                    |
| `--contains <PAT>`   | Substring anywhere (`*` wildcards OK)                                                    | —                    |
| `--count <N>`        | Find N matches                                                                           | `1`                  |
| `--exact`            | Exact casing (base58 chains)                                                             | off                  |
| `--create2`          | EVM CREATE2 salt grind (needs deployer + hash)                                           | off                  |
| `--deployer <HEX>`   | CREATE2 deployer (20 bytes)                                                              | —                    |
| `--init-code-hash`   | CREATE2 init code hash (32 bytes)                                                        | —                    |
| `--puzzle <BITS>`    | Bitcoin puzzle mode (free low bits; BTC P2PKH)                                           | —                    |
| `--puzzle-start`     | Puzzle high-bit template (32-byte hex)                                                   | zeros                |
| `-P, --split-pubkey` | Split-key grind against Alice's secp256k1 pubkey hex (Bitcoin-family)                    | —                    |
| `--save`             | Append match (incl. **plaintext** private keys) to file; `0600` on macOS/Linux           | off                  |
| `--output <PATH>`    | Custom save file (with `--save` or interactive)                                          | `vanity-results.txt` |
| `--no-benchmark`     | Skip 2s speed calibration warm-up                                                        | off                  |
| `--force`            | Allow impractical patterns in CLI mode                                                   | off                  |
| `--json`             | Machine-readable JSON on stdout (direct mode)                                            | off                  |
| `-q, --quiet`        | Minimal plain-text output (script-friendly)                                              | off                  |
| `--threads <N>`      | Override worker threads                                                                  | auto                 |
| `--preset <ID>`      | Power profile: `eco` · `balanced` · `fast` · `extreme` (aliases: efficient, best, superman). Ignored if `--threads` is set | `balanced` |
| `verify`             | Subcommand: check key derives address                                                    | —                    |
| `split-key gen`      | Alice half: keep secret, share pubkey                                                    | —                    |
| `split-key combine`  | Alice secret + Bob part → final key                                                      | —                    |
| `--gpu <MODE>`       | OpenCL GPU: `off` · `auto` (EVM) · `require` (EVM + Solana, Solana experimental)| `off`                |
| `gpu devices`        | List OpenCL GPUs                                                                         | —                    |
| `gpu bench`          | Short OpenCL throughput probe (`--chain evm\|sol`)                                       | —                    |
| `-h, --help`         | Show help                                                                                | —                    |
| `-V, --version`      | Show version                                                                             | —                    |

### Pattern rules

- **Contains / wildcards:** `--contains cafe` or `Cool*xyz` — `*` = any chars
- **Multi-pattern OR:** `--suffix moon,pump,dao` (comma only in one of prefix/suffix/contains)
- **Base58 chains** (Solana, Bitcoin P2PKH, Litecoin, Dogecoin, Dash, Tron, Ripple, Tezos, Polkadot, Kusama): no `0`, `O`, `I`, `l` (where applicable); Ripple uses its own alphabet
- **Bech32** (Cosmos, Osmosis, Sei, Injective, Celestia, dYdX, Kaspa, Cardano, MultiversX, BTC SegWit/Taproot): charset `qpzry9x8gf2tvdw0s3jn54khce6mua7l`
- **Base32** (Algorand, Stellar, Filecoin, ICP): Algorand / Stellar uppercase; Filecoin `f1…`; ICP principals (dashes optional in pattern)
- **Base64url** (TON): `UQ…` Wallet V4R2 non-bounceable
- **Fixed address starts are added for you** and don't count toward difficulty — type only your part (`--chain doge --prefix Moon` → `DMoon…`)
- **Impossible prefixes are rejected up front** with the characters that can appear there. Some chains only allow a few characters right after the fixed start:

| Chain    | Every address starts | Next character              |
| -------- | -------------------- | --------------------------- |
| Kaspa    | `kaspa:q`            | `p` `q` `r` `z`             |
| Cardano  | `addr1v`             | `8` `9` `x` `y`             |
| TON      | `UQ`                 | `A`–`D`                     |
| Stellar  | `G`                  | `A`–`D`                     |
| Dogecoin | `D`                  | `5`–`9`, `A`–`U`            |
| Litecoin | `L`                  | `K`–`Z`, `a`–`i`            |
| Dash     | `X`                  | `a`–`z`                     |
| Tron     | `T`                  | `9`, `A`–`Z`                |
| Tezos    | `tz1`                | `K`–`Z`, `a`–`i`            |
| Kusama   | —                    | `C` `D` `E` `F` `G` `H` `J` |

Bitcoin (`1`), Ripple (`r`), Polkadot (`1`) and Solana allow almost any next character, but some are much rarer than others — the estimate accounts for that.

- **Hex chains** (EVM + aliases, Aptos, Sui, NEAR, Hedera pubkey, CREATE2): `0-9`, `a-f`; optional `0x`

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
vanity-address --chain evm --prefix cafe --preset eco
vanity-address --chain sol --suffix pump --preset extreme
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
