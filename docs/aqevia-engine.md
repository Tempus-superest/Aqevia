# Aqevia Engine

The Aqevia Engine is the authoritative server runtime for a single MUD World. It hosts the game kernel (world state, rules, timers, scripting) and a transport-agnostic communications layer that delivers real-time gameplay over WebSockets while exposing standard HTTP APIs for building content, administration, AI-assist drafting, and observability. The Engine also embeds and serves the Aqevia Client/Builder/Admin web UIs as compiled static assets so a single local run provides the full experience.

## Reference Files

- `/docs/engine/protocol.md` — entrypoint for Engine contracts (WS data plane, HTTP control plane, AI integration, observability).
- `/docs/engine/ws-session.md` — WebSocket gameplay session protocol.
- `/docs/engine/http-conventions.md` — shared HTTP conventions (auth, errors, pagination, idempotency, rate limits).
- `/docs/engine/builder-api.md` — Builder HTTP API contract (authoritative content mutations).
- `/docs/engine/admin-api.md` — Admin HTTP API contract (world management and moderation).
- `/docs/engine/observability-api.md` — health/metrics/status/log access conventions.
- `/docs/engine/ai-api.md` — AI Assist HTTP endpoints (draft/proposal generation only).
- `/docs/engine/ai-runtime.md` — runtime AI narrative assistant model (async jobs, streaming output, guardrails).
- `/docs/engine/ai-providers.md` — AI Provider abstraction (local/cloud), secrets, timeouts, retries, streaming capability flags.

## Notes

- One Engine instance runs exactly one World (typically one container/process per World).
- Gameplay is WS (data plane); builder/admin/AI/observability are HTTP (control plane).
- Contract details live in the Engine leaf docs referenced below.