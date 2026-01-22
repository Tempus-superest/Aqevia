//! Entry point for the Aqevia Engine binary that hosts a single World per deployment unit.

use std::env;
use std::error::Error;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use aqevia_engine::Engine;
use aqevia_storage::StorageConfig;
use aqevia_storage_sqlite::SqliteStorage;
use aqevia_transport::{ObservabilityServer, ObservabilityState};

fn read_project_version() -> Result<String, std::io::Error> {
    let contents = std::fs::read_to_string("VERSION")?;
    Ok(contents
        .lines()
        .find(|line| !line.starts_with('#'))
        .unwrap_or("0.2.0")
        .trim()
        .to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    let version = read_project_version()?;
    let world_id = "aqevia-default-world";
    let storage_path = env::var("AQEVIA_SQLITE_PATH").unwrap_or_else(|_| "storage.sqlite".into());
    let storage = SqliteStorage::new(PathBuf::from(storage_path))?;
    let observability = Arc::new(ObservabilityState::new(version.clone(), world_id, "sqlite"));
    let flush_interval_ms = env::var("PERSIST_FLUSH_INTERVAL_MS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(1000);
    let batch_capacity = env::var("PERSIST_BATCH_CAPACITY")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(10);

    let mut engine = Engine::new(
        storage,
        StorageConfig {
            flush_interval_ms,
            batch_capacity,
        },
        observability.clone(),
    )?;

    let addr: SocketAddr = env::var("AQEVIA_OBSERVABILITY_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:7878".into())
        .parse()?;
    let mut server = ObservabilityServer::start(observability.clone(), addr)?;
    let output = engine.run_one_world("ready")?;
    println!("Server running: {}", output);
    engine.flush_all()?;
    server.shutdown();
    Ok(())
}
