# Docker deployment

## Configuration

### Dockerfile

### docker-compose

- `AQEVIA_SQLITE_PATH` – path to the SQLite database file (default `/data/storage.sqlite` to keep data inside the durable volume).
- `AQEVIA_OBSERVABILITY_ADDR` – observability HTTP address (default `0.0.0.0:7878`, exposed via compose port 7878).
- `PERSIST_FLUSH_INTERVAL_MS` – flush cadence in milliseconds (default `1000`).
- `PERSIST_BATCH_CAPACITY` – number of pending writes before a flush (default `10`).

## Overview

- Aqevia Engine runs exactly one **World** per container; you get a single immutable image (binary baked into `/usr/local/bin/aqevia-engine`) and a single mutable data volume (`/data`) that holds all durable state.
- The image contains no writable game files—the binary and `VERSION` are part of the image and are upgraded only by rebuilding/pulling a new image. Anything written at runtime goes under `/data` so it survives container restarts.
- During early development, persistence is durable across restarts but not necessarily upgrade-compatible; wiping `aqevia_data` is the supported way to recover from incompatible storage changes.

## Build the image

```bash
docker build -t aqevia-engine:latest .
```

The multi-stage Dockerfile compiles the Rust workspace in a `rust` builder image and copies the release `aqevia-engine` binary plus `VERSION` into a slim Debian runtime image. The final image only needs the binary and `/app/VERSION`.

## Run with Docker Compose

```bash
docker compose up --build
```

- `docker-compose.yml` defines one `aqevia-engine` service that exposes port `7878`.
- The service mounts the named volume `aqevia_data` at `/data`, so durable state (including the SQLite database) lives in that volume rather than the container layer.
- Environment variables in the file point `AQEVIA_SQLITE_PATH` at `/data/storage.sqlite` and configure observability and persistence flush behavior.

## Persistence model

- All writable state lives under `/data` (the bind mount is declared as `aqevia_data:/data` in compose). The engine uses `AQEVIA_SQLITE_PATH` to locate the `storage.sqlite` file, which defaults to `/data/storage.sqlite`.
- The only durable resource loaded on startup is `/data/storage.sqlite`; no other directories are written by the container because all UI assets, logs, and observability output are produced by the binary and streamed out.
- Respect secure ownership of `/data` (the Dockerfile creates it and chowns to the `aqevia` user) to prevent unauthorized tampering. Early development builds may drop or reset `/data` when schema changes occur.

## Operational commands

- **Inspect the volume contents**

  ```bash
  docker compose exec aqevia-engine ls -la /data
  ```

- **Backup the volume**

  ```bash
  docker run --rm \
    -v aqevia_data:/data \
    -v "$(pwd)/backups":/backup \
    alpine \
    sh -c 'cd /data && tar czf /backup/aqevia-data-$(date +%F).tgz .'
  ```

- **Reset or wipe persistent state** *(development only)*  

  ```bash
  docker volume rm aqevia_data
  ```

  This removes every file under `/data`. Recreating the volume with `docker compose up` starts from a clean slate; use it only when you accept data loss.

## Single-World constraint

- The compose setup runs a single `aqevia-engine` replica; one container equals one World. To host additional Worlds, deploy separate containers so no Engine ever hosts multiple Worlds simultaneously.
