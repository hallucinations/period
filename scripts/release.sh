#!/usr/bin/env bash
# Usage: ./scripts/release.sh <version>
# Example: ./scripts/release.sh 0.3.0
#
# What it does:
#   1. Validates the version argument
#   2. Ensures the working tree is clean and on the main branch
#   3. Bumps the version in Cargo.toml (and updates Cargo.lock)
#   4. Runs the full test + lint suite
#   5. Commits, tags, and pushes to origin

set -euo pipefail

# ── Argument validation ────────────────────────────────────────────────────────

if [ $# -ne 1 ]; then
  echo "Usage: $0 <version>   (e.g. 0.3.0)"
  exit 1
fi

VERSION="${1#v}"  # strip leading 'v' if present

if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "Error: version must be in X.Y.Z format (got '${VERSION}')"
  exit 1
fi

TAG="v${VERSION}"

# ── Git preconditions ──────────────────────────────────────────────────────────

BRANCH=$(git symbolic-ref --short HEAD)
if [ "$BRANCH" != "main" ]; then
  echo "Error: must be on 'main' branch (currently on '${BRANCH}')"
  exit 1
fi

if ! git diff --quiet || ! git diff --cached --quiet; then
  echo "Error: working tree has uncommitted changes — commit or stash them first"
  exit 1
fi

if git tag --list | grep -q "^${TAG}$"; then
  echo "Error: tag ${TAG} already exists"
  exit 1
fi

# ── Check current version ──────────────────────────────────────────────────────

CURRENT=$(grep '^version' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo "Bumping ${CURRENT} → ${VERSION}"

# ── Update Cargo.toml ─────────────────────────────────────────────────────────

sed -i.bak "s/^version = \"${CURRENT}\"/version = \"${VERSION}\"/" Cargo.toml
rm Cargo.toml.bak

# Update Cargo.lock without upgrading any dependencies
cargo update --workspace --quiet 2>/dev/null || true

# ── Run tests ─────────────────────────────────────────────────────────────────

echo "Running test suite..."
cargo test --all-features --quiet

echo "Running lint..."
cargo clippy --quiet -- -D warnings
cargo fmt --check

# ── Commit, tag, push ─────────────────────────────────────────────────────────

git add Cargo.toml Cargo.lock
git commit -m "chore: release ${TAG}"
git tag "${TAG}"
git push origin main
git push origin "${TAG}"

echo ""
echo "Released ${TAG} — the CI release workflow will publish to crates.io."
