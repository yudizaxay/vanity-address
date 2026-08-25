#!/usr/bin/env bash
set -euo pipefail

# Note: Building for wasm32 target requires a wasm-capable C toolchain.
# Install via: brew install llvm
# Then set environment variables:
#   export CC_wasm32_unknown_unknown="$(brew --prefix llvm)/bin/clang"
#   export AR_wasm32_unknown_unknown="$(brew --prefix llvm)/bin/llvm-ar"

cd "$(dirname "$0")/.."

OUT_DIR="npm/vanity-address/wasm"
rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

echo "Building vanity-wasm (nodejs target)..."
wasm-pack build vanity-wasm --target nodejs --out-dir "../$OUT_DIR/nodejs" --out-name vanity_wasm --release

echo "Building vanity-wasm (bundler target)..."
wasm-pack build vanity-wasm --target bundler --out-dir "../$OUT_DIR/bundler" --out-name vanity_wasm --release

# wasm-pack writes its own package.json into each out-dir; the npm package's
# own package.json (Task 5) is the one that actually gets published, so drop these.
rm -f "$OUT_DIR/nodejs/package.json" "$OUT_DIR/bundler/package.json"
rm -f "$OUT_DIR/nodejs/.gitignore" "$OUT_DIR/bundler/.gitignore"

echo "Done. Artifacts in $OUT_DIR/{nodejs,bundler}"
