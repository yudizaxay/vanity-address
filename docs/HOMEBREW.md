# Homebrew distribution

Install the **CLI** on macOS and Linux via Homebrew. The desktop app (`.dmg` / `.exe`) is not distributed through Homebrew — use [GitHub Releases](https://github.com/yudizaxay/vanity-address/releases/latest).

## For users

### Option A — Tap (recommended)

```bash
brew tap yudizaxay/tap
brew trust yudizaxay/tap
brew install vanity-address
vanity-address --version
```

On **Homebrew 6+**, third-party taps must be trusted once before install (security). Trust only this formula instead of the whole tap:

```bash
brew trust --formula yudizaxay/tap/vanity-address
```

### Untrusted tap error

If you see:

```text
Error: Refusing to load formula yudizaxay/tap/vanity-address from untrusted tap yudizaxay/tap.
Run `brew trust --formula yudizaxay/tap/vanity-address` or `brew trust yudizaxay/tap` to trust it.
```

Run one of the `brew trust` commands above, then `brew install vanity-address` again. See [Homebrew tap trust](https://docs.brew.sh/Tap-Trust).

### Other untrusted taps block install (not vanity-address)

Homebrew 6 may refuse **any** `brew install` if you have **other** untrusted taps with formulae already on your Mac. Example:

```text
Error: Refusing to load formula mongodb/brew/mongodb-database-tools from untrusted tap mongodb/brew.
```

`vanity-address` does **not** depend on MongoDB — this is your local Homebrew setup.

**Fix A — trust the tap that Homebrew names in the error** (if you still use that software):

```bash
brew trust mongodb/brew
brew install vanity-address
```

**Fix B — trust only that formula:**

```bash
brew trust --formula mongodb/brew/mongodb-database-tools
brew install vanity-address
```

**Fix C — fully qualified install** (after `brew trust yudizaxay/tap`):

```bash
brew install yudizaxay/tap/vanity-address
```

If it still fails, trust or untap the other taps listed in the warning (`antoniorodr/memo`, `cloudflare/cloudflare`, `ngrok/ngrok`, etc.) — only for taps you actually use.

**Fix D — skip Homebrew compile** — use a [pre-built CLI from GitHub Releases](INSTALL.md#github-releases-recommended) (~30s, no Rust/MongoDB tap issues).


```bash
git clone https://github.com/yudizaxay/vanity-address.git
cd vanity-address
brew install --build-from-source ./Formula/vanity-address.rb
```

### Option C — Latest `main` (bleeding edge)

```bash
brew install --HEAD yudizaxay/tap/vanity-address
# or from local formula with `head` in Formula/vanity-address.rb:
brew install --HEAD --build-from-source ./Formula/vanity-address.rb
```

### Notes

| Topic | Detail |
| ----- | ------ |
| **Tap trust (Homebrew 6+)** | Run `brew trust yudizaxay/tap` once before first install |
| **Compile time** | 3–8 min first install (Solana SDK); normal for this project |
| **Rust** | Formula depends on `rust` as a build dependency |
| **Faster install** | Use [pre-built CLI binaries](INSTALL.md#github-releases-recommended) (~30s download) |
| **Uninstall** | `brew uninstall vanity-address` |

---

## For maintainers

### Architecture

```text
vanity-address repo                    yudizaxay/homebrew-tap repo
├── Formula/vanity-address.rb  ──sync──► Formula/vanity-address.rb
└── scripts/
    ├── update-homebrew-formula.sh
    └── sync-homebrew-tap.sh
```

Homebrew tap naming: repo `homebrew-tap` under user `yudizaxay` → `brew tap yudizaxay/tap`.

### One-time tap setup

1. Tap repo: **https://github.com/yudizaxay/homebrew-tap** (created)
2. Clone it next to this repo:

```bash
git clone https://github.com/yudizaxay/homebrew-tap.git ../homebrew-tap
```

3. Initial sync:

```bash
./scripts/sync-homebrew-tap.sh --push "Initial vanity-address formula"
```

4. Tell users:

```bash
brew tap yudizaxay/tap
brew trust yudizaxay/tap
brew install vanity-address
```

### Every release

After pushing tag `vX.Y.Z` to `vanity-address` (**tag must exist on GitHub** — `curl -fsSL` will fail otherwise):

```bash
# 1. Update url + sha256 in Formula/vanity-address.rb
./scripts/update-homebrew-formula.sh X.Y.Z

# 2. Commit in vanity-address repo
git add Formula/vanity-address.rb
git commit -m "chore: Homebrew formula vX.Y.Z"

# 3. Push formula to tap repo
./scripts/sync-homebrew-tap.sh --push "vanity-address X.Y.Z"
```

Override tap location: `HOMEBREW_TAP_DIR=/path/to/homebrew-tap ./scripts/sync-homebrew-tap.sh`

### Verify locally

```bash
brew install --build-from-source ./Formula/vanity-address.rb
brew test vanity-address   # if installed via tap
vanity-address --version
```

### Submitting to homebrew-core (optional, later)

Official [homebrew-core](https://github.com/Homebrew/homebrew-core) requires notability (stars, usage). Start with a personal tap; consider core submission after the project grows.

---

## Related docs

- [INSTALL.md](INSTALL.md) — all install methods
- [RELEASING.md](../RELEASING.md) — full release checklist
- [AGENTS.md](../AGENTS.md) — project context for contributors and AI sessions
