# Testing Strategy

This document defines the **testing philosophy and expectations** for **Aqevia**. It is intentionally **test-type focused** (what kinds of tests we expect to exist and why), not a per-milestone checklist.
Specific per-milestone testing requirements live in `milestones.md`.

## Goals

Aqevia testing must provide strong confidence that:

- **New features work as intended** (positive-path behavior).
- **Old features remain working** (regression protection).
- **Operator-visible behavior matches the spec** (WS protocol, HTTP APIs, Web UI, Docker).
- Failures are **actionable** (tests clearly explain what broke and why).

## How tests accumulate

- Tests are **additive**: each milestone adds tests for new behavior and keeps prior tests.
- `cargo test` should run **all accumulated Rust tests** so a passing run provides high confidence that:
  - the build is good, and
  - existing functionality has not regressed.

We expect thorough coverage over time: **if a feature exists, it should be tested** at an appropriate layer.

## Common local test commands

These commands are intended as stable entrypoints. If the repo later adds wrappers (e.g., `make`, `just`, or `scripts/*`), keep this section updated so there is a single obvious way to run checks locally.
Use `./scripts/test.sh` as the canonical local command; it runs the commands listed below.

- `./scripts/test.sh` _(preferred entrypoint; runs the commands below)_
- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -D warnings`
- `cargo test`

If/when the Web UI gains automated tests, add the corresponding command here (e.g., `npm test` / `pnpm test` in the UI directory) and ensure it runs offline.

## Storage and observability tests

- `aqevia-storage` exposes deterministic unit tests that verify dirty batching, flush cadence, and migration awareness. These tests mostly run in-memory or against a temporary SQLite file (`target/` is excluded via `.gitignore`), so they remain offline and fast.
- `aqevia-storage-sqlite` tests confirm migrations create `schema_meta`/`world_records`, and `StorageController` flushes data only when capacity or time demands it.
- `aqevia-transport` contains observability endpoint tests that spin up the in-process HTTP listener and hit `/health`, `/ready`, and `/status` via a raw `TcpStream`.

When running `./scripts/test.sh`, the storage and observability suites execute as part of `cargo test --all`.

## Engine boundary guardrails (Kernel / Router / Transport)

Aqevia’s architecture has hard boundaries:

- **Kernel**: authoritative simulation only; no network I/O and no direct DB I/O.
- **Router**: sessions + delivery; no gameplay rules or world-state logic.
- **Transport adapters**: WS/HTTP I/O only; no gameplay rules.

Tests and CI should make it difficult to accidentally violate these boundaries.

Recommended guardrails (implemented as tests or CI checks as the codebase grows):

- Module/crate dependency structure prevents Kernel from importing network or storage adapters.
- Router/Transport tests ensure protocol wiring does not reach into Kernel internals except via defined interfaces.

## Planes and contract surfaces

Aqevia has two primary operator-visible contract surfaces:

- **Data plane:** WebSocket gameplay sessions (commands in, output/events out).
- **Control plane:** HTTP APIs for Builder/Admin actions, AI Assist drafts/proposals, and observability.

Most contract tests should exercise these surfaces at the lowest layer that still validates the requirement.

## Test layers

Aqevia uses a layered approach. Most behavior should be tested at the **lowest layer** that still validates the requirement, and supplemented with higher-layer tests for operator-visible guarantees.

### 1) Unit tests

Purpose:
- Validate pure logic and invariants (parsing, validation rules, deterministic helpers).

Characteristics:
- Fast, deterministic, minimal IO.
- Should run everywhere and always.

### 2) Component tests

Purpose:
- Validate a subsystem with real collaborators, but without full networking or containers.

Examples:
- Storage module behavior (SQLite schema/migrations, queries, transactions).
- Config parsing/validation and “apply” semantics.
- Dirty-tracking and bounded batch flush behavior (when introduced).

Characteristics:
- May use temporary directories, SQLite files, and real serialization formats.
- Still fast and deterministic.

### 3) In-process HTTP tests (control-plane handler tests)

Purpose:
- Validate HTTP API contracts at the handler boundary without binding real sockets.

Examples:
- Auth requirements and role enforcement.
- Status/health endpoints.
- Input validation produces correct error codes and shapes.

Characteristics:
- No real TLS, no real network stack.
- Focused on request/response semantics.

### 4) In-process WS tests (data-plane session wiring)

Purpose:
- Validate WebSocket session semantics without binding real sockets, when possible.

Examples:
- Message shape validation (reject invalid frames/messages).
- Session lifecycle: connect → authenticate/identify (if applicable) → command → output.
- Backpressure / buffering rules at the Router boundary.

Characteristics:
- Tests should validate the **Router ↔ Kernel** integration via the defined interfaces.
- Avoid “fake” gameplay logic embedded in Transport tests; the Kernel remains authoritative.

### 5) Host-network integration tests (real sockets)

Purpose:
- Validate operator-visible network behavior on the host OS.

Examples:
- Control-plane HTTP server binds successfully on an ephemeral port.
- WebSocket server accepts connections and enforces the documented session flow.
- A simple command round-trip works via a real WS client.

Characteristics:
- Starts servers on **ephemeral ports**.
- Must be written to avoid port conflicts and flakiness.

### 6) CLI integration tests (spawn the binary)

Purpose:
- Validate the CLI (if/when present) as a product surface.

Examples:
- `aqevia status` or equivalent health/diagnostic commands.
- Exit codes, stdout/stderr content, and error messages.

Characteristics:
- Spawns the built binary and asserts observable behavior.
- Uses temporary state directories and ephemeral ports.

### 7) Docker smoke / end-to-end tests

Purpose:
- Validate the container story (image build/run, volumes, ports, and basic workflows).

Examples:
- `docker compose up` brings the system healthy.
- Health endpoint reachable.
- Basic control-plane API reachability inside Docker.
- Basic data-plane WebSocket connectivity and command round-trip inside Docker.

Characteristics:
- Slower and more environmental.
- Typically run in CI as a dedicated job (not necessarily on every local `cargo test` run).
- Must be deterministic and produce clear diagnostics.

#### Dev/test tools in Docker image

If Docker smoke tests require small utilities (e.g., `curl`), keep them clearly scoped to smoke automation and diagnostics. If they become a security or size concern later, split into a dev-only image.

### 8) Web UI contract tests

Purpose:
- Ensure the Web UI stays a **thin client** over the control-plane APIs.

We prefer tests that validate:
- The UI only uses documented `/api/...` endpoints.
- UI pages render and basic workflows operate (minimal, stable automation).

Characteristics:
- May be “contract style” (ensuring expected API calls) or “smoke style” (page loads + a small workflow).
- Browser-level testing is allowed when needed, but keep it minimal and stable.

## What “good tests” look like

All tests should aim for:

- **Determinism:** no dependence on wall-clock time, external services, or random ordering.
- **Isolation:** use temp dirs and ephemeral ports; never rely on developer machines’ state.
- **Clear failures:** assertions include context (endpoint, config, command, output).
- **Minimal scope:** test the smallest surface that proves the requirement.
- **Security-aware coverage:** include negative tests (unauthorized, invalid input, forbidden actions).

## Coverage expectations

As features are implemented, tests should cover:

- **Positive paths:** expected normal behavior.
- **Negative paths:** invalid input, missing auth, forbidden access, bad configs.
- **Regression cases:** known past bugs should get a test.

A feature is not “done” until it has tests at the appropriate layer(s).

## Where tests live

- Rust tests live under:
  - `tests/` for integration tests (recommended for cross-module behavior),
  - and `src/**` `#[cfg(test)]` modules for unit tests.
- Docker/e2e scripts (when needed) live under a dedicated folder such as `scripts/` or `tests/e2e/`.
  - Prefer `scripts/` for operator-style smoke tests (e.g., Docker Compose bring-up + curl/ws assertions).

## Relationship to milestones

- `milestones.md` defines **which milestone requires which test coverage** and what must pass for completion.
- This document defines **how** we test and what types of tests we expect to exist over time.

## Manual acceptance checklist (early development)

Use this checklist when validating early operator-visible behavior before full automation exists:

- `docker compose up` starts the Engine and it becomes healthy.
- Control-plane health/status endpoint returns 200.
- A WebSocket client can connect and complete the documented session flow.
- A basic gameplay command round-trip works (command → output/events).
- Builder/Admin control-plane flows work for a minimal content edit (create/edit a room or entity) and the change is observable via the data plane.
- Web UI (when present) performs the same control-plane flows using the documented APIs (no direct provider calls from the browser).
