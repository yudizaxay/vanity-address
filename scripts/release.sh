#!/usr/bin/env bash
# One-command release: waits for every stage to succeed before starting the next.
#
#   ./scripts/release.sh                 # release the version in vanity-core/Cargo.toml
#   ./scripts/release.sh 0.8.2           # explicit version (must match the tree)
#   ./scripts/release.sh --dry-run       # preflight checks + print the plan, change nothing
#   ./scripts/release.sh --yes           # no confirmation prompts
#   ./scripts/release.sh --skip-local    # skip the local `make check-ci`
#
# Safe to re-run: every step detects work that is already done (tag pushed,
# release built, crate published, ...) and skips it, so after a failure just fix
# the problem and run the same command again.
#
# Stages:
#   0  preflight   tools, auth, branch, clean tree, versions in sync, CHANGELOG, local CI
#   1  push main   (only if local commits are ahead) -> wait for CI on that commit
#   2  tag         create + push vX.Y.Z -> wait for the Release workflow -> verify assets
#   3  homebrew    update Formula (sha256) -> commit + push -> wait CI -> sync tap repo
#   4  crates.io   publish vanity-core -> wait for index -> publish vanity-address
#   5  npm         build wasm -> download release binaries -> publish
#
# Requires: git, gh (authenticated), cargo (logged in), npm, node, curl, wasm-pack.
# Homebrew tap repo expected at ../homebrew-tap (or $HOMEBREW_TAP_DIR).

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "${ROOT}"

REPO="yudizaxay/vanity-address"
CI_TIMEOUT="${CI_TIMEOUT:-1800}"        # seconds to wait for a CI run
RELEASE_TIMEOUT="${RELEASE_TIMEOUT:-3600}"
INDEX_TIMEOUT="${INDEX_TIMEOUT:-600}"
POLL=20

VERSION=""
DRY_RUN=false
ASSUME_YES=false
SKIP_LOCAL=false

for arg in "$@"; do
  case "${arg}" in
    --dry-run) DRY_RUN=true ;;
    --yes|-y) ASSUME_YES=true ;;
    --skip-local) SKIP_LOCAL=true ;;
    -h|--help) sed -n '2,24p' "$0"; exit 0 ;;
    -*) echo "Unknown option: ${arg}" >&2; exit 1 ;;
    *) VERSION="${arg#v}" ;;
  esac
done

# ---------- helpers ----------
if [[ -t 1 ]]; then B=$'\033[1m'; G=$'\033[32m'; R=$'\033[31m'; Y=$'\033[33m'; N=$'\033[0m'; else B=""; G=""; R=""; Y=""; N=""; fi
step() { echo; echo "${B}==> $*${N}"; }
ok()   { echo "${G}✔ $*${N}"; }
warn() { echo "${Y}! $*${N}"; }
die()  { echo "${R}✘ $*${N}" >&2; exit 1; }

confirm() {
  $ASSUME_YES && return 0
  local reply
  read -r -p "$1 [y/N] " reply
  [[ "${reply}" == "y" || "${reply}" == "Y" ]] || die "Aborted."
}

cargo_version() { sed -n 's/^version = "\(.*\)"/\1/p' "$1" | head -1; }
json_version()  { node -p "require('$1').version"; }

# Wait until every run of <workflow> for <commit-or-branch> has finished; fail on any non-success.
# usage: wait_for_runs <workflow-file> <label> <timeout> commit <sha> | branch <name>
wait_for_runs() {
  local wf="$1" label="$2" timeout="$3" kind="$4" ref="$5"
  local start now out total pending bad
  start="$(date +%s)"
  echo "Waiting for ${label} (${wf} @ ${ref:0:12}) ..."
  while true; do
    if [[ "${kind}" == "commit" ]]; then
      out="$(gh run list --repo "${REPO}" --workflow "${wf}" --commit "${ref}" --json status,conclusion,databaseId,url --limit 20 2>/dev/null || echo '[]')"
    else
      out="$(gh run list --repo "${REPO}" --workflow "${wf}" --branch "${ref}" --json status,conclusion,databaseId,url --limit 20 2>/dev/null || echo '[]')"
    fi
    total="$(node -p "JSON.parse(process.argv[1]).length" "${out}")"
    now="$(date +%s)"
    if [[ "${total}" == "0" ]]; then
      (( now - start > 300 )) && die "No ${wf} run appeared for ${ref} after 5 min — is the workflow triggered by this ref?"
      sleep "${POLL}"; continue
    fi
    pending="$(node -p "JSON.parse(process.argv[1]).filter(r=>r.status!=='completed').length" "${out}")"
    bad="$(node -p "JSON.parse(process.argv[1]).filter(r=>r.status==='completed'&&!['success','skipped'].includes(r.conclusion)).map(r=>r.url).join('\n')" "${out}")"
    if [[ -n "${bad}" ]]; then
      die "${label} FAILED. Fix it, push, then re-run this script.
${bad}"
    fi
    if [[ "${pending}" == "0" ]]; then ok "${label} succeeded"; return 0; fi
    (( now - start > timeout )) && die "${label}: timed out after ${timeout}s (still ${pending} run(s) pending)"
    printf '  ... %s run(s) in progress (%ss elapsed)\r' "${pending}" "$(( now - start ))"
    sleep "${POLL}"
  done
}

