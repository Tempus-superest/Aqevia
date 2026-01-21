# Aqevia Codex Rules

This file defines the **core working rules** for Codex/agents contributing to Aqevia.

## References (/docs)

- `/docs/architecture.md` — high-level system layout and boundaries.
- `/docs/session-protocol.md` — message types, sequencing, and reconnect rules.
- `/docs/deployment.md` — container/world instance deployment notes.
- `/docs/database.md` — persistence model and schema conventions.

## Terminology

- **World Server**: the Rust process/container that runs exactly one game world.
- **Kernel**: the authoritative simulation engine for a world (rules, state, timers).
- **Router**: the in-process communications plane that bridges transports and the Kernel.
- **Transport Adapter**: protocol/network layer that connects clients to the Router (WebSocket first).
- **Session**: an authenticated connection context bound to a user and (optionally) a character.
- **Command**: a normalized client input delivered to the Kernel.
- **Event/Output**: server-push messages produced by the Kernel and delivered to a Session.

## Non-negotiables

- **1 instance = 1 game world.** One container/process runs one world runtime.
- **Rust-first world server.** Prefer Rust for safety and consistency.
- **Gameplay uses WebSockets.** Real-time sessions require server-push.
- **Infra routing is external.** DNS/LB/reverse proxy decides which world instance a client reaches.

## Architecture boundaries

- **Kernel** (authoritative simulation)
  - Owns world state + rules + timers.
  - Accepts normalized `Command` inputs.
  - Emits `Event/Output` messages addressed to sessions.
  - **Must not** depend on sockets, WS, HTTP, or client specifics.

- **Router** (in-process communications plane)
  - Session registry (session_id ↔ connection ↔ character).
  - Normalizes transport frames into `Command`.
  - Delivers kernel `Event/Output` to the correct session.
  - Rate limits + backpressure (bounded per-session outgoing queues).
  - Disconnect/reconnect handling.
  - **Must not** implement gameplay rules or track room membership.

- **Transport adapter** (initially WS)
  - Handles network I/O, auth handshake, frame encode/decode.
  - Translates frames ↔ Router messages.
  - Stays thin; no gameplay authority.

## Session protocol

- Text-first message envelope with explicit `type`.
- Client → server: `cmd` (text), optional client capabilities.
- Server → client: `out`, `prompt`, `sys`, optional structured `event`.
- Messages include `seq` for ordered delivery and resume via `last_seq`.