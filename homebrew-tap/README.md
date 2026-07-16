# homebrew-tap

Homebrew tap for [vanity-address](https://github.com/yudizaxay/vanity-address).

## Install

```bash
brew tap yudizaxay/tap
brew trust yudizaxay/tap
brew install vanity-address
```

**Homebrew 6+:** third-party taps must be trusted once. If install fails with *untrusted tap*, run `brew trust yudizaxay/tap` (or `brew trust --formula yudizaxay/tap/vanity-address`).

Builds the CLI from source (requires Rust). First compile often takes 3–8 minutes because of the Solana SDK dependency tree.

## Upgrade

```bash
brew update
brew upgrade vanity-address
```

## Uninstall

```bash
brew uninstall vanity-address
brew untap yudizaxay/tap   # optional
```

## Maintainer sync

The canonical formula lives in the main repo at `Formula/vanity-address.rb`. After each release:

```bash
# In vanity-address repo
./scripts/update-homebrew-formula.sh 0.3.5
./scripts/sync-homebrew-tap.sh --push "vanity-address 0.3.5"
```

See [docs/HOMEBREW.md](https://github.com/yudizaxay/vanity-address/blob/main/docs/HOMEBREW.md) in the main repository.
