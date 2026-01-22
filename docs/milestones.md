# Milestones

This document slices the Aqevia effort into Codex-sized deliverables. Each milestone or sub-milestone is intended to be achievable in one pass: it names affected files, narrates what the work is, lists concrete acceptance criteria, and specifies how to verify it locally without wandering into unrelated code. Follow the dependencies so work flows from baseline docs into runtime behavior, storage, networking, UIs, and AI guardrails while always respecting the **1 World = 1 deployment unit** rule, Kernel/Router/Transport boundaries, modular storage tenants, and AI safety notes outlined in `/AGENTS.md`.

## How to use these milestones with Codex

- Treat each sub-milestone as the entire prompt scope. Do not expand into other sections unless the dependency explicitly says “depends on this.”  
- When you pick a sub-milestone, edit only the files listed under **Files to touch** and stop once the **Acceptance criteria** are satisfied.  
- Always include the **Local verification** commands after implementation; reuse `./scripts/test.sh` when formatting/lint/testing work is required.  
- Maintain the unified **Aqevia Web UI** SPA intent whenever UIs, routing, or documentation surfaces are touched: `/`, `/client/*`, `/builder/*`, `/admin/*` all share one entry point.

## Phase 0 — Baseline documentation & workspace hygiene

### 0.1 Canonical docs & layout
- **Goal:** Prove the repo contains every `/docs/*` contract, UI placeholder, and alignment guide referenced in `/AGENTS.md`.
- **Scope:** In-scope: update doc list sections; out-of-scope: runtime code.  
- **Files to touch:** `docs/milestones.md`, `docs/testing.md`, `docs/style.md`, `docs/aqevia-engine.md`, `ui/README.md`, `ui/*/README.md`.  
- **Acceptance criteria:** Each `/docs/*` contract from `/AGENTS.md` is mentioned in one of the baseline docs; every `ui/*` directory has a README stub; `docs/testing.md` and `docs/style.md` cite `./scripts/test.sh` as the canonical pipeline.  
- **Local verification:** `./scripts/test.sh` (documented as canonical) and `ls docs`; expected success (docs only).  
- **Depends on:** none.

### 0.2 Formatting, linting, and version automation
- **Goal:** Ensure formatting standards, version tracking, and support scripts exist before runtime work.
- **Scope:** In-scope: verifying `.editorconfig`, `VERSION`, `scripts/test.sh`, `scripts/check-version-sync`, and `scripts/version-locations.yml`; out-of-scope: altering runtime crates.  
- **Files to touch:** `.editorconfig`, `VERSION`, `scripts/test.sh`, `scripts/check-version-sync`, `scripts/version-locations.yml`.  
- **Acceptance criteria:** `.editorconfig` documents indent/line rules, `scripts/test.sh` runs `cargo fmt`, `cargo clippy`, `cargo test`; version helper scripts exist and mention `VERSION`.  
- **Local verification:** `./scripts/test.sh` should still execute (even if later commands fail); `./scripts/check-version-sync` exits normally.  
- **Depends on:** 0.1.

## Phase 1 — Engine workspace foundations

### 1.1 Workspace manifest & boundaries
- **Goal:** Define the Rust workspace under `src/` with Kernel/Router/Transport crates while stating single-world hosting constraints.  
- **Scope:** In-scope: workspace `Cargo.toml`, documentation snippets that describe constraint; out-of-scope: crate implementation changes.  
- **Files to touch:** `src/Cargo.toml`, `docs/aqevia-engine.md`, `README.md`.  
- **Acceptance criteria:** `src/Cargo.toml` lists `kernel`, `router`, `transport`, `engine`, `storage`, `storage-sqlite`, `bin/aqevia-engine`; docs emphasize Kernel/Router/Transport boundaries and “1 World = 1 deployment unit”.  
- **Local verification:** `cargo metadata --no-deps` (run from repo root) succeeds to prove workspace structure (record command output in notes).  
- **Depends on:** 0.2.

