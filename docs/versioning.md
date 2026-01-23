# Versioning Policy

## 1. Purpose and scope

This document defines the **canonical versioning policy** for **Aqevia**.

It governs:

- The repository version string (`VERSION`).
- Derived release versions (CLI, Docker/OCI image metadata, docs headings).
- PR naming when a version bump is part of the change.

It does **not** cover:

- Milestone planning (`/docs/milestones.md`).
- Testing requirements (`/docs/testing.md`).
- Engine contracts (e.g., `/docs/engine/*`).

## 2. Canonical source of truth

- **The root `VERSION` file is the single source of truth**.
  - It must contain plain semantic version numbers in `MAJOR.MINOR.PATCH` format with **no** `v` prefix.
  - Every other version-bearing surface is derived from this file; never edit derived surfaces independently.

## 3. Version scheme and semantics

Aqevia uses `MAJOR.MINOR.PATCH` semantics:

- `MAJOR` increments for breaking changes.
  - The project is expected to remain `0.x` until explicit compatibility guarantees exist.
- `MINOR` tracks the highest completed milestone merged to `main`; see `/docs/milestones.md` for milestone definitions.
- `PATCH` captures patch/correction releases that ship after a milestone without adding new milestone-scoped features.

Human-facing labels (CLI prompts, README titles, tags) display as `vX.Y.Z`; the canonical file remains `X.Y.Z`.

## 4. Deterministic target registry

- `scripts/version-locations.yml` is the authoritative registry of every derived target that gets stamped with the repo version.
- `scripts/sync-version` updates **only** the entries declared in that registry; do not bypass the registry with manual search/replace.
- `scripts/check-version-sync` confirms there is no drift between `VERSION` and every registered target.
- The registry now includes Compose’ `.env` file so `AQEVIA_VERSION` stays aligned with `VERSION` every time `scripts/sync-version` runs.

## 5. Bump workflow (mandatory, step-by-step)

1. Edit root `VERSION` to the desired semver.
2. Run `scripts/sync-version` to deterministically update every registered target.
   The script also rewrites `.env` with `AQEVIA_VERSION=<version>` so Docker Compose can read the canonical version without manual `export`.
3. Run `scripts/check-version-sync` to prove there is no drift.
4. Regenerate derived lockfiles via the tooling the sync script invokes (typically `cargo generate-lockfile` for Rust and `npm install --package-lock-only` (or equivalent) in the UI directory) rather than hand-editing them.
   - **DO** let automation regenerate lockfiles alongside the sync step.
   - **DO NOT** hand-edit lockfiles just to match a version string.
   - **DO NOT** instruct agents or contributors to “search the repo for version strings.”
5. If a new version surface is introduced, add it to `scripts/version-locations.yml` and teach `scripts/sync-version` / `scripts/check-version-sync` about it before merging.

## 6. Expected version-bearing surfaces (examples)

These are examples of common registered targets. The exact set is defined by `scripts/version-locations.yml`.

- Rust: `Cargo.toml` `[package].version` (derived from `VERSION`).
- UI: `ui/package.json` `.version` field.
- Docs: `README.md` first line `# Aqevia vX.Y.Z`.
- Docker/OCI: build-time stamping via `ARG VERSION` and `LABEL org.opencontainers.image.version=$VERSION`.
- Metrics/telemetry (where implemented): `aqevia_build_info{version="..."}` emits the canonical version.

## 7. Adding a new version surface

- Add a new entry to `scripts/version-locations.yml`.
- Extend `scripts/sync-version` so it writes to the new surface deterministically.
- Extend `scripts/check-version-sync` so it validates the surface.
- Update operator-facing documentation (e.g., `README.md` or relevant `/docs/*`) if the surface is visible to users.

## 8. PR and release conventions

- **Versioned PRs:** When a PR includes a version bump, prefix the title with `vX.Y.Z - `.
- **Docs-only PRs:** If no version changes occur, omit the version prefix.
- Use the explicit version the user requested wherever the bump appears (commands, README, tags).
- Never include the word “milestone” inside version strings or tags.

## 9. README discipline (version-aware)

Update `README.md` whenever:

- Operator-exposed behavior changes (WS protocol, HTTP APIs, UI, Docker, config, testing), **or**
- The PR contains a version bump.

The first line of `README.md` must always read `# Aqevia vX.Y.Z` and mirror the current `VERSION`.

## 10. Verification and acceptance criteria

- Only `VERSION` is edited manually for a bump.
- `scripts/sync-version` runs and updates the registered targets.
- `scripts/check-version-sync` runs and succeeds.
- No lockfiles are hand-edited to “patch” versions.
- README heading matches `VERSION` when the change affects operator behavior or bumps the version.
- Any new version surface is added to the registry and covered by sync/check scripts.

## 11. Standard snippet for future prompts

> “When bumping versions, do NOT search the repo for version strings. Update root `VERSION`, run `scripts/sync-version`, then `scripts/check-version-sync`. If any new version-bearing location exists, add it to `scripts/version-locations.yml` and extend the sync script.”
