#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
PKG_DIR="$ROOT_DIR/crates/wasm/pkg"
WASM_FILE="$PKG_DIR/genshin_calc_wasm_bg.wasm"

TARGET="${1:-web}"

echo "==> Building WASM (target: $TARGET)..."
wasm-pack build "$ROOT_DIR/crates/wasm" --release --target "$TARGET"

if command -v wasm-opt >/dev/null 2>&1; then
    echo "==> Optimizing with wasm-opt -Oz..."
    wasm-opt -Oz \
        --enable-bulk-memory \
        --enable-nontrapping-float-to-int \
        "$WASM_FILE" -o "$WASM_FILE"
    echo "    Done."
else
    echo "==> wasm-opt not found, skipping. Install with: cargo install wasm-opt"
fi

RAW_SIZE=$(stat -c%s "$WASM_FILE" 2>/dev/null || stat -f%z "$WASM_FILE")
GZ_SIZE=$(gzip -c "$WASM_FILE" | wc -c)
printf "==> Result: %d KB raw / %d KB gzip\n" $((RAW_SIZE / 1024)) $((GZ_SIZE / 1024))
