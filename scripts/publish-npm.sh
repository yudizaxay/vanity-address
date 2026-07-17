#!/usr/bin/env bash
# Publish platform packages then main vanity-address to npm.
# Usage:
#   ./scripts/prepare-npm.sh 0.3.5
#   ./scripts/publish-npm.sh            # dry-run first? use --dry-run
#   ./scripts/publish-npm.sh --dry-run
#
# Requires: npm login (or NPM_TOKEN), binaries prepared via prepare-npm.sh

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DRY=()
if [[ "${1:-}" == "--dry-run" ]]; then
  DRY=(--dry-run)
fi

PLATFORMS=(
  vanity-address-darwin-arm64
  vanity-address-darwin-x64
  vanity-address-linux-x64
  vanity-address-win32-x64
)

require_bin() {
  local dir="$1" name="$2"
  local path="${ROOT}/npm/${dir}/bin/${name}"
  if [[ ! -f "${path}" ]]; then
    echo "Missing ${path} — run ./scripts/prepare-npm.sh first" >&2
    exit 1
  fi
}

require_bin vanity-address-darwin-arm64 vanity-address
require_bin vanity-address-darwin-x64 vanity-address
require_bin vanity-address-linux-x64 vanity-address
require_bin vanity-address-win32-x64 vanity-address.exe

for dir in "${PLATFORMS[@]}"; do
  echo "==> Publishing ${dir}"
  (cd "${ROOT}/npm/${dir}" && npm publish --access public "${DRY[@]}")
done

echo "==> Publishing vanity-address (main)"
(cd "${ROOT}/npm/vanity-address" && npm publish --access public "${DRY[@]}")

echo "Done."
