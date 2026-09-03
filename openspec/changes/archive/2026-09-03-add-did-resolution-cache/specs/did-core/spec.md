## ADDED Requirements

### Requirement: Current no-cache resolution option

`ResolutionOptions` SHALL represent the optional W3C `noCache` boolean.
Absent and explicit false SHALL remain distinguishable on the data type while
both permit configured caching; true SHALL request a fresh VDR result and
bypass generic cache reads and writes.

#### Scenario: no-cache remains an explicit opt-in control

- **WHEN** a caller supplies absent, false or true `noCache`
- **THEN** the exact value SHALL round-trip, and only true SHALL bypass an
  attached caching resolver without changing the base resolver port

### Requirement: Bounded normalized DID resolution cache identity

The DID capability SHALL derive a bounded equality/hash cache key from the
exact validated DID and every result-affecting resolution option except
`noCache`. Extension JSON object members SHALL be recursively normalized so
member order cannot create distinct entries. Debug output SHALL NOT expose the
full DID, option values or encoded key.

#### Scenario: semantic option order cannot bypass cache identity

- **WHEN** two option maps differ only in JSON object member insertion order
  or in absent versus false `noCache`
- **THEN** they SHALL produce the same cache key, while a distinct version,
  representation, expansion choice or extension value produces another key

#### Scenario: oversized identity remains safe

- **WHEN** an otherwise valid native option map would create a key larger than
  64 KiB
- **THEN** caching SHALL be bypassed or fail closed according to explicit
  policy without logging or truncating the key

### Requirement: Injectable bounded resolution cache

The DID capability SHALL define an object-safe asynchronous
`DidResolutionCache` port with a declared capacity from 1 through 4,096,
lookup/store, per-key invalidation and all-options-for-DID invalidation.
Entries SHALL contain a typed result and process-epoch monotonic insertion and
exclusive-expiry ticks and SHALL NOT serialize.

#### Scenario: stale or poisoned entries are never served

- **WHEN** an entry is expired, observes clock regression, or fails result
  validation against the requested DID
- **THEN** it SHALL be invalidated and the resolver SHALL refresh or fail
  closed without returning that cached content

#### Scenario: registration can invalidate every request variant

- **WHEN** a DID update or deactivation succeeds
- **THEN** an outer lifecycle coordinator SHALL be able to invalidate every
  representation, version and extension-option entry for that exact DID

### Requirement: Opt-in caching resolver policy

An immutable caching resolver SHALL decorate any `DidResolver` using injected
cache and monotonic-clock ports. Positive/deactivated results MAY use a bounded
positive TTL up to 24 hours. Only `notFound` failures MAY use a separately
enabled negative TTL up to five minutes. Other failure results SHALL NOT be
cached. W3C document/version metadata, including `nextUpdate`, SHALL NOT be
interpreted as a generic TTL.

#### Scenario: cache policy does not invent standards semantics

- **WHEN** a result carries created, updated, next-update, version or canonical
  metadata
- **THEN** the decorator SHALL preserve that envelope unchanged and use only
  its injected TTL policy for cache expiry

#### Scenario: cache infrastructure failure is explicit

- **WHEN** the clock, key construction or cache backend fails
- **THEN** configured bypass mode SHALL continue safely without that cache,
  while fail-closed mode SHALL return a valid W3C `internalError` envelope

### Requirement: Redaction-safe cache observability and concurrency

The caching resolver SHALL expose typed hit, bypass, stored-miss,
not-stored-miss and refresh/backend-bypass outcomes without request content.
It SHALL be safe to clone and call concurrently. The portable contract SHALL
explicitly permit duplicate concurrent fills and require stores to tolerate
them; it SHALL NOT block executor threads or imply cancellation-unsafe
single-flight behavior.

#### Scenario: concurrent misses preserve correctness

- **WHEN** concurrent callers miss the same key before either fill completes
- **THEN** each MAY invoke the upstream resolver, but no stale/invalid result
  SHALL be served and subsequent valid hits SHALL observe a bounded entry

#### Scenario: cache overhead remains observable

- **WHEN** a representative cache-hit path is invoked repeatedly in release
  mode
- **THEN** throughput SHALL be recorded without a machine-specific CI pass
  threshold
