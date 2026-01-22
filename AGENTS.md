# Aqevia

Aqevia is a Rust-first platform for creating and hosting a single text-based MUD **World**. At the center is the **Aqevia Engine**, a server application that runs a single World with an authoritative game kernel and a transport-agnostic communications layer. Players interact with a World through the **Aqevia Client** over the **data plane**—WebSocket session traffic used for real-time gameplay (commands + server-push output)—so the Engine can push output as the world changes. World builders use **Aqevia Builder** and operators use **Aqevia Admin** over the **control plane**—HTTP APIs used for building content, administration, AI assist, and observability—to create rooms, items, NPCs, and scripts, and to monitor worlds and perform management actions; the **Aqevia Client**, **Aqevia Builder**, and **Aqevia Admin** are delivered as embedded web UIs served by the Engine so a single local run provides the full experience. AI integration is a core feature of Aqevia, powered by an **AI Provider** (local or cloud) accessed via API calls: **AI Builder** in **Aqevia Builder** accelerates content creation by drafting room and area descriptions, generating ambient text and event scaffolding, and producing safe, reviewable script templates for human approval before applying via standard APIs, while **AI Runtime** in the **Aqevia Engine** can optionally act as a narrative assistant—enhancing NPC dialogue and generating dynamic flavor text asynchronously, bounded by the Engine’s rules and safety checks so the game remains consistent and secure.

## References Files (/docs)

- `/docs/architecture.md` — end-to-end system layout and boundaries.
- `/docs/aqevia-engine.md` — Server runtime design and boundaries.
- `/docs/aqevia-client.md` — Player UI design and interaction flows.
- `/docs/aqevia-builder.md` — World-building UI design and publishing workflow.
- `/docs/aqevia-admin.md` — Admin UI design and operator workflows.
- `/docs/database.md` — persistence model, schema conventions, migrations.
- `/docs/testing.md` — test strategy and required coverage expectations.
- `/docs/versioning.md` — protocol/API versioning and compatibility rules.
- `/docs/security.md` — auth, secrets handling, input validation, threat notes.
- `/docs/style.md` — code style, module layout, naming, linting/formatting rules.

## Terminology

This terminology list must remain consistently used across all docuemnts and files when referencing these concepts

- **World**: an isolated game universe (content + state + players) hosted by one **Aqevia Engine** instance.
- **Aqevia Engine**: the Rust server process that hosts exactly one **World**.
- **Control plane**: HTTP APIs used for building content, administration, AI assist, and observability.
- **Data plane**: WebSocket session traffic used for real-time gameplay (commands + server-push output).
- **Aqevia Client**: the player-facing web UI that connects to an **Aqevia Engine** over WebSocket.
- **Aqevia Admin**: the operations web UI that uses HTTP control-plane and observability APIs.
- **Aqevia Builder**: the **World** building web UI that uses HTTP control-plane APIs.
- **AI Provider**: a pluggable backend used for AI features (local or cloud) accessed via API calls.
- **AI Builder**: AI used by **Aqevia Builder** to draft content (rooms/areas, ambient text, script scaffolding) for human review before applying via standard APIs.
- **AI Runtime**: AI used by **Aqevia Engine** during gameplay as a narrative assistant (NPC dialogue and dynamic flavor text), executed asynchronously and bounded by engine rules and safety checks.

## Technology Stack

- **Core server**: Aqevia Engine (Rust)
- **Networking**: WebSocket gameplay sessions + HTTP control-plane APIs
- **Web apps**: TypeScript + React, built with Vite (served by the Engine)
- **Database**: SQLite (storage is modular; additional backends are added as new modules)
- **AI**: Pluggable AI providers (local or cloud) accessed via API


### 1 world per instance

- **One server process = one world runtime**.
- Each world has its own DB (or schema/namespace) and can be deployed/restarted independently.
- Scalability is achieved by running more instances (containers), not by sharding inside a single monolith.

### Rust-first server

- The Aqevia Engine is written in **Rust** for memory safety, correctness, and long-term maintainability.
- The simulation kernel must not depend on networking details.


### WebSocket gameplay sessions

- Gameplay uses **WebSocket (WS)** connections to support:
  - server-push events (room chatter, combat ticks, timers)
  - ordered output delivery
  - backpressure handling
  - reconnect handling

- External infrastructure (DNS/LB/reverse proxy) may route clients to world instances.
- The Aqevia Engine should remain simple: **listen on a port and serve WS + basic health endpoints**.

### Web client stack

- Aqevia web apps (**Aqevia Client**, **Aqevia Builder**, **Aqevia Admin**) are built with **TypeScript + React**, using **Vite** for development and production builds.
- The Engine serves these web apps as embedded static assets so a single Aqevia Engine run provides the full local experience (Client/Builder/Admin + WS + HTTP APIs).
- At runtime, there is no Node/Vite toolchain; only the compiled static assets are served by the Engine.

## Communications plane (Router) concept

The Aqevia Engine contains an in-process communications plane, referred to as the **Router**.

### Responsibilities

- Maintain session registry (session_id ↔ connection ↔ character)
- Authenticate and bind sessions to accounts/characters
- Normalize inbound messages into `Command` messages
- Deliver kernel-generated `Event`/`Output` messages to the correct session
- Apply rate limits and anti-spam controls at the edge
- Manage backpressure (bounded outgoing queues per session)
- Handle disconnects/reconnects cleanly

### Non-responsibilities

- Do **not** implement game rules in the Router.
- Do **not** track room membership or gameplay state.
- Do **not** let the WS adapter become authoritative.

## Kernel concept

The **Kernel** is the authoritative simulation engine for a single world.

### Responsibilities

- Own and mutate world state (rooms, entities, items, combat, timers)
- Process `Command` messages and apply rules
- Emit `Event`/`Output` messages addressed to sessions
- Run scheduled/tick-driven logic (NPC actions, combat rounds, scripted timers)

### Non-responsibilities

- No direct socket/transport access
- No assumptions about client type (web vs native)

## Transport adapters

Initial transport adapter:

- **WS Adapter**: translates WebSocket frames ↔ Router messages.

Future adapters (optional):

- Native TCP/TLS adapter
- gRPC streaming adapter
- QUIC/WebTransport adapter

Design rule: **the session protocol and Router/Kernal interfaces must be transport-agnostic**.

## Session protocol

Define a minimal, versioned message envelope shared across transports.

### Client → Server

- `cmd`: player command input (text-first)
- Optional: client capabilities (terminal size, feature flags)

### Server → Client

- `out`: text output lines
- `prompt`: prompt updates
- `sys`: errors/system notices
- Optional: `event`: structured events for richer UI

### Sequencing and reconnect (planned)

- Include `seq` for ordered delivery and resume support (`last_seq`).

## External routing assumptions

- World selection can be implemented via:
  - per-world subdomains (recommended), or
  - a directory/lobby that returns a world WS URL.
- No in-app gameplay proxy is required to start.

## Agent working rules

### General

- Prefer small, reviewable changes.
- Keep boundaries strict: Kernel ↔ Router ↔ Transport.
- Add tests for command parsing, kernel logic, and protocol framing where feasible.

### Code style

- Prefer explicit types and clear module boundaries.
- Avoid cross-layer dependencies (no kernel importing transport code).

### Deliverables checklist

When implementing features, include:

- brief design note in PR/commit message
- unit tests where applicable
- basic docs updates (README or this AGENTS.md)