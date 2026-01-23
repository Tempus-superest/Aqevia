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

## Open design decisions (resolve before relying on “JSON schema”)

Engineering must still resolve how `WorldRecord` blobs are modeled, validated, mutated, and queried before treating them as a concrete JSON schema. This section collects the outstanding choices and suggests defaults so teams can reason about safety without hand-waving toward a final schema definition.

### WorldRecord Envelope Definition
- **Decision required:** What fields make up a `WorldRecord` envelope (identity, type, optional human key/slug, payload JSON, and metadata such as `world_id`/timestamps)? How do we derive the primary key, enforce uniqueness, and namespace `kind` values?
- **Recommended default (non-binding):** `record_id` is a UUID primary key; `kind` is a namespaced string (`core.room`, `user.<mod>.thing`) and must be unique in combination with an optional `key` string scoped to that `kind`; `payload` holds the handwritten JSON blob; `metadata` carries engine-specific info (timestamps, source, etc.).
- **Example envelope fields:**
```json
{
  "record_id": "1a2b3c4d-...-9f",
  "kind": "core.room",
  "key": "lobby",
  "payload": { "description": "A bright entry hall." },
  "metadata": { "world_id": "main", "created_at": 1660000000 }
}
```

### Validation Model (Who validates, and when?)
- **Decision required:** Which layer validates incoming payloads (control plane before persistence, kernel on load, storage on deserialize)? How do we handle unknown fields and schema version drift?
- **Recommended default (non-binding):** Control plane validates POST/PATCH submissions and rejects schema violations; kernel re-validates/decodes payloads when loading them; unknown fields are preserved in storage but ignored unless a later version recognizes them, which keeps the system forward-compatible without dropping user data.

### Mutation / Update Semantics
- **Decision required:** Are updates whole-document replaces or patches? What patch format do we accept, and at what granularity do we track dirtiness?
- **Recommended default (non-binding):** Store full payload blobs; control plane exposes JSON Merge Patch (RFC 7396) for partial updates; kernel/storage track dirty state at the `WorldRecord` level so flushes operate on whole records even if only one field changed.
- **Example JSON Merge Patch:**
```json
{
  "payload": {
    "description": "A dim corridor with rune-lit walls."
  }
}
```

### Querying / Indexing Strategy (SQLite reality)
- **Decision required:** Do we support only key-based access, or query against payload fields (e.g., search for all rooms in an `area_id`)? Will we rely on SQLite’s JSON1 extension or extract indexed columns?
- **Recommended default (non-binding):** Hybrid: index `kind`, `key`, and `updated_at` at minimum; optionally extract a few high-value fields (like `payload.area_id`) into their own columns when a use case justifies it; avoid prematurely flattening the entire JSON blob.

### References Between Records
- **Decision required:** Do references point to opaque IDs, human-friendly keys, or both? Who validates referential integrity, and how do we behave when references break?
- **Recommended default (non-binding):** Reference by `record_id`, optionally document the `kind` for readability; control plane validates referenced IDs when possible; kernel degrades gracefully (logs warnings or skips missing refs) so gameplay remains resilient.

### Canonicalization / Hashing (Optional but useful)
- **Decision required:** Do we normalize payload JSON before storage? Should we persist a payload hash for change detection or diffing?
- **Recommended default (non-binding):** Canonicalize JSON (sorted keys, stable formatting) on write to make diffs more deterministic; optionally write a `payload_hash` column for quick change checks without re-reading the blob.

### Limits & Safety
- **Decision required:** What caps do we enforce on payload size, nesting depth, and string length, and at which layer do we enforce them?
- **Recommended default (non-binding):** Enforce conservative per-record limits at the control plane (document a suggested cap, e.g., 256 KiB per payload) and reject oversized submissions before they hit storage; document these caps so builders understand the boundaries.

### “Lock these now” Minimal Decision Set
- `record_id` strategy (UUID vs other) and `kind` namespacing.
- Validation placement (control plane vs kernel) and unknown-field policy.
- Mutation semantics (whole replace vs JSON Merge Patch) plus dirty-tracking scope.
- Core indexes (kind/key/updated_at) before committing to JSON versus derived-column indexing.
- Reference format (IDs only vs ID+kind) and missing-reference behavior.

These decisions must be resolved before we treat any “JSON schema” as authoritative; until then, each choice should carry a clearly documented default and a plan for revisiting it once requirements crystallize.

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
