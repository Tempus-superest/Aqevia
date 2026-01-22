# Aqevia Client UX
Summarizes the player UI design and interaction flows for the player area of the **Aqevia Web UI** SPA.

## Unified SPA context

Aqevia Client lives inside the single SPA served at `/`. The SPA exposes three role-gated areas—`/client/*` for players, `/builder/*` for creators, and `/admin/*` for operators—with client-side routing deciding which screens to render once the user is authenticated and their roles are known. Directly opening `/client/login`, `/client/worlds`, or any other player deep link still returns the SPA shell so the browser never 404s before the JavaScript router resolves the view.

Once authenticated, the SPA exposes navigation that shows or hides player controls based on role membership. Switching between player, builder, or admin views happens within the SPA without reloading different hosts or ports; the Router simply keeps serving the same bundle while the router state and authorization context change.

## Gameplay UX expectations

The client area maintains the data-plane WebSocket connection described in `docs/engine/ws-session.md` and renders gameplay output, input fields, and status indicators. The SPA bootstraps by fetching the authenticated session context over the control plane, then opens the WebSocket so the player can send commands and receive streamed output. Even when the player follows deep links into client-specific screens, the SPA resumes those flows after authenticating and reconnecting, reusing the same shared assets and keeping the experience within the unified Aqevia Web UI.
