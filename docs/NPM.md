# npm distribution (CLI)

Publish the **vanity-address** CLI on [npmjs.com](https://www.npmjs.com) so users can run:

```bash
npx vanity-address
npm install -g vanity-address
```

This is a **binary wrapper** (not a JS library). It does not compile Rust. Platform packages ship the same CLI binaries as [GitHub Releases](https://github.com/yudizaxay/vanity-address/releases).

`vanity-app/` stays `private` — desktop app is **not** published to npm.

---

## For users

```bash
npx vanity-address
npx vanity-address --chain sol --suffix axay
npm install -g vanity-address
vanity-address --version
```

Requires **Node.js 18+**. Supported platforms:

| npm package | Platform |
| ----------- | -------- |
| `vanity-address-darwin-arm64` | macOS Apple Silicon |
| `vanity-address-darwin-x64` | macOS Intel |
| `vanity-address-linux-x64` | Linux x86_64 |
| `vanity-address-win32-x64` | Windows x64 |

---

## Package layout

```text
npm/
├── vanity-address/                 # main — bin shim + optionalDependencies
├── vanity-address-darwin-arm64/
├── vanity-address-darwin-x64/
├── vanity-address-linux-x64/
└── vanity-address-win32-x64/
```

Main package `bin/cli.js` resolves the matching optional package and `spawn`s the native binary. **No `postinstall` script.**

Native binaries are **not** committed — filled by `scripts/prepare-npm.sh`.

---

## For maintainers

### One-time

```bash
npm login
# or: export NPM_TOKEN=...
```

### Every release (after GitHub Release assets exist for tag `vX.Y.Z`)

1. Bump versions in sync (Cargo, app, **and** `npm/*/package.json` — or let prepare script set npm versions):

```bash
./scripts/prepare-npm.sh X.Y.Z
```

2. Dry-run:

```bash
./scripts/publish-npm.sh --dry-run
```

3. Publish (platform packages first, then main):

```bash
./scripts/publish-npm.sh
```

### Local smoke test (after prepare)

```bash
./scripts/prepare-npm.sh 0.3.7
cd npm/vanity-address
npm pack
# On matching platform, link platform package:
npm install ../vanity-address-darwin-arm64   # example
node bin/cli.js --version
```

Or:

```bash
npm install -g ./npm/vanity-address
# with platform package installed as optional dep from local paths during test
```

---

## Release checklist add-on

After step “GitHub Release assets verified”:

10. `./scripts/prepare-npm.sh X.Y.Z`
11. `./scripts/publish-npm.sh`

Keep npm version = CLI version (`0.3.7`).

---

## Related

- [INSTALL.md](INSTALL.md)
- [RELEASING.md](../RELEASING.md)
- [AGENTS.md](../AGENTS.md)