crate_published() { # name version
  local code
  code="$(curl -s -o /dev/null -w '%{http_code}' -A "vanity-address-release-script" "https://crates.io/api/v1/crates/$1/$2" || echo 000)"
  [[ "${code}" == "200" ]]
}

wait_for_crate() { # name version
  local start now
  start="$(date +%s)"
  echo "Waiting for $1 $2 on the crates.io index ..."
  until crate_published "$1" "$2"; do
    now="$(date +%s)"; (( now - start > INDEX_TIMEOUT )) && die "$1 $2 not visible on crates.io after ${INDEX_TIMEOUT}s"
    sleep 10
  done
  ok "$1 $2 is on crates.io"
}

# ---------- 0. preflight ----------
step "0/5 Preflight"

for tool in git gh cargo npm node curl; do
  command -v "${tool}" >/dev/null || die "Missing tool: ${tool}"
done
gh auth status >/dev/null 2>&1 || die "gh is not authenticated — run: gh auth login"

TREE_VERSION="$(cargo_version vanity-core/Cargo.toml)"
VERSION="${VERSION:-${TREE_VERSION}}"
TAG="v${VERSION}"
[[ -n "${VERSION}" ]] || die "Could not determine version"

branch="$(git branch --show-current)"
[[ "${branch}" == "main" ]] || die "Not on main (on '${branch}'). Merge the PR and 'git checkout main && git pull' first."
[[ -z "$(git status --porcelain)" ]] || die "Working tree not clean — commit or stash first:
$(git status --short)"

git fetch -q --tags origin
read -r BEHIND AHEAD < <(git rev-list --left-right --count origin/main...HEAD)
(( BEHIND == 0 )) || die "Local main is ${BEHIND} commit(s) behind origin/main — git pull first."

