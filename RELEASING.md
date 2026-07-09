# Releasing

Maintainers cut releases by pushing a version tag. GitHub Actions builds binaries and publishes a GitHub Release automatically.

## Checklist

1. Ensure `main` is green on CI (`make check-ci` or see README)
2. Bump version in **all** packages (keep in sync):
   - `vanity-core/Cargo.toml`
   - `vanity-address/Cargo.toml`
   - `vanity-app/package.json`
   - `vanity-app/src-tauri/Cargo.toml`
   - `vanity-app/src-tauri/tauri.conf.json`
   - `vanity-address/src/banner.rs`, `vanity-app/index.html`, demo SVGs
3. Update `CHANGELOG.md` with the new version section
4. Update `Formula/vanity-address.rb` `url` and `sha256` (see below)
5. Commit, push to `main`
6. Create and push the tag:

```bash
git tag -a v0.3.0 -m "v0.3.0"
git push origin v0.3.0
```

7. Watch the [Release workflow](https://github.com/yudizaxay/vanity-address/actions/workflows/release.yml)
8. Verify assets on the [Releases](https://github.com/yudizaxay/vanity-address/releases) page

## Release assets

| Asset on GitHub | Who should download |
| --------------- | ------------------- |
| `VanityAddress-<ver>-Mac-AppleSilicon-Desktop.dmg` | Mac M1–M4 — desktop app |
| `VanityAddress-<ver>-Mac-AppleSilicon-CLI.tar.gz` | Mac M1–M4 — terminal |
| `VanityAddress-<ver>-Mac-Intel-CLI.tar.gz` | Intel Mac — terminal |
| `VanityAddress-<ver>-Windows-CLI.zip` | Windows — terminal |
| `VanityAddress-<ver>-Linux-CLI.tar.gz` | Linux — terminal |

Each binary has a matching `.sha256` checksum file (optional).

## Archive contents (CLI)

- `vanity-address` binary (or `vanity-address.exe` on Windows)
- `README.md`
- `LICENSE`
- `SECURITY.md`

## Homebrew formula

After tagging, update the formula tarball hash:

```bash
curl -L "https://github.com/yudizaxay/vanity-address/archive/refs/tags/v0.3.0.tar.gz" | shasum -a 256
```

Paste the hash into `Formula/vanity-address.rb` and commit (or include in the release commit).

Install locally:

```bash
brew install --build-from-source ./Formula/vanity-address.rb
```

## Publishing to crates.io

Publish **`vanity-core` first**, then **`vanity-address`**:

```bash
cargo publish -p vanity-core
cargo publish -p vanity-address
```

Users can then install with:

```bash
cargo install vanity-address
```

## Notes

- Tags must match `v*` (e.g. `v0.3.0`) to trigger the workflow
- **CI green ≠ Release published** — push to `main` only runs CI; Release runs on **tag push** or manual dispatch
- **Manual release (no tag push):** Actions → **Release** → **Run workflow** → enter version `0.3.0` → Run
- After CI fixes, **move the tag** to the latest commit or run workflow manually from `main`:
  ```bash
  git tag -fa v0.3.0 -m "v0.3.0"
  git push origin v0.3.0 --force   # requires yudizaxay account
  ```
- Desktop `.dmg` is built on macOS arm64 runners only
- Intel macOS CLI is cross-compiled on `macos-latest` with `x86_64-apple-darwin`
- Linux builds target `x86_64-unknown-linux-gnu` (glibc)
