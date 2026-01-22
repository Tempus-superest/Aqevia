# Engine Protocol Overview
Entrypoint describing Engine contracts across WS data plane, HTTP control plane, AI integration, and observability surfaces.

## Embedded Web UI hosting

The Router also serves the **Aqevia Web UI** static SPA from `/`. This includes the role-gated `/client/*`, `/builder/*`, and `/admin/*` areas, all pointing to the same entry bundle so the browser can bootstrap the SPA even when landing on a deep link. The SPA then fetches the authenticated session context via the control plane, and the WebSocket data plane described in this document can connect once the player area is active.
