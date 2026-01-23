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
