#!/usr/bin/env bash
set -euo pipefail

# Ensure the commands run from the repository root.
cd "$(dirname "$0")/.."

cargo fmt --check
cargo clippy --all-targets --all-features -D warnings
cargo test
