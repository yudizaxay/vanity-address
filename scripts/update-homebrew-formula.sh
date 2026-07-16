#!/usr/bin/env bash
# Update Formula/vanity-address.rb url + sha256 for a tagged release.
# Usage: ./scripts/update-homebrew-formula.sh 0.3.5

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FORMULA="${ROOT}/Formula/vanity-address.rb"
VERSION="${1:?Usage: $0 <version> e.g. 0.3.5}"
TAG="v${VERSION}"
URL="https://github.com/yudizaxay/vanity-address/archive/refs/tags/${TAG}.tar.gz"

echo "Fetching ${URL} ..."
SHA256="$(curl -fsSL "${URL}" | shasum -a 256 | awk '{print $1}')" || {
  echo "Failed to download ${URL} — is tag v${VERSION} pushed to GitHub?" >&2
  exit 1
}
echo "sha256: ${SHA256}"

if [[ ! -f "${FORMULA}" ]]; then
  echo "Formula not found: ${FORMULA}" >&2
  exit 1
fi

# macOS sed needs '' for in-place; Linux does not.
if sed --version >/dev/null 2>&1; then
  sed -i "s|url \".*\"|url \"${URL}\"|" "${FORMULA}"
  sed -i "s|sha256 \".*\"|sha256 \"${SHA256}\"|" "${FORMULA}"
else
  sed -i '' "s|url \".*\"|url \"${URL}\"|" "${FORMULA}"
  sed -i '' "s|sha256 \".*\"|sha256 \"${SHA256}\"|" "${FORMULA}"
fi

echo "Updated ${FORMULA}"
grep -E '^\s+(url|sha256)' "${FORMULA}"
