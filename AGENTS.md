# Aqevia

- **Codex is not authorized to edit this file without explicit instructions and must never make changes to AGENTS.md on its own**

**Aqevia** is a Rust-first platform for creating and hosting text-based MUD **Worlds**. Each deployed Aqevia Engine (native process or container) runs exactly one **World** and serves the embedded web UIs for that World. At the center is the **Aqevia Engine**, a server application that runs a single World with an authoritative game kernel and a transport-agnostic communications layer. Players interact with a World through the **Aqevia Client** over the **data plane**—WebSocket session traffic used for real-time gameplay (commands + server-push output)—so the Engine can push output as the world changes. World builders use **Aqevia Builder** and operators use **Aqevia Admin** over the **control plane**—HTTP APIs used for building content, administration, AI assist, and observability—to create rooms, items, NPCs, and scripts, and to monitor worlds and perform management actions; the **Aqevia Client**, **Aqevia Builder**, and **Aqevia Admin** are delivered as embedded web UIs served by the Engine so a single local run provides the full experience. These web UIs are delivered as a single, unified single-page application (SPA) with a shared App Shell at `/` and role-gated areas at `/admin/*`, `/builder/*`, and `/client/*` (never separate ports or hostnames). AI integration is a core feature of Aqevia, powered by an **AI Provider** (local or cloud) accessed via API calls: **AI Builder** in **Aqevia Builder** accelerates content creation by drafting room and area descriptions, generating ambient text and event scaffolding, and producing safe, reviewable script templates for human approval before applying via standard APIs, while **AI Runtime** in the **Aqevia Engine** can optionally act as a narrative assistant—enhancing NPC dialogue and generating dynamic flavor text asynchronously, bounded by the Engine’s rules and safety checks so the game remains consistent and secure.

## References (/docs)

- `/docs/spec.md` — end-to-end system architechture, layout and boundaries.
- `/docs/aqevia-engine.md` — Server runtime design and boundaries.
- `/docs/engine/protocol.md` — entrypoint for Engine contracts (WS data plane, HTTP control plane, AI integration, observability).
- `/docs/engine/ws-session.md` — WebSocket gameplay session protocol.
- `/docs/engine/http-conventions.md` — shared HTTP conventions (auth, errors, pagination, idempotency, rate limits).
- `/docs/engine/builder-api.md` — Builder HTTP API contract (authoritative content mutations).
- `/docs/engine/admin-api.md` — Admin HTTP API contract (world management and moderation).
- `/docs/engine/observability-api.md` — health/metrics/status/log access conventions.
- `/docs/engine/ai-builder.md` — AI Assist HTTP endpoints (draft/proposal generation only).
- `/docs/engine/ai-runtime.md` — runtime AI narrative assistant model (async jobs, streaming output, guardrails).
- `/docs/engine/ai-providers.md` — AI Provider abstraction (local/cloud), secrets, timeouts, retries, streaming capability flags.
- `/docs/aqevia-client.md` — Player UI design and interaction flows.
- `/docs/aqevia-builder.md` — World-building UI design and publishing workflow.
- `/docs/aqevia-admin.md` — Admin UI design and operator workflows.
- `/docs/database.md` — persistence model, schema conventions, migrations.
- `/docs/docker.md` - Docker deployment standards
- `/docs/testing.md` — test strategy and required coverage expectations.
- `/docs/versioning.md` — protocol/API versioning and compatibility rules.
- `/docs/security.md` — auth, secrets handling, input validation, threat notes.
- `/docs/style.md` — code style, module layout, naming, linting/formatting rules.
- `/docs/milestones.md` - Breaks Aqevia into milestone deliverables. Each milestone includes goals and acceptance criteria.
-- Add docker.md reference

## Terminology

Use these exact terms consistently across docs and code.

