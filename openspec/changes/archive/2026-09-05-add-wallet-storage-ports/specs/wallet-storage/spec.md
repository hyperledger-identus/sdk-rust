## ADDED Requirements

### Requirement: Storage vocabulary is bounded and representation-neutral

The SDK SHALL define non-empty opaque `StorageRevision` values of at most 256
bytes and non-empty opaque `StorageCursor` values of at most 1024 bytes. Both
SHALL preserve their exact bytes for adapter forwarding while Debug exposes
only byte length. Neither type SHALL define serialization, comparison order,
revision generation or backend meaning.

`StoragePageSize` SHALL accept only values from 1 through 256 inclusive.
`StoragePageRequest` SHALL contain one validated size and an optional cursor.
`StoragePage<T>` SHALL contain at most 256 entries and SHALL reject a
continuation cursor when its entry collection is empty.

#### Scenario: exact bounds are accepted without interpretation

- **WHEN** revision, cursor and page-size values are constructed at their exact
  lower and upper bounds
- **THEN** construction succeeds, exact bytes remain available, and Debug does
  not render them

#### Scenario: invalid pagination cannot create unbounded or stalled progress

- **WHEN** a page size is zero or above 256, a token is empty or oversized, a
  page contains 257 entries, or an empty page carries a next cursor
- **THEN** construction fails through a static input error without retaining or
  echoing the rejected value

### Requirement: Single-record mutation intent is explicit

`StorageWriteCondition` SHALL distinguish unconditional `Any`, `InsertOnly`
and `IfRevision(StorageRevision)` writes. `StorageDeleteCondition` SHALL
distinguish unconditional `Any` and `IfRevision(StorageRevision)` deletes.
Adapters SHALL treat a false insert/revision precondition as
`StorageError::Conflict` and SHALL NOT silently retry it as unconditional.

A successful write SHALL return a `StorageWriteReceipt` containing an
`Inserted` or `Replaced` outcome and the new opaque revision. A deletion SHALL
return `Deleted` or `NotFound`. No API SHALL imply cross-record atomicity,
globally ordered revisions or conflict-resolution policy.

#### Scenario: stale write fails closed

- **WHEN** a caller writes with a revision that is not the current exact record
  revision
- **THEN** the port returns Conflict and does not mutate the record

#### Scenario: unconditional deletion reports absence honestly

- **WHEN** a caller deletes a missing exact record with `Any`
- **THEN** the port returns NotFound without fabricating a successful mutation

### Requirement: Generic storage wrappers redact consumer values

`Stored<T>` SHALL contain the owned value and its revision.
`StorageWrite<T>` SHALL contain the owned value and write condition.
`StoragePage<T>` SHALL contain bounded owned entries and an optional cursor.
Their Debug implementations SHALL NOT require `T: Debug` and SHALL render no
value, entry, revision or cursor bytes. Receipt Debug SHALL not render its
revision bytes.

`StorageError` SHALL be data-free and distinguish invalid revision, cursor,
page size and page construction from operational Conflict, CapacityExceeded,
Integrity, AccessDenied, Unavailable and Internal classes. Each SHALL bridge
to a static `wallet.storage_*` SDK error without backend causes or
caller-controlled scope, key, value, revision or cursor data.

#### Scenario: secret-bearing values cannot leak through generic formatting

- **WHEN** a canary value that does not implement Debug is wrapped for a read,
  write or page and each wrapper is formatted
- **THEN** compilation succeeds and the canary plus all opaque token bytes are
  absent from output

#### Scenario: backend detail stays downstream

- **WHEN** each operational error is rendered and bridged
- **THEN** only its static class, public message, capability and SDK error code
  are observable

### Requirement: Store capabilities are separate object-safe async ports

The SDK SHALL define `#[identus::port]`-marked `SecretStore`,
`CredentialStore`, `DidStore`, `ProtocolStateStore` and `StatusCacheStore`
traits. Each SHALL be `Send + Sync`, use associated consumer-owned scope, key
and value types, and return boxed borrowing `Send` futures without an executor
or async-trait dependency. Each SHALL be usable as a trait object after all
associated types are specified.

All five SHALL expose exact-key load, conditional write and conditional delete.
Load SHALL return `Ok(None)` for an absent record and SHALL return a revision
with every present value. Scope and key SHALL be borrowed so the generic
boundary does not clone or format them.

#### Scenario: unrelated consumer records use the same contracts

- **WHEN** independent string-shaped and struct-shaped adapters specify their
  own scope, key and value types
- **THEN** they are callable through the corresponding trait objects without
  importing SDK DID, credential, protocol, chain or product record types

#### Scenario: concurrent dynamic dispatch remains runtime-neutral

- **WHEN** shared trait objects are invoked concurrently by a test executor
- **THEN** the ports remain Send + Sync and complete through their boxed Send
  futures without the SDK depending on that executor

### Requirement: Secret and status cache authority is exact-key only

`SecretStore` SHALL NOT expose list, scan, count, search, export or bulk-read
operations. `StatusCacheStore` SHALL likewise expose only exact-key
load/write/delete operations; eviction, sweeping, expiry clocks and refresh
policy remain outside the port.

The SDK SHALL NOT require a secret value to implement Clone, Debug, Display,
serialization or FFI conversion. It SHALL NOT define key generation, signing,
encryption, raw-key format or custody behavior in this storage capability.

#### Scenario: secret capability cannot enumerate values

- **WHEN** a consumer receives only a `dyn SecretStore` capability
- **THEN** it can act only on an exact caller-supplied scope/key and cannot
  discover other secret keys or values through the trait

#### Scenario: cache policy is not smuggled into storage

- **WHEN** a status cache adapter stores an entry
- **THEN** the port carries no TTL, clock, refresh, trust or eviction decision
  unless the consumer-owned value itself deliberately records evidence

### Requirement: Credential DID and protocol recovery indexes are bounded

`CredentialStore`, `DidStore` and `ProtocolStateStore` SHALL each define a
consumer-owned `IndexEntry` and expose a scope-bound list operation accepting
`StoragePageRequest` and returning `StoragePage<IndexEntry>`. An adapter SHALL
return no more entries than requested and SHALL use the cursor only as an
opaque continuation within that capability and scope.

The list operation SHALL NOT require loading or formatting stored values. The
SDK SHALL define no filtering, ranking, SQL semantics, global enumeration,
retention or recovery policy.

#### Scenario: requested bound limits one page

- **WHEN** a scope contains more index entries than the requested page size
- **THEN** the adapter returns at most that many entries and an optional opaque
  cursor for a later explicit call

#### Scenario: capability indexes remain semantically independent

- **WHEN** credential, DID and protocol stores use unrelated index-entry types
- **THEN** each list contract remains usable without a common record enum or
  generic repository supertrait

### Requirement: Trait-object dispatch performance is observable

The test suite SHALL include an ignored release diagnostic that invokes ready
load futures through all five production trait surfaces and prints elapsed and
throughput data. The diagnostic SHALL use no machine-dependent pass/fail time
threshold and SHALL not replace correctness evidence.

#### Scenario: maintainer measures port overhead

- **WHEN** the ignored release diagnostic is run explicitly
- **THEN** it exercises every dynamic port, validates the returned absence and
  reports aggregate call throughput
