# vanity-app

Desktop UI for [vanity-address](../README.md), built with **Tauri 2** + vanilla TypeScript.

## Flow (matches CLI)

1. **Home** — Start / Help
2. **Chain** — pick from 13 supported chains
3. **Pattern** — suffix / prefix / both tabs, live estimate + warnings
4. **Summary** — system profile, difficulty, confirm (double-confirm for impractical)
5. **Grind** — 2s benchmark, live progress, stop anytime
6. **Result** — highlighted address, blurred keys, copy/save, grind another or new grind

## Requirements

- [Rust](https://rustup.rs/) 1.70+
- [Node.js](https://nodejs.org/) 18+

## Development

```bash
cd vanity-app
npm install
npm run tauri dev
```

## Production build

```bash
npm run tauri build
```

The `.app` / `.dmg` (macOS) or `.deb` / `.AppImage` (Linux) lands in `src-tauri/target/release/bundle/`.

### App icons

UI uses `assets/logo.svg` (also copied to `public/logo.svg`). To refresh dock/menu icons after a logo change:

```bash
rsvg-convert -w 1024 -h 1024 ../assets/logo.svg -o ../assets/logo-icon.png
npx tauri icon ../assets/logo-icon.png -o src-tauri/icons
cp ../assets/logo.svg public/logo.svg
```

## Architecture

| Layer | Role |
|-------|------|
| `src/` | Wizard UI (TypeScript) |
| `src-tauri/src/commands.rs` | Tauri commands → `vanity-core` |
| `vanity-core` | Grinding, estimates, chain logic (shared with CLI) |

All key generation runs in the Rust backend on your machine — no network calls.
