#!/usr/bin/env sh
set -eu

# Run from repo root.
cd "$(dirname "$0")/.."

run_step() {
  printf '\n=== %s ===\n' "$1"
  shift
  "$@"
}

run_step "Running cargo fmt --check" cargo fmt --check
run_step "Running cargo clippy --all-targets --all-features -D warnings" \
  cargo clippy --all-targets --all-features -D warnings
run_step "Running cargo test" cargo test

printf '\n=== All checks passed ===\n'
