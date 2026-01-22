//! Storage contract shared by all persistence backends.

use std::time::{Duration, Instant, SystemTime};

pub type StorageResult<T> = Result<T, StorageError>;

/// Configuration that controls persistence cadence and batching.
#[derive(Clone, Copy)]
pub struct StorageConfig {
    pub flush_interval_ms: u64,
    pub batch_capacity: usize,
}

impl Default for StorageConfig {
    fn default() -> Self {
        StorageConfig {
            flush_interval_ms: 500,
            batch_capacity: 20,
        }
    }
}

/// A single durable record describing the in-memory world state snapshot.
pub struct WorldRecord {
    pub world_id: String,
    pub payload: String,
    pub timestamp: SystemTime,
}

impl WorldRecord {
    pub fn summary(&self) -> String {
        let since_epoch = self
            .timestamp
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        format!("{}@{}", self.world_id, since_epoch)
    }
}

/// Statistics about persisted data.
#[derive(Default, Clone, Copy)]
pub struct StorageStats {
    pub flush_count: usize,
    pub last_flush: Option<SystemTime>,
}

/// Modular storage backend interface for durable persistence.
pub trait StorageBackend: Send {
    fn init(&mut self) -> StorageResult<()>;
    fn persist_batch(&mut self, batch: &[WorldRecord]) -> StorageResult<()>;
    fn stats(&self) -> StorageStats;
    fn backend_name(&self) -> &'static str;
}

/// Controller that drives flushing operations for a storage backend.
pub struct StorageController<B: StorageBackend> {
    backend: B,
    config: StorageConfig,
    pending: Vec<WorldRecord>,
    last_flush: Instant,
}

impl<B: StorageBackend> StorageController<B> {
    pub fn new(mut backend: B, config: StorageConfig) -> StorageResult<Self> {
        backend.init()?;
        Ok(StorageController {
            backend,
            config,
            pending: Vec::with_capacity(config.batch_capacity),
            last_flush: Instant::now(),
        })
    }

    pub fn record(&mut self, payload: WorldRecord) {
        self.pending.push(payload);
    }

    pub fn pending(&self) -> &[WorldRecord] {
        &self.pending
    }

    pub fn flush_if_due(&mut self) -> StorageResult<bool> {
        let since_last = self.last_flush.elapsed();
        let should_flush = !self.pending.is_empty()
            && (self.pending.len() >= self.config.batch_capacity
                || since_last >= Duration::from_millis(self.config.flush_interval_ms));

        if should_flush {
            self.flush_pending()
        } else {
            Ok(false)
        }
    }

    pub fn flush_pending(&mut self) -> StorageResult<bool> {
        if self.pending.is_empty() {
            return Ok(false);
        }

        self.backend.persist_batch(&self.pending)?;
        self.pending.clear();
        self.last_flush = Instant::now();
        Ok(true)
    }

    pub fn flush_all(&mut self) -> StorageResult<()> {
        self.flush_pending()?;
        Ok(())
    }

    pub fn stats(&self) -> StorageStats {
        self.backend.stats()
    }

    pub fn backend_name(&self) -> &'static str {
        self.backend.backend_name()
    }
}

/// Error returned by storage operations.
#[derive(thiserror::Error, Debug)]
#[error("storage error: {0}")]
pub struct StorageError(pub String);

impl From<std::io::Error> for StorageError {
    fn from(value: std::io::Error) -> Self {
        StorageError(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyBackend {
        persisted: Vec<String>,
        stats: StorageStats,
        name: &'static str,
    }

    impl Default for DummyBackend {
        fn default() -> Self {
            DummyBackend {
                persisted: Vec::new(),
                stats: StorageStats::default(),
                name: "dummy",
            }
        }
    }

    impl StorageBackend for DummyBackend {
        fn init(&mut self) -> StorageResult<()> {
            Ok(())
        }

        fn persist_batch(&mut self, batch: &[WorldRecord]) -> StorageResult<()> {
            self.stats.flush_count += 1;
            self.stats.last_flush = Some(SystemTime::now());
            self.persisted
                .extend(batch.iter().map(|record| record.summary()));
            Ok(())
        }

        fn stats(&self) -> StorageStats {
            self.stats
        }

        fn backend_name(&self) -> &'static str {
            self.name
        }
    }

    #[test]
    fn flush_respects_capacity() {
        let backend = DummyBackend::default();
        let config = StorageConfig {
            flush_interval_ms: 1000,
            batch_capacity: 2,
        };
        let mut controller = StorageController::new(backend, config).unwrap();
        controller.record(WorldRecord {
            world_id: "w".into(),
            payload: "one".into(),
            timestamp: SystemTime::now(),
        });
        assert!(!controller.flush_if_due().unwrap());
        controller.record(WorldRecord {
            world_id: "w".into(),
            payload: "two".into(),
            timestamp: SystemTime::now(),
        });
        assert!(controller.flush_if_due().unwrap());
        assert!(controller.pending().is_empty());
    }

    #[test]
    fn flush_all_with_no_pending() {
        let backend = DummyBackend::default();
        let config = StorageConfig::default();
        let mut controller = StorageController::new(backend, config).unwrap();
        assert!(controller.flush_all().is_ok());
    }
}
