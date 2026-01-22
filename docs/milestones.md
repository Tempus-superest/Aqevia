# Milestones

This document breaks Aqevia into milestone deliverables. Each milestone includes goals and acceptance criteria, and the final milestone culminates in a v1 commit/release. The milestones track the repo’s actual state today—sparse tooling plus rich documentation—before progressing toward the fully implemented Engine, UIs, and AI-enabled experience described in `/AGENTS.md`.

## Repository baseline

- **Layout:** the workspace only contains docs (`/docs/`), placeholder `src/` + `ui/` folders with README stubs, `scripts/test.sh`, `VERSION` + versioning helpers, `ui/shared/...` scaffolds, and the top-level docs referenced by `/AGENTS.md`.
- **Build/test entry points:** there is no `Cargo.toml` or UI package yet, so `cargo`/`npm` commands cannot succeed; the only runnable automation is `./scripts/test.sh`, which today prints banners and fails fast when it cannot find a manifest. Format/lint is governed by `.editorconfig`, and `docs/testing.md` now points teams to the script as the canonical sequence (fmt → clippy → test).
- **Docs coverage:** every `/docs/*` contract listed in `/AGENTS.md` exists as a written file. `/docs/milestones.md` is the sole milestone tracker, and `/docs/style.md`, `/docs/testing.md`, `/docs/versioning.md`, `/docs/security.md`, etc., describe the architecture, contracts, and QA expectations.
- **Current implementation gaps:** no Rust workspace, no HTTP/WS server, no SQLite storage, and no packaged web UIs exist yet, so the repo must first establish foundational crates and UI packages before any of the core rules (1 World = 1 deployment unit, Kernel ↔ Router ↔ Transports boundaries, modular storage, AI plumbing) can be enforced in code.

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
- The Engine starts with a real SQLite schema from `/docs/database.md`, tracks dirty state, flushes in bounded batches, and expresses `PERSIST_FLUSH_INTERVAL_MS` (or similar) configuration.
- Observability endpoints (`/health`, `/ready`, `/status`, etc.) exist, return documented shapes, and can be exercised via curl/httpie, matching `/docs/engine/observability-api.md`.
- Storage module is isolated per the modular-storage rule; new backends add modules rather than rewriting core logic.

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
