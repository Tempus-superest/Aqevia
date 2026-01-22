# HTTP Conventions
Covers shared HTTP conventions such as authentication, error handling, pagination, idempotency, and rate limits used across the Router’s control-plane APIs.

## JSON + caching defaults

- Control-plane and observability endpoints always return `Content-Type: application/json`.
- Responses supply `Cache-Control: no-store` so downstream caches or browsers do not retain state-sensitive payloads.
- When a handler cannot assemble a payload (missing storage, auth failure, etc.), return the appropriate HTTP code and include a body that explains the status in JSON.

## Observability responses

- `/health` always returns `200 OK` with `{"status":"ok"}`.
- `/ready` returns `200 OK` once storage has initialized, and `503 Service Unavailable` with `{"status":"initializing"}` prior to readiness.
- `/status` returns `200 OK` containing version, uptime, storage_backend, flush statistics, and any stored error state in JSON. A missing metric should be represented by `null`.
