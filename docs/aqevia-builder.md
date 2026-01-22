# Aqevia Builder UX
Captures the world-building UX within the **Aqevia Web UI** SPA, highlighting how creators navigate content tools, publishing workflows, and supporting APIs.

## Unified SPA context

Aqevia Builder is not a standalone website but the `/builder/*` module inside the single SPA served from `/`. After signing in, the SPA shows navigation tabs for builder flows only to users who possess the creator roles. Deep links such as `/builder/areas`, `/builder/scripts`, or `/builder/review` always receive the same HTML entry point from the Router so client-side routing can resolve the requested editing surface without a separate host or port.

Transitioning from a player or admin area into the builder area stays within the SPA; the Router keeps serving the static bundle while the front-end adjusts routing state, permissions, and queued API calls. This unified hosting model keeps asset caching and authentication consistent across areas, and creator-specific tools share the same global context (session, notifications, telemetry) as the rest of the web UI.

## Builder workflow reminders

Within `/builder/*`, the SPA loads the world definition, content graph, and contributor metadata from control-plane APIs (see `/docs/engine/builder-api.md`). Builder flows start by confirming role access, fetching the current world version, and rendering inline editors while the SPA manages drafts, publishing, and review states. All save/review actions invoke the control-plane endpoints while the SPA remains the surface for orchestrating those steps.
