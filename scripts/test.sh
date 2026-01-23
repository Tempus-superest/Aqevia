#!/usr/bin/env sh
set -eu
set -o pipefail

# Run from the workspace root (src).
REPO_ROOT="$(cd "$(dirname "$0")/.."; pwd)"
WORKSPACE_ROOT="$REPO_ROOT/src"

run_step() {
  printf '\n=== %s ===\n' "$1"
  shift
  "$@"
}

run_step "Checking version sync" ./scripts/check-version-sync

if [ ! -d "$WORKSPACE_ROOT" ]; then
  printf 'Workspace root %s not found\n' "$WORKSPACE_ROOT" >&2
  exit 1
fi

cd "$WORKSPACE_ROOT"

export CARGO_INCREMENTAL=0

run_step "Running cargo fmt --all -- --check" cargo fmt --all -- --check
run_step "Running cargo clippy --all-targets --all-features -- -D warnings" \
  cargo clippy --all-targets --all-features -- -D warnings
run_step "Running cargo test --all" cargo test --all

printf '\n=== All checks passed ===\n'
