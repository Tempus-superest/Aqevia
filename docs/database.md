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

- **Dirty records** are the `WorldRecord` entries that the Kernel emits but the StorageController has not yet flushed to durable storage. Every record is marked dirty when it is enqueued, and `StorageController` buffers them until a flush event occurs.
- **Flush configuration**:
  - `PERSIST_FLUSH_INTERVAL_MS` (default `1000` ms) controls the timer that wakes the controller to flush even if the batch is not full; a shorter interval favors durability at the cost of more frequent disk work, while longer intervals group writes for throughput.
  - `PERSIST_BATCH_CAPACITY` (default `10`) caps how many dirty records a single flush can persist; when the queue grows faster than the flush cadence, additional flush cycles continue draining the backlog until caught up.
  - The Engine owns the cadence (when timers fire or capacity is reached), while the backend owns how the records are written in a transaction.
- **Batch formation and sustained pressure**:
  - When either interval or capacity triggers, StorageController issues `persist_batch` with up to `PERSIST_BATCH_CAPACITY` records; batches are processed sequentially so ordering is preserved per flush batch.
  - If write pressure remains high, multiple flush cycles run back-to-back, each draining another batch until the dirty queue is empty; flush stats (`flush_count`, `batch_size`, `last_flush`, `last_flush_error`) reveal how often and how much data is being persisted.
- **Shutdown expectations**:
  - Clean shutdown attempts a final flush before exiting, giving StorageBackend a best-effort chance to commit remaining dirty records and report via `flush_error` if it fails.
  - Abrupt shutdown (killed process or crashes) can lose dirty records because the backend only persists what its latest flush completed; this aligns with the dev posture that data is disposable and boots start from scratch after restarts.
  - Observability surfaces read these stats so operators can monitor whether dirty records were drained before shutdown or if errors need attention.

## Configuration and environment

- `AQEVIA_SQLITE_PATH` chooses the durable store location (default `storage.sqlite` in the repo root, or `/data/storage.sqlite` inside the Docker container). Keep the directory owned by the Aqevia process so data cannot be tampered with outside the Engine.
- `PERSIST_FLUSH_INTERVAL_MS` controls how often the Engine attempts to flush dirty records (default `1000` milliseconds). Raising it groups more writes per flush but delays durability; lowering it makes persistence more aggressive.
- `PERSIST_BATCH_CAPACITY` limits how many records the Engine accumulates before flushing (default `10`). Bump it for throughput-heavy workloads or lower it when you need tighter durability windows.
- Development data is disposable: schema mismatches drop/recreate the SQLite tables, so no upgrade path exists yet. Flush settings and file locations are configured via the env vars above so new deployments can initialize from scratch predictably.
