# Milestones

This document breaks Aqevia into milestone deliverables. Each milestone includes goals and acceptance criteria, and the final milestone culminates in a v1 commit/release. The milestones track the repo’s actual state today—sparse tooling plus rich documentation—before progressing toward the fully implemented Engine, UIs, and AI-enabled experience described in `/AGENTS.md`.

## Repository baseline

- **Layout:** the workspace contains docs (`/docs/`), the Rust engine workspace under `src/`, storage crates (`aqevia-storage` / `aqevia-storage-sqlite`), empty `ui/` folders, `scripts/test.sh`, `VERSION`, and the docs referenced by `/AGENTS.md`.
- **Build/test entry points:** `./scripts/test.sh` now executes from `src/`, running `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all` against the workspace. `.editorconfig`, `docs/testing.md`, and `docs/style.md` describe these commands as the canonical automation path.
- **Docs coverage:** every `/docs/*` contract listed in `/AGENTS.md` exists. `docs/database.md` now documents the SQLite schema and flush cadence, `docs/engine/observability-api.md` spells out `/health`, `/ready`, and `/status`, and `docs/engine/http-conventions.md` and `docs/security.md` explain the resulting behavior.
- **Current implementation gaps:** HTTP control-plane handlers, WS data-plane logic, and packaged UIs are still pending; the repo must now layer on storage-backed observability before implementing actual gameplay/API contracts while continuing to honor “1 World = 1 deployment unit,” Kernel/Router/Transport boundaries, modular storage, and AI guardrails.

## Milestone 0 — Scaffolding foundation and doc compliance

### Goals
- Record the authoritative docs, configuration, and scripts that describe the system before any runtime exists.
- Align the repo with the “1 World = 1 deployment unit” rule and layered engine boundaries by describing them in `/docs/` assets and covering them in `/docs/testing.md`, `/docs/style.md`, and `/docs/versioning.md`.
- Provide minimal automation so contributors know how to format, lint, and test once code appears.

### Acceptance criteria
- All `/docs/` files referenced by `/AGENTS.md` exist (even as placeholders) and are listed from the repo root (`ls docs/ | grep ...` or similar visibility check).
- `.editorconfig`, `scripts/test.sh`, `VERSION`, `scripts/version-locations.yml`, and `scripts/check-version-sync` are present; `/docs/testing.md` and `/docs/style.md` now point to the canonical script and formatting rules.
- `src/` and each `ui/…/` folder contains a README describing the intended artifact; these directories prove the workspace layout for later milestones.
- Repo rules such as “1 World = 1 deployment unit,” Kernel/Router/Transport boundaries, pluggable storage, and AI guardrails are explained (at least in `/docs/aqevia-engine.md`, `/docs/style.md`, or `/docs/engine/*`) so that Milestone 0 documentation is complete.

## Milestone 1 — Engine workspace and formatting/linting pipeline

### Goals
- Introduce a Rust workspace under `src/` that will host the Engine kernel, router, and transport crates while preserving the documented 1 World = 1 deployment unit and strict layer boundaries.
- Ensure the canonical format/lint/test automation works end-to-end by wiring `cargo fmt --check`, `cargo clippy --all-targets --all-features -D warnings`, and `cargo test` into `./scripts/test.sh`.

### Acceptance criteria
- A `Cargo.toml` workspace at `src/` defines the Engine package(s), and `cargo fmt`, `cargo clippy`, and `cargo test` each succeed locally (even if they currently run no tests).
- The workspace enforces Kernel/Router/Transport separation (no cross-layer imports in crate manifests) and owns the “one World per engine” config (the runnable binary only serves a single World and exposes the embedded UIs referenced in `/docs/aqevia-client.md`, `/docs/aqevia-builder.md`, and `/docs/aqevia-admin.md`).
- `./scripts/test.sh` consistently drives the fmt → clippy → test sequence without manual steps, and its logs are mentionable from `/docs/testing.md` as the preferred maintenance command.

## Milestone 2 — Storage plus observability contracts

### Goals
- Implement a modular storage interface with SQLite as the first backend and document schema/persistence rules per `/docs/database.md` while ensuring the Engine controls flush cadence.
- Provide minimal observability (health/status) HTTP handlers so the `/docs/engine/observability-api.md` contract becomes real and monitorable.

