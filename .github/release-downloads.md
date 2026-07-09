## Quick download — pick one file

| I want… | My computer | Download |
| ------- | ----------- | -------- |
| **Desktop app** (window UI) | Mac M1 / M2 / M3 / M4 | **VanityAddress-*-Mac-AppleSilicon-Desktop.dmg** |
| **Terminal app** | Mac M1 / M2 / M3 / M4 | **VanityAddress-*-Mac-AppleSilicon-CLI.tar.gz** |
| **Terminal app** | Mac Intel | **VanityAddress-*-Mac-Intel-CLI.tar.gz** |
| **Terminal app** | Windows 10/11 | **VanityAddress-*-Windows-CLI.zip** |
| **Terminal app** | Linux | **VanityAddress-*-Linux-CLI.tar.gz** |

> **Not sure?** Mac with Apple chip → download the **Desktop .dmg** for the easiest start.  
> Each file has a matching `.sha256` checksum (optional, for security verification).

### After downloading

- **`.dmg`** — run `xattr -cr` on the file (see below) → open → drag **Vanity Address** to Applications → `xattr -cr` on the app → launch
- **`.zip` / `.tar.gz`** — extract → run `vanity-address` (or `vanity-address.exe` on Windows)

### macOS: “app is damaged and can’t be opened”

GitHub releases are **not Apple-notarized** (normal for free open-source apps). macOS may show **damaged** — fix:

```bash
xattr -cr ~/Downloads/VanityAddress-*-Mac-AppleSilicon-Desktop.dmg
# after installing to Applications:
xattr -cr "/Applications/Vanity Address.app"
open -a "Vanity Address"
```

Or **right-click → Open** on the app (first launch only).

Full step-by-step guide: [README — Install](https://github.com/yudizaxay/vanity-address#-install)

---
