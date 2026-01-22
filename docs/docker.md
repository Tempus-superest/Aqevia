# Docker deployment

## Overview
- Each `aqevia-engine` container hosts exactly one **World**; the Compose stack purposely keeps a single service so there are never multiple Worlds inside one Engine instance.
- The dev image is built from `rust:1.92.0-slim-trixie`, so the toolchain and binary live together in one image (the binary stays at `/usr/local/bin/aqevia-engine` and `/app/VERSION` is embedded there as metadata). Mutable state is isolated inside `/data`, which is backed by the named volume `aqevia_data`.
- Persistence survives container restarts but is not guaranteed to be upgrade-compatible during the pre‑1.0 phase; if schema changes break compatibility, wiping `aqevia_data` is the supported recovery path. Production releases will later switch to a stable Debian runtime image (multi-stage) so only vetted runtime dependencies ship with the binary.

## Build

```bash
docker build -t aqevia-engine:dev .
```

- The Dockerfile is single-stage for dev parity: it uses `rust:1.92.0-slim-trixie`, installs `ca-certificates`, and runs `cargo fmt`, `cargo clippy`, and `cargo build --release --locked` under `/app/src`, keeping compilation and runtime dependencies in the same image.
- The release binary is copied into `/usr/local/bin/aqevia-engine` inside that same image, so the container can run immediately after a successful build.

## Run with Docker Compose

```bash
docker compose up --build
```

- Compose runs the `aqevia-engine` container as the non-root `aqevia` user, publishes port `7878`, and mounts the named volume `aqevia_data:/data` so `AQEVIA_SQLITE_PATH` can point to `/data/storage.sqlite` without writing into the container layer.
- The environment variables in compose match the Dockerfile defaults so the observability listener and persistence flush tuning behave identically inside the container.

## Configuration

- **Image/build settings**
  - Base image: `rust:1.92.0-slim-trixie`
  - Toolchain stages: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo build --release --locked` inside `/app/src`

- **Runtime filesystem**
  - `WORKDIR /app` (both for build and runtime steps)
  - `/usr/local/bin/aqevia-engine` (release binary)
  - `/app/VERSION` (copied from the repo and made readable by `aqevia`)
  - `VOLUME /data` (backed by compose volume `aqevia_data`)

- **User/permissions**
  - Dockerfile creates `aqevia` group/user (`groupadd -r aqevia && useradd --no-log-init -r -g aqevia aqevia`)
  - `/data` and `/app/VERSION` are chowned to `aqevia:aqevia` so the non-root process can read/write needed files

- **Network**
  - Compose maps host port `7878` to container port `7878`
  - The binary listens on `AQEVIA_OBSERVABILITY_ADDR` (default `0.0.0.0:7878`)

- **Environment variables**
  - `AQEVIA_SQLITE_PATH=/data/storage.sqlite` – keeps SQLite inside the durable named volume
  - `AQEVIA_OBSERVABILITY_ADDR=0.0.0.0:7878` – control-plane/observability endpoint
  - `PERSIST_FLUSH_INTERVAL_MS=1000` – flush cadence in milliseconds
  - `PERSIST_BATCH_CAPACITY=10` – how many records trigger an immediate flush

- **Entrypoint**
  - `ENTRYPOINT ["aqevia-engine"]` – no extra args; the binary reads env vars and runs the engine loop

## Operations

- **Inspect logs**: `docker compose logs -f aqevia-engine`
- **Inspect `/data`**: `docker compose exec aqevia-engine ls -la /data` or `docker run --rm -v aqevia_data:/data alpine ls /data`
- **Backup the volume**

  ```bash
  docker run --rm \
    -v aqevia_data:/data \
    -v "$(pwd)/backups":/backup \
    alpine \
    sh -c 'cd /data && tar czf /backup/aqevia-data-$(date +%F).tgz .'
  ```

- **Reset/purge data** *(development only, data loss warning)*:

  ```bash
  docker compose down
  docker volume rm aqevia_data
  docker compose up --build
  ```

  Recreating the named volume starts with a clean slate; use this only if you accept wiping every persisted snapshot.

## Single-World constraint

- One `aqevia-engine` container means one World. Deploy separate containers when you need multiple Worlds; never collapse them into a single Engine process.

## Roadmap note

- This dev workflow keeps the Rust toolchain and runtime image unified for simplicity and CI parity. When production readiness stabilizes, we will move to a multi-stage Dockerfile that builds with `rust:1.92.0-slim-trixie` but runs the release binary inside a stable Debian runtime image to ship only the necessary runtime dependencies.
