# Aqevia Spec
Ends-to-end system architecture, layout, and boundaries for the single-world Aqevia deployment, covering how Engine, Router, transports, and the embedded web UI interact across networking and persistence layers.

## Aqevia Web UI

The Engine serves a single embedded **Aqevia Web UI** SPA at `/`. This SPA hosts three role-gated areas—`/client/*`, `/builder/*`, and `/admin/*`—rather than running three independent websites. The Router returns the same SPA shell for any of these paths so the front-end can render the appropriate area based on client-side routing, authentication, and permissions.

A landing user can open `/`, authenticate, and see navigation that reveals links to the areas their role authorizes. Navigation between `/client`, `/builder`, and `/admin` is a seamless SPA transition; there is no need to “switch sites,” and bookmarks or deep links to any of those paths still return the SPA with the correct view once authorized.

## Routing expectations

The Router rewrites `/`, `/client/*`, `/builder/*`, and `/admin/*` into the same static entrypoint HTML so any browser navigation result delivers the SPA. This ensures the SPA can bootstrap, fetch the user’s session/role metadata from established control-plane APIs, and hydrate the correct module while the data-plane WebSocket traffic continues to target the Router on the configured gameplay endpoint.
