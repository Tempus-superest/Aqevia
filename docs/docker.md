# Docker

This document explains how the Aqevia Engine Docker artifacts stay aligned with the repository `VERSION` file, including the build-time metadata in `Dockerfile` and the Compose service tag that operators use locally.

## Image build (`Dockerfile`)

The single-stage `Dockerfile` compiles the Rust binary, copies the built artifact into `/usr/local/bin`, and ships a minimal runtime image. A build-time argument (`AQEVIA_VERSION`) is passed in from the Compose stack so the final image surfaces the release number via OCI metadata:

- `ARG AQEVIA_VERSION` makes the version available during the build.
- `LABEL org.opencontainers.image.title=Aqevia` and `LABEL org.opencontainers.image.version=$AQEVIA_VERSION` ensure the produced image can be traced back to the labeled release number without needing Git metadata.
- `VERSION` is copied into `/app/VERSION` so runtime tooling can inspect the captured release number if needed.

## Compose stack (`docker-compose.yml`)

The Compose file builds the local `Dockerfile`, tags the resulting image as `aqevia-engine:${AQEVIA_VERSION}`, and keeps the database volume for `/data`. The `AQEVIA_VERSION` build arg is forwarded to the container build so the embedded OCI label stays in sync with the tagged image.

Compose automatically reads `.env` files at the project root, so you no longer need to `export` `AQEVIA_VERSION` manually. Run `scripts/sync-version` after editing `VERSION` (the script rewrites `.env` with `AQEVIA_VERSION=<version>`), verify everything with `scripts/check-version-sync`, and then run `docker compose up -d --build`. Docker Compose uses the `.env`-provided `AQEVIA_VERSION` when tagging the image and forwarding the build arg.

If you reset to a new release, rerun `scripts/sync-version` so `.env` stays aligned with the updated tag; Compose’ automatic loading of `.env` keeps the service definition (`aqevia-engine`) and runtime defaults unchanged while the tagged image is refreshed.

## Compose versioning (`.env`)

The root `.env` file is a derived surface of `VERSION` that `scripts/sync-version` regenerates. It contains only:

```
AQEVIA_VERSION=<current-version>
```

Compose reads this file and interpolates `${AQEVIA_VERSION}` in `docker-compose.yml`, making the workflow deterministic:

1. Edit `VERSION`.
2. Run `./scripts/sync-version` (regenerates `.env`).
3. Run `./scripts/check-version-sync`.
4. Run `docker compose up -d --build`.

## Runtime configuration (env)

Compose passes the following env vars into the runtime image so defaults remain deterministic:

- `AQEVIA_SQLITE_PATH=/data/storage.sqlite` — durable store location inside the container (matches the Dockerfile default); override if you mount a different data volume.
- `PERSIST_FLUSH_INTERVAL_MS=1000` and `PERSIST_BATCH_CAPACITY=10` — tune the persistence cadence and bounded batch size when you need throughput or durability adjustments.
- `AQEVIA_OBSERVABILITY_ADDR=0.0.0.0:7878` — opens `/health`, `/ready`, and `/status` on port 7878 inside the container and can be rewritten by external proxies; keep the listener per-process and do not expose it publicly without a trusted proxy (see `docs/engine/http-conventions.md` for runtime defaults).

## Deployment constraints

- **1 World = 1 deployment unit.** Each Docker container runs exactly one Aqevia Engine and its associated World, so scale by running additional containers rather than sharing a container between Worlds.
- Observability and control-plane endpoints stay on the same HTTP origin (the unified SPA model) while `/health`, `/ready`, and `/status` are exposed on `AQEVIA_OBSERVABILITY_ADDR`. Avoid binding that address to public interfaces without extra gateway protections, especially since `/status` discloses runtime metrics.
