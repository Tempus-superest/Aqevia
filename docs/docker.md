# Docker

This document describes Backlit's Docker environment, including the Dockerfile build pipeline, the Compose configuration, runtime defaults, and the supported smoke-test workflow.

## Image build (Dockerfile)

Backlit ships a multi-stage Dockerfile that compiles both the Rust binary and the Web UI assets before assembling a slim runtime image. The builder stage uses the `rustlang/rust:nightly` base image, installs Rust build dependencies plus Node/NPM for the UI build, installs UI dependencies, builds the UI, and then builds the `backlit` release binary. The final runtime stage uses `debian:stable-slim` and copies the built binary into `/usr/bin/backlit`.【F:Dockerfile†L1-L33】

Key details from the Dockerfile:

- **Builder stage**: installs `build-essential`, `pkg-config`, `libssl-dev`, `nodejs`, `npm`, and `cmake` for compiling Rust and UI assets; runs `npm install` and `npm run build` in `ui/`, then `cargo build --release --bin backlit`.【F:Dockerfile†L1-L21】
- **Runtime stage**: installs minimal tooling (`ca-certificates`, `curl`, `iproute2`, `dnsutils`) to support smoke tests and container diagnostics; these tools are explicitly noted as dev/test helpers and not required for runtime correctness.【F:Dockerfile†L23-L33】
- **Runtime user**: creates a non-root `backlit` user and sets `/opt/backlit` as the working directory, owned by that user.【F:Dockerfile†L30-L33】
- **Entrypoint**: runs `backlit serve --data-dir /opt/backlit` as the default command.【F:Dockerfile†L33-L41】
- **Exposed ports**: 80 (HTTP), 443 (HTTPS), and 9090 (admin API).【F:Dockerfile†L37-L38】

## Docker Compose configuration

The repository includes a `docker-compose.yml` that builds the local Dockerfile, tags the resulting image with the Backlit semantic version, and exposes the public and admin listeners. It also mounts a persistent volume for `/opt/backlit` so the SQLite state database and secrets material survive container restarts.【F:docker-compose.yml†L1-L19】

Key details from the Compose file:

- **Service name**: `backlit` (single container).【F:docker-compose.yml†L1-L4】
- **Image tag**: `backlit:v0.6.22`, keeping the container image aligned with the crate version and release artifacts.【F:docker-compose.yml†L3-L4】
- **Port mappings**:
  - `80:80` (public HTTP)
  - `443:443` (public HTTPS)
  - `127.0.0.1:9090:9090` (admin API bound to loopback on the host).【F:docker-compose.yml†L5-L9】
- **Volume**: `backlit-data:/opt/backlit` for persistent state and secrets artifacts.【F:docker-compose.yml†L10-L12】
- **Restart policy**: `unless-stopped`.【F:docker-compose.yml†L12-L13】
- **Environment defaults**:
  - `RUST_LOG=info`
  - `RUST_BACKTRACE=1`【F:docker-compose.yml†L13-L16】
- **Optional DNS override**: commented-out `dns:` section for testing against specific resolvers.【F:docker-compose.yml†L16-L20】

## Runtime defaults and persistence

- **Data directory**: the image runs `backlit serve --data-dir /opt/backlit`, so all state is rooted at `/opt/backlit` inside the container by default.【F:Dockerfile†L33-L41】
- **Persistent storage**: the Compose stack binds the `backlit-data` volume to `/opt/backlit`, preserving the SQLite database and any secrets material across restarts and rebuilds.【F:docker-compose.yml†L10-L12】
- **Non-root runtime**: the container runs as the `backlit` user to avoid root-level execution in production deployments.【F:Dockerfile†L30-L39】

## Logging and diagnostics

Backlit emits logs to stdout/stderr, so container logs are available via `docker logs` without file logging. The runtime image also includes a small set of utilities (`curl`, `ss` via `iproute2`, DNS tools) to support debugging and smoke tests. These tools are explicitly marked as dev/test helpers and not part of runtime correctness, so they may be removed or split into a dev-only image in the future.【F:Dockerfile†L23-L33】

## Smoke testing in Docker

Backlit provides an end-to-end Docker smoke test script that brings up the Compose stack, captures the bootstrap admin password from container logs, provisions a proxy site via the Admin API, and verifies routing through the public listener. The script also starts a temporary upstream container on the Compose network and asserts that requests routed through Backlit reach that upstream. The script cleans up the Compose stack and test upstream container on exit.【F:scripts/smoke_docker_proxy.sh†L1-L105】【F:scripts/smoke_docker_proxy.sh†L106-L176】

You can run the smoke test via Make:

```bash
make smoke
```

The `make smoke` target is defined in the repo Makefile and points to `scripts/smoke_docker_proxy.sh`, keeping the Docker smoke workflow discoverable and consistent with the testing documentation.【F:Makefile†L9-L15】

## Operator workflow (Compose)

To run Backlit locally with Docker Compose:

```bash
docker compose up -d --build
```

This publishes the admin listener on `127.0.0.1:9090`, while HTTP/HTTPS remain exposed on `0.0.0.0:80` and `0.0.0.0:443`. A named volume (`backlit-data`) keeps `/opt/backlit` persisted for state and secrets. Diagnostics can be performed with `docker exec backlit ss -lntp` to confirm listeners and `docker logs backlit` for runtime logs.【F:README.md†L165-L178】【F:docker-compose.yml†L5-L12】

## References

- Docker smoke test expectations and tooling are outlined in the testing documentation, including the rationale for bundling `curl`/`ss` inside the image for end-to-end checks.【F:docs/testing.md†L111-L152】
