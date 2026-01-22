# WebSocket Session Protocol
Defines the WebSocket gameplay session envelope, sequencing, backpressure, and reconnect expectations between clients and the Router.

## SPA clients

All WebSocket clients connect to the Router that also serves the **Aqevia Web UI** SPA. Whether the browser is in `/client/*`, `/builder/*`, or `/admin/*`, the WebSocket data plane shares the same host and port described in this document so gameplay traffic stays aligned with the single SPA surface.
