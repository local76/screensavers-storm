#!/usr/bin/env bash
set -euo pipefail

echo "==> Building in release mode..."
cargo build --release

echo "==> Copying binary to dist/binaries..."
mkdir -p dist/binaries
cp target/release/storm dist/binaries/

echo "==> Build complete. Binary located in dist/binaries/storm"
