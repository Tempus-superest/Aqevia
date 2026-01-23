# Aqevia Engine Overview
Summarizes the server runtime design and boundaries of Aqevia-Engine, covering Kernel authority, Router responsibilities, transport adapters, and deployment expectations.

## Architecture boundaries

- **Kernel**: authoritative game simulation only; it does not perform network or direct database I/O, it only runs world rules and mutates in-memory state before handing snapshots to storage.
- **Router**: manages sessions and delivery, forwarding commands to the Kernel and streaming output to WebSocket clients without implementing gameplay rules.
- **Transport adapters**: provide HTTP/WebSocket surfaces (control-plane observability, Builder/Admin APIs, data-plane gameplay) and respect the Kernel/Router boundaries while never embedding game logic.

These boundaries enforce that the Engine hosts exactly one World per process/container, consistent with the “1 World = 1 deployment unit” rule described in `/AGENTS.md`.

## Docs index

The canonical `/docs/*` contract list appears in `docs/milestones.md#docs-index`. Use it as the baseline whenever this overview touches architecture, transport, AI, UI, persistence, or observability contracts so the Engine narrative stays synchronized with the rest of the docs.