### 1.2 Engine entrypoint & env bootstrap (doc stage)
- **Goal:** Document the minimal config flow (env vars `AQEVIA_SQLITE_PATH`, `PERSIST_*`, `AQEVIA_OBSERVABILITY_ADDR`) and enforce serial boot before runtime code exists.  
- **Scope:** In-scope: note the env vars and their defaults; out-of-scope: implementing them.  
- **Files to touch:** `docs/engine/http-conventions.md`, `docs/database.md`, `docs/docker.md`.  
- **Acceptance criteria:** Each document mentions the env vars and shows defaults; `docs/docker.md` reiterates single-world constraint and observability port.  
- **Local verification:** None (docs only).  
- **Depends on:** 1.1.

## Phase 2 — Storage + persistence control-plane

### 2.1 Storage interface contract
- **Goal:** Define `StorageBackend`, `WorldRecord`, and `StorageController` expectations plus dirty-tracking responsibilities.  
- **Scope:** In-scope: docs describing `StorageBackend` trait obligations and flush cadence (Engine owns when, storage owns how); out-of-scope: actual backend implementation.  
- **Files to touch:** `docs/database.md`, `docs/engine/protocol.md`.  
- **Acceptance criteria:** `docs/database.md` explains trait methods, `StorageConfig`, and flush stats; `docs/engine/protocol.md` references storage state in observability snapshot.  
- **Local verification:** None (docs only).  
- **Depends on:** 1.2.

### 2.2 SQLite backend schema + bootstrap (dev reset on schema change)
- **Goal:** Describe schema (`schema_meta`, `world_records`) and the dev bootstrap behavior (schema version stamp + reset-on-mismatch).  
- **Scope:** In-scope: schema definition (columns/types) and bootstrap semantics; out-of-scope: tests and any data-preserving upgrades.  
- **Files to touch:** `docs/database.md`, `docs/style.md` (if needed).  
- **Acceptance criteria:** Schema is spelled out (columns/types). `schema_meta` exists and stores a schema version. On version mismatch in dev, the DB is reset and re-initialized (no retention, no upgrade path).  
- **Local verification:** None (docs only).  
- **Depends on:** 2.1.

### 2.3 Persistence flush policy
- **Goal:** Clarify flush interval/capacity config, dirty batching behavior, and shutdown flush expectations in docs.  
- **Scope:** In-scope: updates to `docs/database.md` and `docs/testing.md`; out-of-scope: code.  
- **Acceptance criteria:** `docs/database.md` lists `PERSIST_FLUSH_INTERVAL_MS`, `PERSIST_BATCH_CAPACITY`, and explains their tunability; `docs/testing.md` mentions storage tests around batching.  
- **Files to touch:** `docs/database.md`, `docs/testing.md`.  
- **Local verification:** `./scripts/test.sh` (documented command) still passes after doc edits.  
- **Depends on:** 2.2.

### 2.4 Observability endpoints
- **Goal:** Document `/health`, `/ready`, `/status` shapes, caching headers, and their location in the transport layer.  
- **Scope:** In-scope: `docs/engine/observability-api.md`, `docs/engine/http-conventions.md`, `docs/security.md`; out-of-scope: server code.  
- **Acceptance criteria:** Observability doc includes status JSON, readiness text referencing storage readiness, and `/docs/security.md` notes the default observability port/guardrails.  
- **Local verification:** None (docs only).  
- **Depends on:** 2.3.

## Phase 3 — Control-plane HTTP scaffolding

### 3.1 Transport + observability server stub
- **Goal:** Confirm `aqevia-transport` exposes HTTP endpoints for health/ready/status and is wired into the Engine entrypoint (docs describe this).  
- **Files to touch:** `docs/engine/http-conventions.md`, `docs/engine/protocol.md`, `docs/milestones.md` (phase references).  
- **Acceptance criteria:** Docs mention `aqevia-transport` handles observability endpoints and respects HTTP conventions; `docs/milestones.md` notes transport responsibility.  
- **Local verification:** `cargo test --lib -p aqevia-transport` (or `./scripts/test.sh` to run entire suite).  
- **Depends on:** 2.4.

### 3.2 Builder API first endpoint
- **Goal:** Incrementally describe (and later implement) one Builder HTTP endpoint group (e.g., rooms CRUD) per `/docs/engine/builder-api.md`.  
- **Scope:** In-scope: doc updates describing the endpoint, required request/responses; out-of-scope: Admin or AI endpoints.  
- **Files to touch:** `docs/engine/builder-api.md`, `docs/milestones.md`.  
- **Acceptance criteria:** The doc lists a rooms endpoint (path, method, auth rule), sample payload/response, and references role gating under `/builder/*`.  
- **Local verification:** `rg "/builder" docs/engine/builder-api.md` shows entry exists.  
- **Depends on:** 3.1.

