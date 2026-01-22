//! Engine crate: composes kernel, router, and transport layers into a single-world runner.

use aqevia_kernel::Kernel;
use aqevia_router::Router;
use aqevia_transport::Transport;

pub struct Engine {
    transport: Transport,
}

impl Engine {
    /// Build the engine by wiring the Kernel → Router → Transport pipeline for one World.
    pub fn new() -> Self {
        let kernel = Kernel::new();
        let router = Router::new(kernel);
        let transport = Transport::new(router);
        Engine { transport }
    }

    /// Execute a minimal tick while remaining single-world and deterministic.
    pub fn run_one_world(&self) -> String {
        self.transport.deliver("ready")
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Engine;

    #[test]
    fn engine_runs_single_world() {
        let engine = Engine::new();
        assert!(engine.run_one_world().contains("Delivered 'ready'"));
    }
}
