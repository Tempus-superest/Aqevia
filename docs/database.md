# Database

This document defines Aqevia’s persistence approach and database strategy. It captures the current decision to start with SQLite while keeping storage strictly modular so additional database backends can be added later by implementing a new module rather than rewriting the Engine.

## Notes

- **Modular storage is a hard requirement.** Persistence must be implemented behind a storage interface so future backends (e.g., Postgres, MongoDB) can be added as new modules with minimal impact to the rest of the system.
- **SQLite is the initial backend.** It is the default and only supported database at the start of the project.

### Persistence model

- The **authoritative game state lives in memory** inside the Aqevia Engine.
- The database is used for **durable storage** (content, accounts/roles, audit logs, and state snapshots/updates).

### Save/flush policy vs storage backend

- The Engine owns **when** persistence happens (flush cadence, checkpoint cadence, shutdown behavior). These are global engine settings and must not be tied to SQLite-specific code.
- The storage module owns **how** persistence happens (schema, queries, transactions, batching, migrations, connection behavior).

### Write behavior (high level)

- Prefer **dirty tracking + periodic flush** rather than “write everything constantly.”
- Flush should write **only changed entities** in bounded batches.
- Use transactions for atomic updates where needed.
- Optionally support periodic **snapshots/checkpoints** to speed recovery.

### SQLite operational notes

- Enable SQLite settings appropriate for concurrent reads and safe writes (e.g., WAL mode and sane busy timeouts).
- SQLite tuning belongs in the SQLite module configuration; flush intervals and save cadence belong to Engine configuration.

### Migration and schema conventions

- Keep a clear, versioned schema and migration story from day one.
- Schema and migrations are owned by the active storage module.

-------------
