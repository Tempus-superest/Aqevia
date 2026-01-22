//! Kernel crate: authoritative world state and simulation primitives.
//! It never performs network or direct database I/O.

/// Represents the single World that this Engine will host.
pub struct Kernel {
    world_id: &'static str,
}

impl Kernel {
    /// Create a new kernel instance for the default World.
    pub fn new() -> Self {
        Kernel {
            world_id: "aqevia-default-world",
        }
    }

    /// Identity of the single World managed by this kernel.
    pub fn world_id(&self) -> &'static str {
        self.world_id
    }
}

impl Default for Kernel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Kernel;

    #[test]
    fn kernel_tracks_world_id() {
        let kernel = Kernel::new();
        assert_eq!(kernel.world_id(), "aqevia-default-world");
    }
}
