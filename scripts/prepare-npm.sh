#!/usr/bin/env bash
# Download GitHub Release CLI archives into npm/*/bin for publishing.
# Usage:
#   ./scripts/prepare-npm.sh           # uses version from npm/vanity-address/package.json
#   ./scripts/prepare-npm.sh 0.3.5
# Env:
#   GITHUB_TOKEN / GH_TOKEN — optional, for higher rate limits

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MAIN_PKG="${ROOT}/npm/vanity-address/package.json"
VERSION="${1:-}"

if [[ -z "${VERSION}" ]]; then
  VERSION="$(node -p "require('${MAIN_PKG}').version")"
fi

TAG="v${VERSION}"
BASE="https://github.com/yudizaxay/vanity-address/releases/download/${TAG}"
TMP="$(mktemp -d)"
trap 'rm -rf "${TMP}"' EXIT

AUTH_ARGS=()
if [[ -n "${GH_TOKEN:-${GITHUB_TOKEN:-}}" ]]; then
  AUTH_ARGS=(-H "Authorization: Bearer ${GH_TOKEN:-${GITHUB_TOKEN}}")
fi

download() {
  local url="$1" out="$2"
  echo "Downloading ${url}"
  if ((${#AUTH_ARGS[@]})); then
    curl -fsSL "${AUTH_ARGS[@]}" -o "${out}" "${url}"
  else
    curl -fsSL -o "${out}" "${url}"
  fi
}

# package_dir | asset_label | archive_type | binary_name
PLATFORMS=(
  "vanity-address-darwin-arm64|Mac-AppleSilicon-CLI|tar|vanity-address"
  "vanity-address-darwin-x64|Mac-Intel-CLI|tar|vanity-address"
  "vanity-address-linux-x64|Linux-CLI|tar|vanity-address"
  "vanity-address-win32-x64|Windows-CLI|zip|vanity-address.exe"
)

set_version() {
  local dir="$1"
  local pkg="${ROOT}/npm/${dir}/package.json"
  node -e "
    const fs = require('fs');
    const p = process.argv[1];
    const v = process.argv[2];
    const j = JSON.parse(fs.readFileSync(p, 'utf8'));
    j.version = v;
    fs.writeFileSync(p, JSON.stringify(j, null, 2) + '\n');
  " "${pkg}" "${VERSION}"
}

# Keep main package + optionalDependencies in sync
node -e "
  const fs = require('fs');
  const p = process.argv[1];
  const v = process.argv[2];
  const j = JSON.parse(fs.readFileSync(p, 'utf8'));
  j.version = v;
  for (const k of Object.keys(j.optionalDependencies || {})) {
    j.optionalDependencies[k] = v;
  }
  fs.writeFileSync(p, JSON.stringify(j, null, 2) + '\n');
" "${MAIN_PKG}" "${VERSION}"

for entry in "${PLATFORMS[@]}"; do
  IFS='|' read -r DIR LABEL KIND BIN <<<"${entry}"
  set_version "${DIR}"
  DEST_DIR="${ROOT}/npm/${DIR}/bin"
  mkdir -p "${DEST_DIR}"
  ASSET="VanityAddress-${VERSION}-${LABEL}"

  if [[ "${KIND}" == "tar" ]]; then
    FILE="${ASSET}.tar.gz"
    download "${BASE}/${FILE}" "${TMP}/${FILE}"
    mkdir -p "${TMP}/${DIR}"
    tar -xzf "${TMP}/${FILE}" -C "${TMP}/${DIR}"
    # Archive may put binary at top level or nested
    FOUND="$(find "${TMP}/${DIR}" -type f -name "${BIN}" | head -1)"
    if [[ -z "${FOUND}" ]]; then
      echo "Binary ${BIN} not found in ${FILE}" >&2
      ls -laR "${TMP}/${DIR}" >&2
      exit 1
    fi
    cp "${FOUND}" "${DEST_DIR}/${BIN}"
    chmod +x "${DEST_DIR}/${BIN}"
  else
    FILE="${ASSET}.zip"
    download "${BASE}/${FILE}" "${TMP}/${FILE}"
    mkdir -p "${TMP}/${DIR}"
    unzip -qo "${TMP}/${FILE}" -d "${TMP}/${DIR}"
    FOUND="$(find "${TMP}/${DIR}" -type f -name "${BIN}" | head -1)"
    if [[ -z "${FOUND}" ]]; then
      echo "Binary ${BIN} not found in ${FILE}" >&2
      ls -laR "${TMP}/${DIR}" >&2
      exit 1
    fi
    cp "${FOUND}" "${DEST_DIR}/${BIN}"
  fi

  echo "OK ${DIR}/bin/${BIN} ($(wc -c < "${DEST_DIR}/${BIN}" | tr -d ' ') bytes)"
done

echo "Prepared npm packages for v${VERSION}"
