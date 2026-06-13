#!/usr/bin/env bash
set -euo pipefail

# Run tests
echo "==> Running tests before release..."
cargo test

# Build release
echo "==> Building release..."
cargo build --release

# Get current version from Cargo.toml
VERSION=$(grep -m1 '^version = ' Cargo.toml | cut -d '"' -f2)
TAG="v$VERSION"

echo "==> Tagging Git release as $TAG..."
if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo "Tag $TAG already exists, skipping tagging."
else
    git tag -a "$TAG" -m "Release $TAG"
    echo "==> Pushing git tag $TAG..."
    git push origin "$TAG" || echo "Warning: failed to push tag to remote origin, tag created locally."
fi

echo "==> Release process completed successfully."
