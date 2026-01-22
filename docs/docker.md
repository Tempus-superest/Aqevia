# Docker deployment

This section captures how to build and run the single-World Aqevia Engine container so operators can reproduce the same binary that CI runs.

## Build the image

```bash
docker build -t aqevia-engine:0.3.0 .
```

The `Dockerfile` is multi-stage: it compiles the Rust workspace (via `cargo fmt`, `cargo clippy`, and `cargo build --release --locked`) and copies the `aqevia-engine` binary into a slim Debian runtime image. The default entrypoint reads `VERSION` at `/app/VERSION` so version bumps are reflected inside the container.

## Run via Docker Compose

```bash
docker compose up --build
```

The provided `docker-compose.yml` runs a single `aqevia-engine` service (one World per deployment unit) and exposes the observability/HTTP port `7878`. It mounts `./data/sqlite` into `/data` so SQLite state is durable between restarts. Environment variables used by the service:

- `AQEVIA_SQLITE_PATH` (default `/data/storage.sqlite`)
- `AQEVIA_OBSERVABILITY_ADDR` (default `0.0.0.0:7878`)
- `PERSIST_FLUSH_INTERVAL_MS` / `PERSIST_BATCH_CAPACITY` (flush cadence)

The `.dockerignore` file prevents git metadata, build artifacts, and local data from being pushed into the container context.

## Single-World constraint

The Docker Compose setup declares exactly one `aqevia-engine` replica and does not run additional worlds in the same container or service. To host more worlds, deploy multiple containers (one per World) rather than stacking them inside a single service.
