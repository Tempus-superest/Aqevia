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

- `aqevia-storage-sqlite` stores records in `world_records` and keeps `schema_meta` as a schema version stamp so every bootstrap can detect mismatches.

### SQLite schema

- `schema_meta` (purpose: schema version guard)
  - `id INTEGER PRIMARY KEY` — surrogate key for the stamp.
  - `version INTEGER NOT NULL` — stores the compiled `SCHEMA_VERSION` (`1` today) so bootstrap knows whether the on-disk format matches the code-generated schema.
  - Expectation: each bootstrap writes a single row with the current schema version; if the row is missing (new database) or stale (version mismatch), the bootstrap path resets the schema before inserting the new stamp.
- `world_records` (purpose: durable snapshots)
  - `id INTEGER PRIMARY KEY` — sequential identifier assigned by SQLite.
  - `world_id TEXT NOT NULL` — the World identifier from `WorldRecord::world_id`.
  - `payload TEXT NOT NULL` — serialized snapshot from `WorldRecord::payload`.
  - `timestamp INTEGER NOT NULL` — `WorldRecord::timestamp` expressed as seconds since Unix epoch.
  - This table holds the buffered records that the Engine flushes according to `StorageConfig` ("batch capacity" / "flush interval"); each flush inserts a batch of rows inside a SQLite transaction.

### Bootstrap + dev reset semantics

- On startup, `StorageBackend::init` executes the schema creation statements (`CREATE TABLE IF NOT EXISTS …`) and attempts to read the latest `version` from `schema_meta`.
- If no version row exists, the backend inserts `SCHEMA_VERSION` (currently `1`) and continues.
- If the stored version differs from `SCHEMA_VERSION`, the backend drops `schema_meta` and `world_records`, recreates the schema, and writes the fresh version stamp. This aligns with the “reset-on-mismatch” development posture—there is no upgrade or migration path yet, and the database is returned to a clean state rather than trying to reconcile incompatible schemas.
- Because Schema resets discard persisted rows, development data is disposable, matching the early-stage rule that bootstrapping starts from scratch rather than preserving history.

## Dirty tracking and flush policy

- The Engine records `WorldRecord` entries whenever it drills state updates. `StorageController` buffers those entries and flushes only when the batch capacity is reached or the configured interval elapses, preventing constant full-state writes.
- Flushing is also triggered during shutdown so that any lingering records reach durable storage before the process exits.
- Flush statistics (`flush_count`, `last_flush`) feed observability so operators can track persistence behavior.

## Configuration and environment

- `AQEVIA_SQLITE_PATH` chooses the durable store location (default `storage.sqlite` in the repo root, or `/data/storage.sqlite` inside the Docker container). Keep the directory owned by the Aqevia process so data cannot be tampered with outside the Engine.
- `PERSIST_FLUSH_INTERVAL_MS` controls how often the Engine attempts to flush dirty records (default `1000` milliseconds). Raising it groups more writes per flush but delays durability; lowering it makes persistence more aggressive.
- `PERSIST_BATCH_CAPACITY` limits how many records the Engine accumulates before flushing (default `10`). Bump it for throughput-heavy workloads or lower it when you need tighter durability windows.
- Development data is disposable: schema mismatches drop/recreate the SQLite tables, so no upgrade path exists yet. Flush settings and file locations are configured via the env vars above so new deployments can initialize from scratch predictably.
