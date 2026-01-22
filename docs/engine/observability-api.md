# Observability API Contract
Captures health/status conventions for monitoring an Aqevia world runtime, including the readiness checks consumed by operators and the embedded HTTP/transport server provided by the Engine.

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
