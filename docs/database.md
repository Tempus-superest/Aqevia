# Database Model
Describes the persistence model, schema conventions, and schema bootstrap practices for an Aqevia world.

## Modular storage interface

- `aqevia-storage` defines the `StorageBackend` trait, `WorldRecord`, and `StorageController` that every implementation must follow. The Engine owns **when** to persist (flush scheduling and shutdown hooks), and each backend owns **how** (schema and queries).
- Backends must expose `init`, `persist_batch`, and stats so the Engine can drive dirty tracking, bounded batches, and observability updates while keeping authoritative state in memory. The default `StorageController` orchestrates flush cadence based on `StorageConfig` (flush interval + batch capacity).

## SQLite backend (initial implementation)

- `aqevia-storage-sqlite` is the first backend. It stores records in `world_records` and keeps `schema_meta` as a schema version stamp for deterministic bootstrap.
- Schema:
-  - `schema_meta(id INTEGER PRIMARY KEY, version INTEGER)` marks the schema version detected during startup.
-  - `world_records(id INTEGER PRIMARY KEY, world_id TEXT, payload TEXT, timestamp INTEGER)` stores the durable snapshots/history emitted by the Engine.
- Schema bootstrap runs via `init()` and is idempotent (`CREATE TABLE IF NOT EXISTS …`). Startup writes an initial `schema_meta` row (version `1`) when the table is empty. During development, schema changes trigger a reset: the database is dropped/recreated (or the tables are dropped) so storage reinitializes from scratch (no incremental upgrades). Production-grade schema transitions that preserve data are deferred until after 1.0 when a dedicated upgrade plan is introduced.

## Dirty tracking and flush policy

- The Engine records `WorldRecord` entries whenever it drills state updates. `StorageController` buffers those entries and flushes only when the batch capacity is reached or the configured interval elapses, preventing constant full-state writes.
- Flushing is also triggered during shutdown so that any lingering records reach durable storage before the process exits.
- Flush statistics (`flush_count`, `last_flush`) feed observability so operators can track persistence behavior.

## Configuration and environment

- The SQLite database file path is configurable via `AQEVIA_SQLITE_PATH` (defaults to `storage.sqlite` in the repo root). Keep the directory owned by the Aqevia process so no other user can tamper with durable state.
- Flush cadence is driven by `PERSIST_FLUSH_INTERVAL_MS` (default `1000`) and `PERSIST_BATCH_CAPACITY` (default `10`). Tune these values per workload to balance durability vs. throughput.
