#!/usr/bin/env bash
# Copy Formula/ into a local homebrew-tap repo and optionally commit + push.
#
# One-time setup (create empty GitHub repo yudizaxay/homebrew-tap first):
#   git clone https://github.com/yudizaxay/homebrew-tap.git ../homebrew-tap
#
# Usage:
#   ./scripts/sync-homebrew-tap.sh                    # copy only
#   ./scripts/sync-homebrew-tap.sh --push "v0.3.5"   # copy, commit, push

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TAP_DIR="${HOMEBREW_TAP_DIR:-${ROOT}/../homebrew-tap}"
PUSH=false
COMMIT_MSG=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --push)
      PUSH=true
      COMMIT_MSG="${2:-Update vanity-address formula}"
      shift 2
      ;;
    *)
      echo "Usage: $0 [--push \"commit message\"]" >&2
      exit 1
      ;;
  esac
done

mkdir -p "${TAP_DIR}/Formula"
cp "${ROOT}/Formula/vanity-address.rb" "${TAP_DIR}/Formula/vanity-address.rb"

if [[ ! -f "${TAP_DIR}/README.md" ]]; then
  cp "${ROOT}/homebrew-tap/README.md" "${TAP_DIR}/README.md"
fi

echo "Synced formula to ${TAP_DIR}/Formula/vanity-address.rb"

if [[ "${PUSH}" != true ]]; then
  echo "Next: cd ${TAP_DIR} && git add Formula && git commit -m '...' && git push"
  exit 0
fi

if [[ ! -d "${TAP_DIR}/.git" ]]; then
  echo "Not a git repo: ${TAP_DIR}" >&2
  echo "Clone first: git clone https://github.com/yudizaxay/homebrew-tap.git ${TAP_DIR}" >&2
  exit 1
fi

cd "${TAP_DIR}"
git add Formula/vanity-address.rb README.md 2>/dev/null || git add Formula/vanity-address.rb
if git diff --staged --quiet; then
  echo "No changes to commit."
  exit 0
fi
git commit -m "${COMMIT_MSG}"
git push origin main
echo "Pushed to https://github.com/yudizaxay/homebrew-tap"
