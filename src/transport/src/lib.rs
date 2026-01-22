//! Transport crate: responsible for WebSocket/HTTP plumbing without touching gameplay logic.

use aqevia_router::Router;

pub struct Transport {
    router: Router,
}

impl Transport {
    /// Compose transports over the provided router.
    pub fn new(router: Router) -> Self {
        Transport { router }
    }

    /// Deliver a payload via the router context.
    pub fn deliver(&self, payload: &str) -> String {
        format!(
            "Delivered '{}' into {}",
            payload,
            self.router.world_context()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aqevia_router::Router;

    #[test]
    fn transport_delivers_via_router_context() {
        let router = Router::default();
        let transport = Transport::new(router);
        let output = transport.deliver("ping");
        assert!(output.contains("Delivered 'ping'"), "output was {}", output);
    }
}
