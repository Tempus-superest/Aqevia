# Aqevia v0.1.1

This repo hosts the Aqevia Engine, Builder, Admin, and Client tooling. `./scripts/test.sh` runs the canonical format, lint, and test pipeline against the Rust workspace under `src/`. For containerized runs see [`docs/docker.md`](docs/docker.md).

## Workspace layout

- Root `Cargo.toml` points to the `src/` workspace manifest so `cargo metadata --no-deps` can resolve the `kernel`, `router`, `transport`, `engine`, `storage`, `storage-sqlite`, and `bin/aqevia-engine` crates.
- The workspace keeps Kernel/Router/Transport boundaries clear (Kernel owns game rules, Router delivers sessions, Transport handles HTTP/WS) and supports the “1 World = 1 deployment unit” constraint described in the docs.
