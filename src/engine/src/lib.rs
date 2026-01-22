//! Engine crate: composes kernel, router, and transport layers into a single-world runner.

use std::sync::Arc;
use std::time::{Instant, SystemTime};

use aqevia_kernel::Kernel;
use aqevia_router::Router;
use aqevia_storage::{
    StorageBackend, StorageConfig, StorageController, StorageResult, WorldRecord,
};
use aqevia_transport::{ObservabilityState, Transport};

pub struct Engine<B: StorageBackend> {
    transport: Transport,
    storage: StorageController<B>,
    observability: Arc<ObservabilityState>,
    start: Instant,
    world_id: String,
}

impl<B: StorageBackend> Engine<B> {
    pub fn new(
        backend: B,
        config: StorageConfig,
        observability: Arc<ObservabilityState>,
    ) -> StorageResult<Self> {
        let kernel = Kernel::new();
        let router = Router::new(kernel);
        let world_id = router.world_context().to_string();
        let transport = Transport::new(router);
        let storage = StorageController::new(backend, config)?;
        observability.mark_storage_ready(true);
        let stats = storage.stats();
        observability.note_flush(stats.flush_count, stats.last_flush);
        Ok(Engine {
            transport,
            storage,
            observability,
            start: Instant::now(),
            world_id,
        })
    }

    pub fn run_one_world(&mut self, payload: &str) -> StorageResult<String> {
        let record = WorldRecord {
            world_id: self.world_id.clone(),
            payload: payload.to_string(),
            timestamp: SystemTime::now(),
        };
        self.storage.record(record);
        if self.storage.flush_if_due()? {
            let stats = self.storage.stats();
            self.observability
                .note_flush(stats.flush_count, stats.last_flush);
        }
        Ok(self.transport.deliver(payload))
    }

    pub fn flush_all(&mut self) -> StorageResult<()> {
        self.storage.flush_all()?;
        let stats = self.storage.stats();
        self.observability
            .note_flush(stats.flush_count, stats.last_flush);
        Ok(())
    }

    pub fn uptime_seconds(&self) -> u64 {
        self.start.elapsed().as_secs()
    }

    pub fn storage_backend_name(&self) -> &'static str {
        self.storage.backend_name()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aqevia_storage::{StorageBackend, StorageStats};

    #[derive(Default)]
    struct DummyBackend {
        stats: StorageStats,
        persisted: usize,
    }

    impl StorageBackend for DummyBackend {
        fn init(&mut self) -> StorageResult<()> {
            self.stats.flush_count = 0;
            self.stats.last_flush = None;
            Ok(())
        }

        fn persist_batch(&mut self, batch: &[WorldRecord]) -> StorageResult<()> {
            self.stats.flush_count += 1;
            self.stats.last_flush = Some(SystemTime::now());
            self.persisted += batch.len();
            Ok(())
        }

        fn stats(&self) -> StorageStats {
            self.stats
        }

        fn backend_name(&self) -> &'static str {
            "dummy"
        }
    }

    #[test]
    fn engine_records_and_delivers() {
        let state = Arc::new(ObservabilityState::new("0.2.0", "world", "dummy"));
        let mut engine = Engine::new(
            DummyBackend::default(),
            StorageConfig {
                flush_interval_ms: 1,
                batch_capacity: 1,
            },
            state,
        )
        .unwrap();
        let output = engine.run_one_world("look").unwrap();
        assert!(output.contains("Delivered 'look'"));
    }

    #[test]
    fn engine_flushes_collection() {
        let state = Arc::new(ObservabilityState::new("0.2.0", "world", "dummy"));
        let mut engine = Engine::new(
            DummyBackend::default(),
            StorageConfig {
                flush_interval_ms: 1,
                batch_capacity: 10,
            },
            state,
        )
        .unwrap();
        engine.run_one_world("say").unwrap();
        assert!(engine.flush_all().is_ok());
    }
}