### 3.3 Admin API starter
- **Goal:** Document one Admin endpoint (e.g., `/admin/worlds` overview) aligned with `/docs/engine/admin-api.md`.  
- **Files to touch:** `docs/engine/admin-api.md`, `docs/milestones.md`.  
- **Acceptance criteria:** Admin doc captures path, method, required operator role, and expected JSON response; ensures role gating uses `/admin/*` and notes SPA behavior.  
- **Local verification:** `rg "/admin" docs/engine/admin-api.md`.  
- **Depends on:** 3.1.

### 3.4 AI Assist draft contract
- **Goal:** Outline `/docs/engine/ai-builder.md` describing a draft-generation endpoint that returns proposals only.  
- **Files to touch:** `docs/engine/ai-builder.md`, `docs/milestones.md`.  
- **Acceptance criteria:** Endpoint description includes request payload shape, response draft fields, “AI suggests, Builder decides”, and reference to `AI Provider` secrets staying server-side.  
- **Local verification:** `rg "AI Assist" docs/engine/ai-builder.md`.  
- **Depends on:** 3.2.

## Phase 4 — Data-plane WebSocket + gameplay

### 4.1 WS handshake & sequencing
- **Goal:** Define the WebSocket session handshake, message envelope, and sequence number semantics from `/docs/engine/ws-session.md`.  
- **Scope:** Doc updates only.  
- **Acceptance criteria:** Doc outlines handshake (init payload, acknowledgements), sequence numbering, referencing Router-delivered sessions and Kernel authority.  
- **Files to touch:** `docs/engine/ws-session.md`, `docs/engine/protocol.md`.  
- **Local verification:** `rg "sequence" docs/engine/ws-session.md`.  
- **Depends on:** 3.2.

### 4.2 Keepalive/backpressure expectations
- **Goal:** Document keepalive ping/pong, reconnect behavior, and backpressure rules.  
- **Files to touch:** `docs/engine/ws-session.md`, `docs/testing.md`.  
- **Acceptance criteria:** Sections describing keepalive intervals, backpressure handling, and tests covering session delivery.  
- **Local verification:** `rg "keepalive" docs/engine/ws-session.md`.  
- **Depends on:** 4.1.

### 4.3 Minimal gameplay command flow
- **Goal:** Describe data-plane command handling (e.g., `look`, `say`), Kernel authority, and Router delivery.  
- **Files to touch:** `docs/engine/protocol.md`, `docs/aqevia-client.md`.  
- **Acceptance criteria:** `docs/engine/protocol.md` lists command types and Kernel responsibility; `docs/aqevia-client.md` references data-plane WebSocket connections.  
- **Local verification:** `rg "look" docs/engine/protocol.md` or similar.  
- **Depends on:** 4.2.

## Phase 5 — Embedded Aqevia Web UI delivery

### 5.1 Static SPA hosting
- **Goal:** Document that the Engine serves one SPA entrypoint at `/`, rewrites `/client`, `/builder`, `/admin`, and role-gates views.  
- **Files to touch:** `docs/spec.md`, `docs/engine/http-conventions.md`, `docs/milestones.md`.  
- **Acceptance criteria:** Spec states SPA routing, HTTP doc mentions `/client/*` rewrites, milestone plan references SPA hosting; there are no docs implying separate hosts.  
- **Local verification:** `rg "SPA" docs/spec.md`.  
- **Depends on:** 3.4.

### 5.2 Shared UI libraries
- **Goal:** Introduce doc placeholders for shared API/auth/helper packages (`ui/shared`).  
- **Files to touch:** `ui/shared/README.md`, `docs/milestones.md`.  
- **Acceptance criteria:** README describes shared transport/auth components split by `admin/`, `builder/`, `client/`.  
- **Local verification:** `rg "shared" ui/shared/README.md`.  
- **Depends on:** 5.1.

