# Releasing

Maintainers cut releases by pushing a version tag. GitHub Actions builds binaries and publishes a GitHub Release automatically.

## Checklist

1. Ensure `main` is green on CI (`cargo test`, `cargo clippy -- -D warnings`)
2. Bump version in `vanity-address/Cargo.toml` if needed
3. Update `CHANGELOG.md` with the new version section
4. Commit, push to `main`
5. Create and push the tag:

```bash
git tag -a v0.2.0 -m "v0.2.0"
git push origin v0.2.0
```

6. Watch the [Release workflow](https://github.com/yudizaxay/vanity-address/actions/workflows/release.yml)
7. Verify assets on the [Releases](https://github.com/yudizaxay/vanity-address/releases) page:
   - `vanity-address-<version>-linux-x86_64.tar.gz`
   - `vanity-address-<version>-macos-arm64.tar.gz`
   - matching `.sha256` checksum files

## Archive contents

Each tarball contains:

- `vanity-address` (release binary)
- `README.md`
- `LICENSE`

## Notes

- Tags must match `v*` (e.g. `v0.2.0`) to trigger the workflow
- Intel macOS runners are not available on GitHub Actions; Apple Silicon (`macos-arm64`) is the prebuilt macOS target
- Linux builds target `x86_64-unknown-linux-gnu` (glibc)
