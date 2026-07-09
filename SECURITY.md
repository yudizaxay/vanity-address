# Security Policy

**vanity-address** generates cryptocurrency private keys on your machine. Treat this tool and its output with the same care as any wallet software.

## Supported versions

| Version | Supported |
| ------- | --------- |
| 0.3.x   | ✅        |
| 0.2.x   | ✅        |
| < 0.2   | ❌        |

## Reporting a vulnerability

**Do not** open a public GitHub issue for security-sensitive reports.

1. Email or DM the maintainer via [GitHub profile](https://github.com/yudizaxay) with **“vanity-address security”** in the subject.
2. Include steps to reproduce, affected version, and impact assessment.
3. Allow up to **7 days** for an initial response.

We will coordinate disclosure and credit reporters who wish to be named.

## Design principles

| Principle | Detail |
| --------- | ------ |
| **Local only** | No network calls during key generation, benchmarking, or grinding |
| **No telemetry** | CLI and desktop app do not phone home |
| **Open source** | Audit `vanity-core` grinding and chain finalize logic before trusting keys |
| **User responsibility** | Vanity grinding is probabilistic; verify addresses before sending funds |

## Private key handling

- **Never commit** `vanity-results.txt`, `*.keys.txt`, or any file containing private keys.
- These paths are listed in `.gitignore` — do not remove them.
- The `--save` / `--output` flags and the desktop **Save** dialog write **plaintext** private keys to disk. Encrypt backups and restrict file permissions.
- Use `--json` only in trusted environments; stdout may contain private key material.

## Threat model (out of scope)

The following are **not** goals of this project:

- HSM / hardware wallet integration
- Encrypted at-rest key storage
- Protection against malware on the host machine
- Side-channel resistance against physical attackers

If you need those guarantees, use established wallet infrastructure after generating keys offline in an air-gapped environment.

## Dependency updates

Rust and npm dependencies are monitored via [Dependabot](.github/dependabot.yml). Release binaries are built from tagged commits in [GitHub Actions](.github/workflows/release.yml).

## Checksums

Official CLI releases ship with `.sha256` checksum files. Verify downloads before use:

```bash
shasum -a 256 -c VanityAddress-*-Linux-CLI.tar.gz.sha256
```
