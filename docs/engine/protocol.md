# Engine Protocol Overview
Entrypoint describing Engine contracts across WS data plane, HTTP control plane, AI integration, and observability surfaces.

## Embedded Web UI hosting

The Router also serves the **Aqevia Web UI** static SPA from `/`. This includes the role-gated `/client/*`, `/builder/*`, and `/admin/*` areas, all pointing to the same entry bundle so the browser can bootstrap the SPA even when landing on a deep link. The SPA then fetches the authenticated session context via the control plane, and the WebSocket data plane described in this document can connect once the player area is active.

## Observability snapshot (storage state)

The `/status` payload includes storage state so operators understand how the backend is handling dirty tracking, even though the Engine owns the flush cadence:

- `storage_backend`: the storage implementation (`sqlite` for the default implementation).
- `storage_ready`: whether `StorageBackend::init` completed and schema metadata is stamped.
- `dirty_count`: number of buffered `WorldRecord` entries pending flush.
- `last_flush`: timestamp when the latest flush completed.
- `last_flush_error`: any error message surfaced by the backend during the most recent flush attempt.

These fields complement the storage stats documented in `/docs/database.md` and keep the observability view consistent with the shared HTTP conventions for `/health`, `/ready`, and `/status`.