- **Aqevia**: The name of the applicaiton suite consisting of **Aqevia Engine**, **Aqevia Admin**, **Aqevia Builder** and **Aqevia Client**
- **World**: an isolated game universe (content + state + players) hosted by one **Aqevia Engine** instance.
- **Aqevia Engine**: the Rust server process that hosts exactly one **World**.
- **Aqevia Web UI**: the single-page web application served by the Engine at `/`, containing role-gated Admin/Builder/Client areas at `/admin/*`, `/builder/*`, `/client/*`.
- **Control plane**: HTTP APIs used for building content(**Aqevia Builder**), administration(**Aqevia Admin**), **AI Builder**, and observability.
- **Data plane**: WebSocket session traffic used for real-time gameplay (commands + server-push output).
- **Aqevia Client**: the player-facing web UI that connects to an **Aqevia Engine** over WebSocket.
- **Aqevia Admin**: the operations web UI that uses HTTP control-plane and observability APIs.
- **Aqevia Builder**: the **World** building web UI that uses HTTP control-plane APIs.
- **AI Provider**: a pluggable backend used for AI features (local or cloud) accessed via API calls.
- **AI Builder**: AI used by **Aqevia Builder** to draft content (rooms/areas, ambient text, script scaffolding) for human review before applying via standard APIs.
- **AI Runtime**: AI used by **Aqevia Engine** during gameplay as a narrative assistant (NPC dialogue and dynamic flavor text), executed asynchronously and bounded by engine rules and safety checks.

## Technology Stack

- **Core server**: **Aqevia Engine** (Rust)
- **Networking**: WebSocket gameplay sessions + HTTP control-plane APIs
- **Web apps**: TypeScript + React, built with Vite (served by the Engine)
- **Database**: SQLite (storage is modular; additional backends are added as new modules)
- **Deployment**: Runs natively or in a container; 1 World per deployment unit (no multi-world, no multi-instance per server)
- **AI**: Pluggable AI providers (local or cloud) accessed via API

## Core Rules

- **Development data is disposable.** Do not add migrations or retention logic. Each build initializes from scratch.
- **1 World = 1 deployment unit.** One Aqevia Engine (native process or container) runs exactly one **World** and serves the embedded Aqevia Client/Builder/Admin for that World.
- **Single-origin Web UI.** The Engine serves one unified SPA for the World at `/` with area routes `/admin/*`, `/builder/*`, `/client/*`; never serve Admin/Builder/Client on separate ports or hostnames.
- **No multi-world.** A single Engine process/container must never host more than one World.
- **No multi-instance per server.** A server must run at most one Engine instance; to host 5 Worlds, deploy 5 separate servers/containers.
- **Strict boundaries:**
  - **Kernel**: authoritative simulation only; never perform network I/O or direct DB I/O.
  - **Router**: sessions + delivery; never implement gameplay rules or world state logic.
  - **Transport adapters**: WS/HTTP I/O only; never implement gameplay rules.

## Interfaces

- **Data plane:** WebSocket sessions for gameplay commands and server-push output.
- **Control plane:** HTTP APIs for Builder/Admin actions, AI Assist, and observability.
- **Contracts live under `/docs/`.** When changing protocol/API behavior, update the matching doc.

## Persistence

- **Authoritative state is in memory.** The database is durable storage.
- **Storage is modular.** New database support is added by creating a new storage module.
- **SQLite is the initial backend.**
- **Engine owns when to persist** (flush cadence, shutdown flush). **Storage owns how** (schema, queries, transactions, migrations).
- Prefer **dirty tracking + bounded batch flush** over frequent full-state writes.

## AI

- **AI Assist (Builder/Admin):** generate drafts/proposals only; apply changes via standard control-plane APIs.
- **AI Runtime (Engine):** asynchronous narrative assistant; never block the Kernel waiting on AI.
- **AI suggests, Kernel decides.** AI output never mutates authoritative state without validation and explicit allowed actions.
- **Provider secrets stay server-side.** Never place cloud provider keys in browser code.
- **AI Provider** calls are server-side only (Engine); browsers never call providers directly.

## Codex Operating Rules

- **Assumptions:** do not make assumptions about application structure or intent. Clarify intent and update documentation
- **Dependencies:** do not add crates/packages or upgrade toolchains unless explicitly instructed.
- **Secrets:** never commit secrets, tokens, or private keys; use env vars/secret files and redact logs.
- **Scope:** make small, reviewable changes; avoid broad refactors unless explicitly requested.
- **Tests:** add/update tests for behavior changes; keep tests deterministic and offline.
- **Docs:** when changing a contract, update its doc in `/docs/*` and keep terminology consistent.
- **Quality:** keep lint/format clean; avoid cross-layer imports (Kernel must not import Router/Transport).
