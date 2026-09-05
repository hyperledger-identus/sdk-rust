# Design: least-authority wallet storage ports

## Context

Issue #89 advances `IDR-010` from
`develop@5141044384e51cc26a73a19adb11d0f18f70f74a`. Oxid proves that wallet
profiles, credentials and DID records need distinct persistence capabilities.
Midnight proves that secret controller state has lifecycle-specific keys and
must not be enumerable. Lace proves that resumable protocol state needs a
bounded recovery index. Those implementations also contain product IDs,
runtime choices, encryption formats and policy that must not move upstream.

The existing `identus-wallet` package is a quarantined orchestration-layer
placeholder. This slice activates only a generic storage contract. It does not
claim a wallet product or custody implementation.

## Provenance and isolation

No donor code or fixture is copied. Exact revisions, file digests, licenses
and pre-existing worktree states are recorded in issue #89. Oxid,
midnight-identity and Lace ID Portal remain read-only. NeoPRISM and Apollo are
not source inputs for this slice.

## Decisions

### D1 — Storage coordination belongs in the orchestration ring

The five capabilities live in `identus-wallet::storage`. Storage crosses DID,
credential and protocol concerns, so placing it in one inner domain crate
would reverse dependency direction. Creating the blueprint's future
`identus-ports` package now would add a new crate and rulebook member for one
slice. Activating the existing orchestration package is smaller and reversible.

The implementation depends only on `identus-core` at runtime and on
`identus-derive` for the repository-standard port marker. Associated types let
consumer adapters use their own domain records without pulling DID,
credential, presentation or protocol crates into this contract.

### D2 — Five capabilities, no generic repository supertrait

The SDK defines `SecretStore`, `CredentialStore`, `DidStore`,
`ProtocolStateStore` and `StatusCacheStore`. Each is `#[identus::port]`,
`Send + Sync`, object-safe with fully specified associated types and returns a
boxed borrowing `Send` future. There is no async-trait or executor dependency.

Each port repeats its small operation set intentionally. A blanket generic
CRUD abstraction would let a secret store accidentally acquire enumeration or
would force every capability into the same indexing and retention semantics.

### D3 — Consumer-owned types preserve repository boundaries

Each port has associated `Scope`, `Key` and `Value` types. Credential, DID and
protocol-state ports additionally define an `IndexEntry` used for bounded
listing. Scope and key values are borrowed; stored values move into writes and
out of reads. The SDK neither creates an Oxid profile ID nor interprets a
Midnight contract address, credential codec, DID representation or protocol
session payload.

`SecretStore` and `StatusCacheStore` expose exact-key operations only. Secret
enumeration is a least-authority violation. Status cache sweeping, expiry and
eviction are adapter/application policy rather than generic read authority.

### D4 — Opaque revisions provide single-record optimistic concurrency

`StorageRevision` owns 1 through 256 opaque bytes. It may carry a database
version, content hash or ETag without defining its algorithm or wire format.
`StorageWriteCondition` distinguishes `Any`, `InsertOnly` and
`IfRevision`; `StorageDeleteCondition` distinguishes `Any` and `IfRevision`.

A successful write returns `StorageWriteReceipt` with `Inserted` or `Replaced`
and the new revision. Deletion returns `Deleted` or `NotFound`. A conditional
operation whose precondition is not true returns `StorageError::Conflict`;
adapters do not silently fall back to unconditional mutation. The contract
promises atomicity only for the named single record, not cross-record
transactions or global monotonic revisions.

### D5 — Reads and indexes are bounded

An exact read returns `Option<Stored<T>>`; absence is ordinary data, not an
operational error. `Stored<T>` carries the value and its opaque revision.

`StoragePageSize` accepts 1 through 256. `StorageCursor` accepts 1 through 1024
opaque bytes. `StoragePageRequest` owns the size and optional cursor.
`StoragePage<T>` rejects more than 256 entries and rejects an empty page with a
continuation cursor, preventing an ambiguous non-progressing pagination loop.
List-capable ports must return no more entries than the requested size; this
behavior is exercised by test doubles because it spans request and response.

No API lists full secret or cache values. Index entries are consumer-owned and
may be cheap identifiers or summaries.

### D6 — Debug and errors reveal structure, never caller values

Opaque revision and cursor Debug output contains only byte length. Stored
values, write values, page entries and receipt revisions are not rendered.
Compound Debug output exposes only conditions, counts and presence flags.

`StorageError` is data-free and distinguishes invalid revision, cursor, page
size or page construction from operational `Conflict`, `CapacityExceeded`,
`Integrity`, `AccessDenied`, `Unavailable` and `Internal` classes. Every
variant maps to a static `wallet.storage_*` `IdentusError`; no adapter cause,
scope, key, value, cursor or revision crosses the boundary.

### D7 — Test doubles are evidence, not production adapters

Tests implement independent consumer-shaped ports using synchronized memory.
They prove exact scope/key forwarding, missing reads, insert-only and revision
conflicts, revision rotation, idempotent unconditional deletion, bounded
pagination, non-enumerable secrets, trait-object use and concurrent calls.
Production memory and encrypted adapters remain consumer or future
outer-boundary work.

An ignored release diagnostic polls ready futures through all five trait
objects and reports calls per second without a machine-dependent threshold.

## Risks and trade-offs

- Associated types require a composition root to spell concrete trait-object
  type equalities. That verbosity is accepted because it prevents SDK-owned
  product record types and keeps the dependency cone minimal.
- Boxed futures allocate per call. This matches existing SDK async ports,
  supports dynamic adapters and is measured explicitly. A later GAT/RPITIT
  migration would be a compatibility decision.
- Conditional semantics are strong enough for one-record lost-update
  prevention but cannot make multi-record wallet workflows atomic. A unit of
  work requires independent evidence from at least two consumers.
- `Any` permits deliberate last-writer-wins behavior. Orchestration code that
  has read a revision should use `IfRevision`; the port does not impose product
  conflict-resolution policy.
- Values may contain secrets, but the SDK never formats or serializes them.
  Consumer types and adapters remain responsible for zeroization and custody.

## Migration and rollback

The API is additive and unreleased and defines no stored representation. Oxid,
Midnight and Lace adapters remain unchanged. A focused revert restores the
wallet placeholder. Production adapters, conformance across in-memory and
encrypted stores, downstream adoption and release remain follow-up issues.
