# Observability API Contract
Captures health/status conventions for monitoring an Aqevia world runtime, including the readiness checks consumed by operators and the embedded HTTP/transport server provided by the Engine.
## Observability transport endpoints

`aqevia-transport` owns the `/health`, `/ready`, and `/status` HTTP endpoints used by operators. These endpoints run alongside the unified SPA hosting and share the `Cache-Control: no-store` header plus JSON/text conventions described in `docs/engine/http-conventions.md`.

### `GET /health`

- Purpose: quick liveness check that the process is running.
- Response: `200 OK` with `{ "status": "ok" }` or a short text body; header `Content-Type: application/json`, `Cache-Control: no-store`.
- No body means the health probe just ensures the transport listener is alive.

### `GET /ready`

- Purpose: readiness gate verifying the Kernel, Router, and storage are initialized so gameplay/control-plane traffic may flow.
- Response: `200 OK` with `{ "status": "ready" }` once:
  - the storage backend completed `StorageBackend::init` and wrote the current `schema_meta` version,
  - the Router has bound its session listeners.
- Prior to readiness the endpoint returns `503 Service Unavailable` with `{"status":"initializing"}`; headers `Content-Type: application/json`, `Cache-Control: no-store`.

### `GET /status`

- Purpose: diagnostic snapshot for both humans and automation; rate-limit these endpoints to avoid overload (a few requests/sec per instance).
- Response: `200 OK`, `Content-Type: application/json`, `Cache-Control: no-store`, with a payload like:
  ```json
  {
    "version":"0.2.4",
    "uptime_ms":123456,
    "storage_backend":"sqlite",
    "storage_ready":true,
    "dirty_count":5,
    "last_flush":"2025-12-31T12:34:56Z",
    "last_flush_error":null,
    "ws_sessions":2
  }
  ```
  where `ws_sessions` is optional if exposure is supported elsewhere; the key storage fields mirror the stats defined in `docs/database.md`.
- Avoid exposing `/status` publicly without a trusted proxy or auth guard because it discloses version, uptime, and flush/error state.
All endpoints return JSON with `Content-Type: application/json` and `Cache-Control: no-store` headers per `/docs/engine/http-conventions.md`.

## GET /health

- Fast `/health` probe that confirms the process is alive. It does not depend on storage readiness.
- Response:
  ```json
  {"status":"ok"}
  ```
- Always returns `200 OK`.

## GET /ready

- Indicates whether the Engine has completed storage initialization/migrations and is ready to accept connections.
- Returns `200 OK` once `aqevia-storage-sqlite` has run migrations and `StorageController` has marked the backend ready.
- Returns `503 Service Unavailable` with payload `{"status":"initializing"}` until readiness is achieved.

## GET /status

- Returns a snapshot that includes:
  - `version`: engine version (mirrors `/VERSION`).
  - `world_id`: the single World that this Engine hosts.
  - `storage_backend`: the backend name (e.g., `sqlite`).
  - `storage_ready`: whether persistent storage is initialized.
  - `flush_count`: how many batch flushes have completed.
  - `last_flush_at`: UNIX timestamp of the latest flush (optional).
  - `uptime_seconds`: how long the binary has been running.
  - `storage_error`: last storage error, if any.
- Sample response:
  ```json
  {
    "version":"0.2.0",
    "world_id":"aqevia-default-world",
    "storage_backend":"sqlite",
    "storage_ready":true,
    "flush_count":3,
    "last_flush_at":1674000000,
    "uptime_seconds":120,
    "storage_error":null
  }
  ```