### 5.3 Client area basics
- **Goal:** Doc the `/client/*` module (connection screen, status, gameplay feed) staying within SPA.  
- **Files to touch:** `docs/aqevia-client.md`, `docs/spec.md`.  
- **Acceptance criteria:** Document mentions `/client/login` deep-linking, WebSocket usage, and in-SPA navigation.  
- **Local verification:** `rg "/client" docs/aqevia-client.md`.  
- **Depends on:** 5.1.

### 5.4 Builder area basics
- **Goal:** Document `/builder/*` editing flows, role checks, and control-plane API calls.  
- **Files to touch:** `docs/aqevia-builder.md`, `docs/engine/builder-api.md`.  
- **Acceptance criteria:** Builder doc narrates role gating, deep-link routing, and connection to builder APIs.  
- **Local verification:** `rg "/builder" docs/aqevia-builder.md`.  
- **Depends on:** 5.2, 5.3.

### 5.5 Admin area basics
- **Goal:** Document `/admin/*` observability/dashboard views, session controls, and moderation hooks.  
- **Files to touch:** `docs/aqevia-admin.md`, `docs/engine/admin-api.md`.  
- **Acceptance criteria:** Admin doc references SPA hosting, observability + moderation APIs, and controls staying on same host.  
- **Local verification:** `rg "/admin" docs/aqevia-admin.md`.  
- **Depends on:** 5.1, 5.4.

## Phase 6 — AI provider & runtime guardrails

### 6.1 AI provider abstraction
- **Goal:** Describe the pluggable `AI Provider` interface (local/cloud, secrets always server-side, timeouts/retries).  
- **Files to touch:** `docs/engine/ai-providers.md`, `docs/security.md`.  
- **Acceptance criteria:** Document lists provider capabilities, streaming flags, and secret handling.  
- **Local verification:** `rg "AI Provider" docs/engine/ai-providers.md`.  
- **Depends on:** 3.4.

### 6.2 AI Assist drafts
- **Goal:** Document that AI Assist only drafts builder content and references control-plane API application flow.  
- **Files to touch:** `docs/engine/ai-builder.md`, `docs/spec.md`.  
- **Acceptance criteria:** Clarify AI Assist drafts, approval process, and mention in SPA Builder area.  
- **Local verification:** `rg "AI Assist" docs/engine/ai-builder.md`.  
- **Depends on:** 6.1, 5.4.

### 6.3 AI Runtime queue
- **Goal:** Outline async narrative assistant behavior (job queue, non-blocking Kernel, Router delivering results).  
- **Files to touch:** `docs/engine/ai-runtime.md`, `docs/engine/protocol.md`.  
- **Acceptance criteria:** Doc describes async jobs, guardrails, and how Router/data plane surfaces results without blocking the Kernel.  
- **Local verification:** `rg "AI Runtime" docs/engine/ai-runtime.md`.  
- **Depends on:** 4.3, 6.1.

## Phase 7 — Hardening & tests

-### 7.1 Storage + observability tests
- **Goal:** Document storage test expectations (dirty batching, schema bootstrap) and observability health checks per `/docs/testing.md`.  
- **Files to touch:** `docs/testing.md`, `docs/database.md`.  
- **Acceptance criteria:** Testing doc lists storage/observability suites, referencing `aqevia-storage`, `aqevia-storage-sqlite`, and `aqevia-transport`.  
- **Local verification:** `./scripts/test.sh` (should pass once implementation catches up).  
- **Depends on:** 2.3, 2.4.

### 7.2 WS/auth integration tests
- **Goal:** Plan tests for WebSocket sessions, auth boundaries, and Router/Transport separation.  
- **Files to touch:** `docs/testing.md`, `docs/security.md`.  
- **Acceptance criteria:** Testing doc mentions WS session tests and auth guardrails; security doc references Kernel/Router/Transport boundaries for auth paths.  
- **Local verification:** `rg "Kernel" docs/testing.md`.  
- **Depends on:** 4.2, 4.3.

### 7.3 UI + API contract verification
- **Goal:** Ensure docs mention verifying SPA routes, shared APIs, and control-plane flows as part of testing.  
- **Files to touch:** `docs/testing.md`, `docs/spec.md`.  
- **Acceptance criteria:** Testing doc references UI contract smoke checks; spec reiterates unified SPA navigation.  
- **Local verification:** `rg "SPA" docs/testing.md`.  
- **Depends on:** 5.5.
