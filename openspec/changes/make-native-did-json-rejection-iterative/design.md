# Design

## Internal cleanup primitive

Add one crate-private module with:

- a generic `RejectionGuard<T>` that owns a candidate before validation,
  exposes a shared reference, and yields the exact candidate on success;
- a cleanup callback executed only while an armed guard drops; and
- one `drop_json_values_iteratively` worklist that consumes owned
  `serde_json::Value` roots, moves array/object children onto the worklist, and
  never recursively destroys a container.

Small private adapters destructure each candidate and feed only its raw JSON
roots into that primitive. They contain no traversal algorithm. The guard is
armed before the first validation branch, so invalid non-JSON fields cannot
cause an unguarded hostile tree to drop early.

## Constructor integration

- Document adapters drain verification properties, service endpoint/extension
  maps, and document context/top-level maps on rejected construction.
- Resolution adapters drain problem/metadata extensions, document metadata
  builder extensions, dereferenced content, and content metadata.
- Query adapters drain native option extensions for direct constructors and
  builder `build` failures.
- Registration drains rejected `RegistrationPublicData` maps.

Accepted candidates disarm the guard and preserve their existing allocation,
type, serde representation, and accessor behavior. Already-validated nested
typed values remain within current depth bounds and require no special drop.

## Evidence strategy

Construct 32,768-level arrays iteratively and place a fresh tree at every
audited entry path. Each constructor must return its existing redacted error,
including selected cases whose non-JSON field fails before JSON resource
validation. A successful bounded value proves acceptance/accessor stability.
Focused tests run with all features and no default features; full gates remain
required because the shared module crosses four DID source modules.

## Documentation and inventory

Change the native DID inventory rows from a cleanup limitation to explicit
iterative rejection evidence. Narrow `SDK-LIM-007` and the human architecture
documents only after tests pass. Preserve outer-preallocation and every other
residual exactly.

## Risks and mitigations

- A missed constructor would make the limitation removal false: the research
  table and one regression per entry family are reviewed together.
- A cleanup adapter could omit a raw field: adapters destructure candidates so
  compiler errors expose newly added fields.
- A clone or recursive formatter is outside rejection cleanup: tests pass
  ownership directly and errors remain redacted without debug formatting.
- The explicit worklist can grow with hostile breadth: that memory is bounded
  by already-owned nodes and is preferable to process-aborting recursion.

## Rollback

Revert the internal module, integrations, tests, spec, inventory, and
limitation wording together. Restore the caller pre-entry depth obligation;
there is no accepted-value or persisted-data migration.
