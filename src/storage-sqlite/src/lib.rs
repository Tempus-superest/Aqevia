//! SQLite storage backend.

use std::fs;
use std::path::Path;

#[cfg(test)]
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use aqevia_storage::{StorageBackend, StorageError, StorageResult, StorageStats, WorldRecord};
use rusqlite::{params, Connection, Error as RusqliteError};

fn to_storage_error(err: RusqliteError) -> StorageError {
    StorageError(err.to_string())
}

pub struct SqliteStorage {
    connection: Connection,
    stats: StorageStats,
}

impl SqliteStorage {
    pub fn new(path: impl AsRef<Path>) -> StorageResult<Self> {
        let db_path = path.as_ref();
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let connection = Connection::open(db_path).map_err(to_storage_error)?;
        Ok(SqliteStorage {
            connection,
            stats: StorageStats::default(),
        })
    }

    fn run_migrations(&self) -> StorageResult<()> {
        self.connection
            .execute_batch(
                "
            CREATE TABLE IF NOT EXISTS schema_meta (
                id INTEGER PRIMARY KEY,
                version INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS world_records (
                id INTEGER PRIMARY KEY,
                world_id TEXT NOT NULL,
                payload TEXT NOT NULL,
                timestamp INTEGER NOT NULL
            );
        ",
            )
            .map_err(to_storage_error)?;
        Ok(())
    }
}

impl StorageBackend for SqliteStorage {
    fn init(&mut self) -> StorageResult<()> {
        self.run_migrations()?;
        let count: i64 = self
            .connection
            .query_row("SELECT COUNT(*) FROM schema_meta", [], |row| row.get(0))
            .map_err(to_storage_error)?;
        if count == 0 {
            self.connection
                .execute("INSERT INTO schema_meta (version) VALUES (1)", params![])
                .map_err(to_storage_error)?;
        }
        Ok(())
    }

    fn persist_batch(&mut self, batch: &[WorldRecord]) -> StorageResult<()> {
        let tx = self.connection.transaction().map_err(to_storage_error)?;
        let mut stmt = tx
            .prepare("INSERT INTO world_records (world_id, payload, timestamp) VALUES (?1, ?2, ?3)")
            .map_err(to_storage_error)?;
        for record in batch {
            let secs = record
                .timestamp
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;
            stmt.execute(params![record.world_id, record.payload, secs])
                .map_err(to_storage_error)?;
        }
        drop(stmt);
        tx.commit().map_err(to_storage_error)?;
        self.stats.flush_count += 1;
        self.stats.last_flush = Some(SystemTime::now());
        Ok(())
    }

    fn stats(&self) -> StorageStats {
        self.stats
    }

    fn backend_name(&self) -> &'static str {
        "sqlite"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aqevia_storage::{StorageConfig, StorageController, WorldRecord};
    use rusqlite::Connection;
    use std::env;

    fn test_db_path() -> PathBuf {
        env::temp_dir().join("aqevia_storage_test.db")
    }

    #[test]
    fn migrations_create_tables() {
        let path = test_db_path();
        let _ = fs::remove_file(&path);
        let mut storage = SqliteStorage::new(&path).unwrap();
        storage.init().unwrap();
        let connection = Connection::open(&path).unwrap();
        let count: i64 = connection
            .query_row("SELECT COUNT(*) FROM world_records", [], |row| row.get(0))
            .unwrap_or(0);
        assert_eq!(count, 0);
    }

    #[test]
    fn sqlite_persists_batch_records() {
        let path = test_db_path();
        let _ = fs::remove_file(&path);
        let backend = SqliteStorage::new(&path).unwrap();
        let mut controller = StorageController::new(
            backend,
            StorageConfig {
                flush_interval_ms: 1,
                batch_capacity: 1,
            },
        )
        .unwrap();
        controller.record(WorldRecord {
            world_id: "world".into(),
            payload: "payload".into(),
            timestamp: SystemTime::now(),
        });
        controller.flush_pending().unwrap();
        drop(controller);
        let connection = Connection::open(&path).unwrap();
        let count: i64 = connection
            .query_row("SELECT COUNT(*) FROM world_records", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }
}