### Acceptance criteria
- The Engine builds the `aqevia-storage` controller + `aqevia-storage-sqlite` backend, runs stone migrations (`schema_meta`, `world_records`), buffers dirty records, and exposes `PERSIST_FLUSH_INTERVAL_MS`/`PERSIST_BATCH_CAPACITY` settings for the flush cadence documented in `/docs/database.md`.
- Observability endpoints (`/health`, `/ready`, `/status`) exist inside the transport layer, honor `/docs/engine/http-conventions.md`, and return the documented JSON shapes that include version, uptime, storage backend, and flush stats.
- Storage and observability code lives in dedicated crates and modules so future backends and transports can swap in while the Engine remains the only piece deciding when to persist state.

## Milestone 3 — Control plane and data plane contracts

### Goals
- Implement the HTTP control-plane contracts (`/docs/engine/http-conventions.md`, `/docs/engine/builder-api.md`, `/docs/engine/admin-api.md`, `/docs/engine/ai-builder.md`, `/docs/engine/ai-providers.md`) and the WebSocket data-plane contract (`/docs/engine/ws-session.md`, `/docs/engine/protocol.md`, `/docs/engine/ai-runtime.md`).
- Enforce auth/roles, AI rules (Assist vs Runtime), and the documented endpoint set for Builder/Admin while keeping AI suggestions non-mutating until approved.

### Acceptance criteria
- Builder/Admin HTTP APIs exist and return the documented response shapes, enforce roles, and log audit-worthy control-plane actions with inputs validated per `/docs/security.md`.
- WS endpoint accepts commands, sequences outputs (type/seq), handles keepalive, and obeys the `data plane = WS traffic` rule; client messages trigger Kernel-authoritative effects only.
- AI Assist endpoints return drafts/proposals without applying them; AI Runtime runs asynchronously and streams safe outputs per `/docs/engine/ai-runtime.md`; provider secrets remain server-side only.

## Milestone 4 — Embedded web UIs and shared API libraries

### Goals
- Deliver the three web apps (Client/Builder/Admin) under `ui/`, share API/auth/type helpers (`ui/shared/...`), and serve built assets from the Engine process so a single deployment exposes all UIs per `/docs/aqevia-client.md`, `/docs/aqevia-builder.md`, `/docs/aqevia-admin.md`.
- Provide shared API plumbing that honors `/docs/engine/protocol.md` and `/docs/engine/http-conventions.md` while keeping endpoint sets separated into `admin/`, `builder/`, and `client/`.

### Acceptance criteria
- Each web UI builds (via Vite or similar) and the Engine serves the static assets at stable routes (`/client`, `/builder`, `/admin`).
- Shared API/auth/components under `ui/shared/` document the split responsibilities; `/docs/milestones.md` and `/docs/style.md` note that the shared API core contains transport/auth/error plumbing while each endpoint set is in its own subfolder.
- Web UIs respect token constraints (no secrets in browser), connect to the Engine over WS/HTTP, and present the minimal gameplay/management flows described in `/docs/aqevia-client.md`, `/docs/aqevia-builder.md`, and `/docs/aqevia-admin.md`.

## Milestone 5 — Hardening, AI, and v1 release

### Goals
- Finalize AI Runtime + Assist, tighten docs/tests/observability, and prove the full stack (Engine + UIs + AI) meets the contracts so that v1 can be tagged.
- Document release readiness (container + native), run the canonical `./scripts/test.sh`, and ensure all `/docs/engine/*` files stay synchronized with implementation.

### Acceptance criteria
- All API/protocol docs under `/docs/engine/` match the implementation (confirmed by manual review or generated checks), and `docs/security.md`, `docs/testing.md`, and `docs/style.md` reflect the current behavior.
- Container and native build paths produce a runnable Engine serving all three web UIs; `./scripts/test.sh` succeeds, and observability endpoints report health.
- AI Assist/Runtime behave per `/docs/engine/ai-builder.md` and `/docs/engine/ai-runtime.md` (drafts only, async runtime responses), and provider secrets remain server-side.
- Tag and commit the release as `v1` (or equivalent) to mark final acceptance.
