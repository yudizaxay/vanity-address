# Install guide

Full platform-by-platform instructions for **vanity-address** CLI and the **Vanity Address** desktop app.

**Latest release:** [github.com/yudizaxay/vanity-address/releases/latest](https://github.com/yudizaxay/vanity-address/releases/latest)

No Rust or Node.js required for pre-built downloads — pick your file, extract, and run.

---

## Which file do I need?

| I want… | My computer | File to download |
| ------- | ----------- | ---------------- |
| **Desktop app** (easiest — window UI) | Mac M1 / M2 / M3 / M4 | `VanityAddress-*-Mac-AppleSilicon-Desktop.dmg` |
| **Desktop app** (easiest — window UI) | Windows 10 / 11 | `VanityAddress-*-Windows-Desktop.exe` |
| **Terminal app** | Mac M1 / M2 / M3 / M4 | `VanityAddress-*-Mac-AppleSilicon-CLI.tar.gz` |
| **Terminal app** | Mac Intel | `VanityAddress-*-Mac-Intel-CLI.tar.gz` |
| **Terminal app** | Windows 10 / 11 | `VanityAddress-*-Windows-CLI.zip` |
| **Terminal app** | Linux | `VanityAddress-*-Linux-CLI.tar.gz` |

> **Tip:** `*` = version number (e.g. `0.3.2`). Prefer [latest release](https://github.com/yudizaxay/vanity-address/releases/latest) for the newest filenames.  
> **Checksum files** (`.sha256`) are optional — for security verification only; most users can skip them.

---

## Linux (CLI)

```bash
# Replace 0.3.2 with the version on the Releases page if newer
curl -LO https://github.com/yudizaxay/vanity-address/releases/download/v0.3.2/VanityAddress-0.3.2-Linux-CLI.tar.gz
tar xzf VanityAddress-0.3.2-Linux-CLI.tar.gz
./vanity-address
```

Optional — install globally:

```bash
sudo cp vanity-address /usr/local/bin/
vanity-address --version
```

---

## macOS — CLI (Terminal)

**Apple Silicon (M1/M2/M3/M4):**

```bash
curl -LO https://github.com/yudizaxay/vanity-address/releases/download/v0.3.2/VanityAddress-0.3.2-Mac-AppleSilicon-CLI.tar.gz
tar xzf VanityAddress-0.3.2-Mac-AppleSilicon-CLI.tar.gz
./vanity-address
```

**Intel Mac:** download `VanityAddress-*-Mac-Intel-CLI.tar.gz` instead.

If macOS blocks the binary (“unidentified developer” or **“damaged”**), see [macOS Gatekeeper](#macos-gatekeeper-damaged-app) below.

---

## macOS — Desktop app (`.dmg`)

Recommended for Mac users who prefer a graphical wizard.

1. Download **`VanityAddress-*-Mac-AppleSilicon-Desktop.dmg`** from [Releases](https://github.com/yudizaxay/vanity-address/releases/latest)
2. **Remove download quarantine** (required for unsigned open-source builds):
   ```bash
   xattr -cr ~/Downloads/VanityAddress-*-Mac-AppleSilicon-Desktop.dmg
   ```
3. Double-click the `.dmg` → drag **Vanity Address** to **Applications**
4. Clear quarantine on the installed app, then launch:
   ```bash
   xattr -cr "/Applications/Vanity Address.app"
   open -a "Vanity Address"
   ```

> **Apple Silicon Macs only** (M1/M2/M3/M4). Intel Mac → use the **CLI**. Windows → use the [Windows desktop installer](#windows--desktop-app-installer) or CLI. Linux → use the **CLI** or [build from source](#build-from-source).

---

## Windows — Desktop app (installer)

Recommended for Windows users who prefer a graphical wizard.

1. Download **`VanityAddress-*-Windows-Desktop.exe`** from [Releases](https://github.com/yudizaxay/vanity-address/releases/latest)
2. Double-click the installer
3. If **Windows SmartScreen** appears → **More info → Run anyway** (unsigned open-source build)
4. Finish the installer, then launch **Vanity Address** from the Start menu

> Requires **WebView2** (included on Windows 11; the installer can download it on Windows 10 if missing).

---

## Windows (CLI)

1. Download **`VanityAddress-*-Windows-CLI.zip`** from [Releases](https://github.com/yudizaxay/vanity-address/releases/latest)
2. Right-click → **Extract All**
3. Open the extracted folder and double-click **`vanity-address.exe`**, or in PowerShell:

```powershell
.\vanity-address.exe
```

4. Optional — add the folder to your PATH for terminal access from anywhere

Windows SmartScreen may warn on first run — click **More info → Run anyway** (unsigned open-source build).

---

## macOS Gatekeeper (“damaged” app)

GitHub releases are **not Apple-notarized** (normal for free open-source apps). macOS may show **“Vanity Address is damaged and can’t be opened”** — this is **not** a broken download.

**Fix — CLI binary:**

```bash
xattr -cr ./vanity-address
./vanity-address
```

**Fix — Desktop app:** run the `xattr -cr` commands in the [Desktop app](#macos--desktop-app-dmg) section above.

**Alternatives:**

- **Right-click** the app → **Open** → **Open** again (first launch only)
- **System Settings → Privacy & Security → Open Anyway** after the first blocked launch

---

## Verify downloads (checksums)

Every archive on the Releases page ships with a `.sha256` sidecar file.

```bash
# Linux / macOS (optional)
shasum -a 256 -c VanityAddress-0.3.2-Linux-CLI.tar.gz.sha256
```

```powershell
# Windows (optional) — compare hash manually
Get-FileHash VanityAddress-0.3.2-Windows-CLI.zip -Algorithm SHA256
```

---

## What's inside each archive?

| Archive | Contents |
| ------- | -------- |
| `*-CLI.tar.gz` / `*-CLI.zip` | `vanity-address` binary + docs |
| `*-Desktop.dmg` | Mac desktop app installer (double-click to install) |
| `*-Windows-Desktop.exe` | Windows desktop NSIS installer (double-click to install) |

**First run:** just execute the binary — interactive menu starts with no flags. See [Usage guide](USAGE.md).

---

## Other install methods

### Homebrew (macOS / Linux)

```bash
brew install --build-from-source ./Formula/vanity-address.rb
```

See [RELEASING.md](../RELEASING.md) for tap setup and formula hash updates.

### crates.io (`cargo install`)

```bash
cargo install vanity-address
```

Requires [Rust](https://rustup.rs/) 1.70+. Installs the latest published CLI from [crates.io/crates/vanity-address](https://crates.io/crates/vanity-address).

**First install is slow (often 3–8 minutes)** — normal for this project. Cargo compiles from source, including the large Solana SDK dependency tree. The second install on the same machine is faster thanks to Cargo’s cache.

**Faster options (no compile):**

| Method | Typical time |
| ------ | ------------ |
| [GitHub Releases](https://github.com/yudizaxay/vanity-address/releases/latest) pre-built CLI | ~30 seconds (download + extract) |
| `cargo install` (first time) | ~3–8 minutes |
| `cargo install` (same machine again) | ~1–2 minutes |

**Speed up `cargo install` on an already-published version** (optional env vars, no code change):

```bash
CARGO_PROFILE_RELEASE_LTO=false CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 cargo install vanity-address
```

Use all CPU cores (default on most setups):

```bash
cargo install vanity-address -j "$(nproc 2>/dev/null || sysctl -n hw.ncpu)"
```

Maintainers: see [RELEASING.md](../RELEASING.md#publishing-to-cratesio).

### Uninstall

**CLI installed via `cargo install`:**

```bash
cargo uninstall vanity-address
```

Removes `~/.cargo/bin/vanity-address`. You do **not** need to uninstall `vanity-core` — it is a library dependency, not a global command.

**Desktop app:**

| Platform | How to remove |
| -------- | ------------- |
| macOS | Drag **Vanity Address** from Applications to Trash |
| Windows | **Settings → Apps → Vanity Address → Uninstall** (or the installer’s uninstall entry) |

**Optional cleanup** (saved grind results, not the app itself):

```bash
rm -f vanity-results.txt   # default save file from `--save` / interactive save
```

### Build from source

**Requirements:** [Rust](https://rustup.rs/) 1.70+

```bash
git clone https://github.com/yudizaxay/vanity-address.git
cd vanity-address
cargo build --release
```

Binary: `target/release/vanity-address`

**Install globally:**

```bash
cargo install --path vanity-address
```

**Desktop app from source** also needs [Node.js](https://nodejs.org/) 18+:

```bash
cd vanity-app
npm install
npm run tauri dev      # development
npm run tauri build    # native .app / .dmg (macOS)
```
