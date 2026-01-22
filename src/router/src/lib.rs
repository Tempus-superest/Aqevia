//! Router crate: handles session delivery boundaries without embedding gameplay rules.
//! It depends on the Kernel for world metadata but never performs network I/O itself.

use aqevia_kernel::Kernel;

pub struct Router {
    kernel: Kernel,
}

impl Router {
    /// Create a new router around the provided kernel.
    pub fn new(kernel: Kernel) -> Self {
        Router { kernel }
    }

    /// Route a command string to the kernel.
    pub fn route(&self, command: &str) -> String {
        format!("Routing '{}' for world {}", command, self.kernel.world_id())
    }

    /// Expose lightweight context for transports.
    pub fn world_context(&self) -> &'static str {
        self.kernel.world_id()
    }

    /// Create the default router tied to the default kernel.
    pub fn with_default_kernel() -> Self {
        Router::new(Kernel::new())
    }
}

impl Default for Router {
    fn default() -> Self {
        Router::with_default_kernel()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn router_routes_command_through_kernel() {
        let kernel = Kernel::new();
        let router = Router::new(kernel);
        let output = router.route("look");
        assert!(
            output.contains("Routing 'look'") && output.contains("aqevia-default-world"),
            "output was {}",
            output
        );
    }
}
