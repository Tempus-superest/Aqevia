# Security Considerations
Covers authentication expectations, secrets handling, input validation, and threat notes for Aqevia deployments.

## Configuration hygiene

- Keep `AQEVIA_SQLITE_PATH` pointing to a directory with tight permissions; the Engine stores durable state there, so unauthorized modifications or symlinks can corrupt a World’s authoritative data.
- The observability port (`AQEVIA_OBSERVABILITY_ADDR`, default `127.0.0.1:7878`) exposes `/health`, `/ready`, and `/status`. Do not bind it to public interfaces without an authenticated proxy, especially since `/status` discloses version, uptime, and storage backend metadata.
