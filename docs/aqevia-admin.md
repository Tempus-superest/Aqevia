# Aqevia Admin UX
Summarizes the operator-facing UX within the single **Aqevia Web UI** SPA, covering observability flows, moderation controls, and world management tools exposed under `/admin/*`.

## Unified SPA context

Aqevia Admin lives at `/admin/*` inside the same SPA that also contains the player and builder experiences. When the Router serves `/admin/metrics`, `/admin/moderation`, or any other admin route, it always delivers the same SPA shell so the front-end router can load the appropriate dashboard. Only authenticated operator roles see the admin navigation, and the SPA keeps the full session context so switching between admin, builder, and client areas never requires hitting a new hostname or port.

Deep links directly to admin routes hydrate the SPA, which then performs any necessary permission checks before rendering sensitive operator controls. The Router’s static hosting of `/admin/*` ensures that the SPA can intercept navigation events locally and redirect unauthorized users, while still keeping the control-plane APIs for admin actions consistent with the rest of the Engine’s HTTP surface.

## Operator workflows

Within `/admin/*`, the SPA orchestrates observability calls (health, status, logs) and management APIs (deploy, shutdown, moderation) as outlined in `docs/engine/admin-api.md`. The admin module also uses the unified session bootstrap to display information about the current world and the data-plane WebSocket sessions that the Router tracks, enabling operators to stay in the same cohesive UI while responding to alerts or performing governance tasks.