# every version field must equal VERSION
mismatch=""
check_ver() { # label actual
  [[ "$2" == "${VERSION}" ]] || mismatch="${mismatch}  $1: $2 (expected ${VERSION})"$'\n'
}
check_ver vanity-core/Cargo.toml "$(cargo_version vanity-core/Cargo.toml)"
check_ver vanity-address/Cargo.toml "$(cargo_version vanity-address/Cargo.toml)"
check_ver vanity-wasm/Cargo.toml "$(cargo_version vanity-wasm/Cargo.toml)"
check_ver vanity-app/src-tauri/Cargo.toml "$(cargo_version vanity-app/src-tauri/Cargo.toml)"
check_ver vanity-app/package.json "$(json_version ./vanity-app/package.json)"
check_ver vanity-app/src-tauri/tauri.conf.json "$(json_version ./vanity-app/src-tauri/tauri.conf.json)"
for p in npm/*/package.json; do check_ver "${p}" "$(json_version "./${p}")"; done
[[ -z "${mismatch}" ]] || die "Version mismatch — bump every file to ${VERSION}:
${mismatch}"
grep -q "^## \[${VERSION}\]" CHANGELOG.md || die "CHANGELOG.md has no '## [${VERSION}]' section"
ok "version ${VERSION} consistent everywhere, CHANGELOG present"

TAG_ON_REMOTE=false
git ls-remote --exit-code --tags origin "refs/tags/${TAG}" >/dev/null 2>&1 && TAG_ON_REMOTE=true
if $TAG_ON_REMOTE; then
  tag_commit="$(git rev-list -n1 "${TAG}")"
  [[ "${tag_commit}" == "$(git rev-parse HEAD)" ]] || die "Tag ${TAG} already exists on a different commit than HEAD — refusing to continue."
  warn "Tag ${TAG} already on origin (resuming a previous release)"
fi

TAP_DIR="${HOMEBREW_TAP_DIR:-${ROOT}/../homebrew-tap}"
[[ -d "${TAP_DIR}/.git" ]] || die "Homebrew tap repo not found at ${TAP_DIR} (set HOMEBREW_TAP_DIR)"

if ! $SKIP_LOCAL && ! $DRY_RUN; then
  step "Local check: make check-ci"
  make check-ci >/dev/null 2>&1 || { make check-ci 2>&1 | tail -30; die "Local make check-ci failed"; }
  ok "make check-ci passed"
fi

echo
echo "Release plan for ${B}${TAG}${N}:"
(( AHEAD > 0 )) && echo "  1. push ${AHEAD} local commit(s) to origin/main, wait for CI" || echo "  1. main already pushed, wait for CI on $(git rev-parse --short HEAD)"
$TAG_ON_REMOTE && echo "  2. tag exists — wait for Release workflow + verify assets" || echo "  2. create + push tag, wait for Release workflow, verify assets"
echo "  3. update Homebrew formula, push, wait CI, sync tap (${TAP_DIR})"
echo "  4. cargo publish vanity-core, wait for index, publish vanity-address"
echo "  5. build wasm, prepare + publish npm packages"

if $DRY_RUN; then ok "Dry run complete — nothing was changed."; exit 0; fi
confirm "Start the release of ${TAG}?"

# ---------- 1. push main + CI ----------
step "1/5 Push main and wait for CI"
if (( AHEAD > 0 )); then
  git push origin main
  ok "pushed ${AHEAD} commit(s)"
fi
wait_for_runs ci.yml "CI on main" "${CI_TIMEOUT}" commit "$(git rev-parse HEAD)"

# ---------- 2. tag + Release workflow ----------
step "2/5 Tag ${TAG} and wait for the Release workflow"
if ! $TAG_ON_REMOTE; then
  git tag -a "${TAG}" -m "${TAG}"
  git push origin "${TAG}"
  ok "pushed tag ${TAG}"
fi
wait_for_runs release.yml "Release workflow" "${RELEASE_TIMEOUT}" branch "${TAG}"

ASSETS="$(gh release view "${TAG}" --repo "${REPO}" --json assets -q '.assets | length' 2>/dev/null || echo 0)"
(( ASSETS >= 6 )) || die "GitHub Release ${TAG} has only ${ASSETS} asset(s) (expected >= 6 binaries)"
ok "GitHub Release ${TAG} has ${ASSETS} assets"

# ---------- 3. Homebrew ----------
step "3/5 Homebrew formula + tap"
if grep -q "refs/tags/${TAG}.tar.gz" Formula/vanity-address.rb; then
  ok "Formula already points to ${TAG}"
else
  ./scripts/update-homebrew-formula.sh "${VERSION}"
  git add Formula/vanity-address.rb
  git commit -q -m "chore: Homebrew formula ${VERSION}"
  git push origin main
  wait_for_runs ci.yml "CI on formula commit" "${CI_TIMEOUT}" commit "$(git rev-parse HEAD)"
fi
if cmp -s Formula/vanity-address.rb "${TAP_DIR}/Formula/vanity-address.rb"; then
  ok "Tap already has this formula"
else
  HOMEBREW_TAP_DIR="${TAP_DIR}" ./scripts/sync-homebrew-tap.sh --push "vanity-address ${VERSION}"
fi

# ---------- 4. crates.io ----------
step "4/5 crates.io"
confirm "Publish vanity-core and vanity-address ${VERSION} to crates.io (irreversible)?"
if crate_published vanity-core "${VERSION}"; then
  ok "vanity-core ${VERSION} already published"
else
  cargo publish -p vanity-core
  wait_for_crate vanity-core "${VERSION}"
fi
if crate_published vanity-address "${VERSION}"; then
  ok "vanity-address ${VERSION} already published"
else
  cargo publish -p vanity-address
  wait_for_crate vanity-address "${VERSION}"
fi

# ---------- 5. npm ----------
step "5/5 npm"
if [[ "$(npm view "vanity-address@${VERSION}" version 2>/dev/null || true)" == "${VERSION}" ]]; then
  ok "npm vanity-address@${VERSION} already published"
else
  ./scripts/build-wasm.sh
  ./scripts/prepare-npm.sh "${VERSION}"
  ./scripts/publish-npm.sh
  ok "npm packages published"
fi

echo
ok "Release ${TAG} complete."
echo "Still manual: update AGENTS.md / .cursor/rules status, and check https://github.com/${REPO}/releases/tag/${TAG}"
