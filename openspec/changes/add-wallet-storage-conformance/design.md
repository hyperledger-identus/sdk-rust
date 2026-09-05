# Design: reusable wallet storage adapter conformance

## Context

Issue #91 follows the five storage ports delivered by #89 at
`develop@db83fbc1d7dbe66f7c7a09bbc13a7f606559e67d`. The SDK contract is generic
over consumer scope, key, value and index types. The next useful upstream
component is therefore a portable behavioral test kit, not an SDK-owned
storage implementation.

## Decisions

### D1 — A dedicated verification crate owns consumer test support

`identus-wallet-conformance` sits in the verification ring and depends only on
`identus-wallet`. It is distinct from `identus-conformance`, whose current job
is self-policing repository architecture and source shape. Production crates
must not depend on either verification crate.

### D2 — Checks invoke the five production traits directly

The public API provides one executor-neutral async entry point for each store.
Exact-key fixtures carry a primary scope, isolated scope, used key, missing
key, initial value and replacement value. List fixtures carry a preseeded
scope and the exact expected index entries. Bounds require only `Clone` and
`PartialEq` where the lifecycle must consume or compare a value; no consumer
type needs `Debug`, `Display`, serialization, hashing or ordering.

The five entry points intentionally repeat trait dispatch at their boundary.
Private helpers share assertions without introducing a production repository
supertrait.

### D3 — Exact-record lifecycle is the minimum common contract

Every suite checks missing read, insert-only success, read-after-write,
duplicate insert conflict, unconditional replacement, stale-revision write and
delete conflict, current-revision delete and unconditional delete of a missing
record. A failed precondition must leave the current record unchanged.

A successful replacement must return a revision different from the revision
it invalidates. Without that rule, `IfRevision` cannot prevent lost updates.
Revision bytes remain opaque and need not be globally unique or ordered.

The isolated scope must not observe the primary record. This proves that scope
is forwarded and enforced without revealing either scope value.

### D4 — Pagination evidence is preseeded and bounded

Credential, DID and protocol-state checks paginate a caller-preseeded index
using a page size smaller than the expected set. Every page must respect the
requested bound, every non-final page must make cursor progress, traversal must
terminate within a bound derived from the expected count, and the observed
entries must equal the expected multiset with no duplicates.

The kit does not define how writes create index entries. Consumers seed through
their native adapter API so index policy remains downstream.

### D5 — Diagnostics are safe and portable

Checks return a `StorageConformanceReport` containing only completed operation
and observed-entry counts. Failures contain a static suite step and a closed
failure kind. Debug and Display never render consumer-owned values, revisions
or cursors. The kit selects no executor; consumers await it in their native
test runtime.

### D6 — Local memory code proves the kit, not the product

The crate tests contain a synchronized in-memory implementation shared by
consumer-shaped wrappers for all five traits. It rotates per-record revisions,
implements exact compare-and-swap and deterministic opaque pagination. It is
compiled only for tests and is not exported.

An ignored release diagnostic repeatedly runs a ready exact-record suite and
prints elapsed time and throughput without a machine-specific threshold.

## Risks and trade-offs

- Requiring `Clone + PartialEq` for test fixtures is stricter than the storage
  ports themselves, but only test data carries the requirement.
- The public functions have similar implementations because Rust cannot
  abstract over unrelated associated-type traits without adding the generic
  repository interface the production design rejected.
- Equality of opaque revisions is used only to prove invalidation, never to
  infer ordering or backend structure.
- The test kit can detect contract violations but cannot prove encryption,
  crash consistency or platform custody; those require downstream evidence.

## Migration and rollback

The new crate is additive and unreleased. Downstreams may dev-depend on an
immutable SDK revision and call only the entry point matching each adapter.
Removing the crate before release changes no stored data or production runtime.
