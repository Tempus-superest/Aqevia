# Database Model

Describes the persistence model, schema conventions, and schema bootstrap practices for an Aqevia world.

## Modular storage interface

- `aqevia-storage` defines a `StorageBackend` trait with the following responsibilities:
  - `init(&mut self)` – initialize the schema and schema metadata (`schema_meta`) before the Engine starts issuing records.
  - `persist_batch(&mut self, batch: &[WorldRecord])` – atomically commit the provided batch of `WorldRecord` snapshots in a transaction that includes payload, world identifier, and timestamp.
  - `stats(&self)` – expose flush statistics so observability can report durability health.
  - `backend_name(&self)` – return a short identifier (e.g., `sqlite`) used in observability snapshots.
  - `WorldRecord` encapsulates the authoritative snapshot payload, world ID, and timestamp emitted by the Kernel; the backend serializes/stores these blobs, while the Engine marks them dirty and decides when to flush them.
- `StorageController` (default implementation in `aqevia-storage`) encapsulates dirty-tracking, batching, and flush orchestration:
  - It buffers incoming `WorldRecord` entries, marks them dirty, and triggers a flush when the `StorageConfig` thresholds are met.
  - `StorageConfig` includes `flush_interval_ms` and `batch_capacity`, which the Engine tunes per workload while the backend remains responsible for transactional guarantees.
  - Flush stats include `flush_count`, `last_flush`, `batch_size`, and the most recent `flush_error` (if any). These stats are published to observability so operators can understand persistence cadence and failures.
- Dirty tracking follows the rule “Engine decides *when* to flush, storage decides *how* to flush safely.” The Engine schedules flushes based on `StorageConfig` timers/capacities and `StorageController` enqueues the records, while each `StorageBackend` implements the durable transaction semantics and error handling.
- Early development uses a reset-on-mismatch posture: the backend compares the stored schema version in `schema_meta` to the compiled `SCHEMA_VERSION`, and if they differ it drops/recreates the schema so no upgrade path is attempted yet.

## SQLite backend (initial implementation)

- `aqevia-storage-sqlite` is the first backend. It stores records in `world_records` and keeps `schema_meta` as a schema version stamp for deterministic bootstrap.
- Schema:
- `schema_meta(id INTEGER PRIMARY KEY, version INTEGER)` marks the schema version detected during startup.
- `world_records(id INTEGER PRIMARY KEY, world_id TEXT, payload TEXT, timestamp INTEGER)` stores the durable snapshots/history emitted by the Engine.
- Schema bootstrap runs via `init()` and is idempotent (`CREATE TABLE IF NOT EXISTS …`). Startup writes an initial `schema_meta` row (version `1`) when the table is empty. During development, schema changes trigger a reset: the database is dropped/recreated (or the tables are dropped) so storage reinitializes from scratch (no incremental upgrades). Production-grade schema transitions that preserve data are deferred until after 1.0 when a dedicated upgrade plan is introduced.

## Dirty tracking and flush policy

- The Engine records `WorldRecord` entries whenever it drills state updates. `StorageController` buffers those entries and flushes only when the batch capacity is reached or the configured interval elapses, preventing constant full-state writes.
- Flushing is also triggered during shutdown so that any lingering records reach durable storage before the process exits.
- Flush statistics (`flush_count`, `last_flush`) feed observability so operators can track persistence behavior.

## Configuration and environment

- `AQEVIA_SQLITE_PATH` chooses the durable store location (default `storage.sqlite` in the repo root, or `/data/storage.sqlite` inside the Docker container). Keep the directory owned by the Aqevia process so data cannot be tampered with outside the Engine.
- `PERSIST_FLUSH_INTERVAL_MS` controls how often the Engine attempts to flush dirty records (default `1000` milliseconds). Raising it groups more writes per flush but delays durability; lowering it makes persistence more aggressive.
- `PERSIST_BATCH_CAPACITY` limits how many records the Engine accumulates before flushing (default `10`). Bump it for throughput-heavy workloads or lower it when you need tighter durability windows.
- Development data is disposable: schema mismatches drop/recreate the SQLite tables, so no upgrade path exists yet. Flush settings and file locations are configured via the env vars above so new deployments can initialize from scratch predictably.
