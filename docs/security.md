# Security Considerations

Covers authentication expectations, secrets handling, input validation, and threat notes for Aqevia deployments.

## Configuration hygiene

- Keep `AQEVIA_SQLITE_PATH` pointing to a directory with tight permissions; the Engine stores durable state there, so unauthorized modifications or symlinks can corrupt a World’s authoritative data.

## Observability guardrails

- The observability listener (`AQEVIA_OBSERVABILITY_ADDR`, default `127.0.0.1:7878` but configurable to `0.0.0.0:7878` inside Docker) hosts `/health`, `/ready`, and `/status`. Bind it to loopback or a protected network by default and avoid routing it through public interfaces.
- `/status` surfaces version, uptime, storage backend, flush stats, and recent errors, so requesters must be trusted. If you expose it outside the host, place it behind a proxy/gateway that enforces auth (tokens, mTLS, IP allowlists).
- Keep caching disabled (`Cache-Control: no-store`) so probes always reflect the current readiness state rather than stale responses cached by intermediaries.
