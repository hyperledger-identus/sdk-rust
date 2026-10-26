# did-core Specification

## MODIFIED Requirements

### Requirement: Bounded single-pass parsing and component views

`Did` SHALL reject input larger than 131,072 bytes and `DidUrl` SHALL reject
input larger than 139,264 bytes before scanning or allocation. A DID URL SHALL
also enforce the embedded bare-DID ceiling and an 8,192-byte maximum for the
combined path, query, and fragment suffix independently. Validation SHALL
perform one linear ASCII byte pass without regex, URL or external DID parser
dependencies. Successful values SHALL cache byte offsets so method,
method-specific identifier, DID, path, query and fragment reads return borrowed
slices without allocation. `TryFrom<String>` SHALL reuse the supplied string
allocation; converting `Did` into the equivalent `DidUrl` SHALL move it.

An external grammar engine SHALL replace this implementation only when pinned,
attributable differential evidence proves the same accepted language, exact
serialization, pre-allocation limits, immutable facade, allocation reuse,
borrowed views, redacted errors and supported-target behavior without adding
unused protocol semantics or unresolved unsafe reach.

#### Scenario: oversized input is rejected before semantic parsing

- **WHEN** a bare DID or DID URL exceeds its public SDK byte limit by one or more bytes
- **THEN** it SHALL be rejected as too long without scanning the full grammar or reflecting the input in an error

#### Scenario: state-bearing method identifier fits the generic envelope

- **WHEN** a syntactically valid PRISM-shaped DID is no larger than 128,075 bytes
- **THEN** generic DID parsing SHALL accept it without claiming method validity or decoding its embedded state

#### Scenario: short DIDs do not inherit the state-bearing suffix budget

- **WHEN** a syntactically valid DID URL has a short embedded DID and more than 8,192 bytes of path, query, and fragment suffix
- **THEN** it SHALL be rejected as too long before traversing the suffix grammar

#### Scenario: repeated component access does not allocate

- **WHEN** any component accessor is called repeatedly on a valid value
- **THEN** it SHALL return a borrowed slice from the one owned representation

#### Scenario: candidate mismatch retains the local boundary

- **WHEN** a proposed parser differs on grammar, exact storage, resource work, facade invariants, allocation reuse, errors, targets, dependency cohesion or safety reach
- **THEN** the local parser SHALL remain and the candidate SHALL be recorded with a bounded reconsideration trigger rather than wrapped into production

### Requirement: Bounded absolute URI value

The DID Core capability SHALL provide an immutable owned `Uri` value for DID
document fields whose normative type is an RFC 3986 URI rather than a DID URL.
It SHALL require an absolute ASCII scheme, accept only generic URI characters
and complete percent escapes, reject fragments with a second `#`, and reject
values larger than 139,264 bytes before allocation. Parsing SHALL be bounded
and preserve exact spelling without dereferencing or scheme-specific
normalization. Every valid `Did` and `DidUrl` SHALL remain valid when converted
through their infallible `Uri` conversions.

#### Scenario: generic absolute URI schemes remain open

- **WHEN** HTTPS, DID, URN and custom absolute URI values satisfy RFC 3986 generic syntax
- **THEN** they SHALL be accepted without assigning transport or trust policy

#### Scenario: maximum DID URL converts without violating URI policy

- **WHEN** a valid DID URL at exactly 139,264 bytes is converted into `Uri`
- **THEN** the conversion and a subsequent URI or DID URL parse SHALL preserve the exact value

#### Scenario: malformed or relative values fail safely

- **WHEN** a URI is relative, contains whitespace/control/non-ASCII bytes, has a malformed escape, invalid scheme or exceeds its byte limit
- **THEN** native and serde construction SHALL reject it without reflecting the caller value through the public error bridge

### Requirement: Bounded normalized DID resolution cache identity

The DID capability SHALL derive an equality/hash cache key from the exact
validated DID and every result-affecting resolution option except `noCache`,
subject to an independent 65,536-byte retention ceiling. Extension JSON object
members SHALL be recursively normalized so member order cannot create distinct
entries. Debug output SHALL NOT expose the full DID, option values or encoded
key.

#### Scenario: semantic option order cannot bypass cache identity

- **WHEN** two option maps differ only in JSON object member insertion order or in absent versus false `noCache`
- **THEN** they SHALL produce the same cache key, while a distinct version, representation, expansion choice or extension value produces another key

#### Scenario: valid identity exceeds the retention budget

- **WHEN** an otherwise valid request would create a normalized key larger than 65,536 bytes
- **THEN** the caching resolver SHALL bypass cache reads, writes, clock and backend work under either cache failure mode
- **AND** it SHALL resolve upstream and expose a redaction-safe `KeyBudgetBypass` outcome

### Requirement: Bounded standalone DID method value

The DID Core capability SHALL provide an immutable owned `DidMethod` value for
the W3C method-name grammar. It SHALL accept non-empty lowercase ASCII letters
and digits up to 2,042 bytes. That ceiling SHALL remain independent when the
enclosing DID budget changes, so state-bearing method-specific identifiers do
not widen the generic method-name surface. The method component parsed by
`Did` and `DidUrl` SHALL enforce the same ceiling. Validation SHALL check the byte
ceiling before traversing method characters and SHALL NOT retain or render
rejected caller input in validator-produced errors.

#### Scenario: Exact standalone method ceiling is accepted

- **WHEN** a method name contains exactly 2,042 valid ASCII bytes
- **THEN** borrowed, owned and serde construction SHALL accept it
- **AND** it SHALL remain usable in a syntactically valid bare DID with a one-byte method-specific identifier

#### Scenario: One-over method is rejected before grammar traversal

- **WHEN** a method name exceeds 2,042 bytes, including one whose later byte also violates the grammar
- **THEN** standalone, bare-DID, and DID-URL construction paths SHALL reject it as over-limit
- **AND** validator-produced `Debug`, `Display`, serde and stable error output SHALL NOT contain caller-controlled rejection text

#### Scenario: Existing method consumers remain compatible

- **WHEN** an accepted method is used by the immutable method registry or a DID Registration request
- **THEN** exact equality, hashing, display, serialization and dispatch behavior SHALL remain unchanged

### Requirement: Opt-in caching resolver policy

An immutable caching resolver SHALL decorate any `DidResolver` using injected
cache and monotonic-clock ports. Positive/deactivated results MAY use a bounded
positive TTL up to 24 hours. Only `notFound` failures MAY use a separately
enabled negative TTL up to five minutes. Other failure results SHALL NOT be
cached. W3C document/version metadata, including `nextUpdate`, SHALL NOT be
interpreted as a generic TTL. A valid request whose normalized cache identity
exceeds the independent retention ceiling SHALL bypass the cache before clock
or backend work under either cache failure mode and SHALL resolve upstream.

#### Scenario: cache policy does not invent standards semantics

- **WHEN** a result carries created, updated, next-update, version or canonical
  metadata
- **THEN** the decorator SHALL preserve that envelope unchanged and use only
  its injected TTL policy for cache expiry

#### Scenario: cache infrastructure failure is explicit

- **WHEN** the clock or cache backend fails
- **THEN** configured bypass mode SHALL continue safely without that cache,
  while fail-closed mode SHALL return a valid W3C `internalError` envelope

#### Scenario: oversized cache identity is not infrastructure failure

- **WHEN** a valid request cannot fit the independent normalized-key retention
  budget
- **THEN** both cache failure modes SHALL bypass clock and backend work, resolve
  upstream, and expose a redaction-safe `KeyBudgetBypass` outcome

### Requirement: Redaction-safe cache observability and concurrency

The caching resolver SHALL expose typed hit, bypass, key-budget-bypass,
stored-miss, not-stored-miss and refresh/backend-bypass outcomes without
request content. It SHALL be safe to clone and call concurrently. The portable
contract SHALL explicitly permit duplicate concurrent fills and require stores
to tolerate them; it SHALL NOT block executor threads or imply
cancellation-unsafe single-flight behavior.

#### Scenario: concurrent misses preserve correctness

- **WHEN** concurrent callers miss the same key before either fill completes
- **THEN** each MAY invoke the upstream resolver, but no stale/invalid result
  SHALL be served and subsequent valid hits SHALL observe a bounded entry

#### Scenario: cache overhead remains observable

- **WHEN** a representative cache-hit path is invoked repeatedly in release
  mode
- **THEN** throughput SHALL be recorded without a machine-specific CI pass
  threshold

### Requirement: Independently evidenced URI recognition

The production `Uri` parser SHALL preserve the RFC 3986 `URI` production and
the SDK's documented ASCII and 139,264-byte resource profile using exact
`fluent-uri 0.4.1` as a private grammar engine with default features disabled.
The byte ceiling SHALL execute before parsing. Candidate types, errors,
normalization, IRI, resolution and component policy SHALL NOT enter the public
API or retained representation; successful values SHALL retain exact caller
spelling in the existing Identus-owned type.

Deterministic conformance tests SHALL compare accept/reject decisions with the
exact development-only `uriparse 0.6.4` oracle used by NeoPRISM over scheme,
authority, path, query, fragment, percent-encoding, IPv4, IPv6 and IPvFuture
classes. Every mismatch SHALL be a pinned regression with an explicit
normative or SDK-policy classification. DID and DID URL lexical parsing SHALL
remain on their existing method-specific implementation and SHALL NOT be
replaced by the generic URI engine.

#### Scenario: Standards-shaped component combinations agree

- **WHEN** bounded RFC 3986 URI component combinations are generated deterministically
- **THEN** the SDK grammar engine and independent oracle SHALL agree except for explicitly recorded SDK resource/profile or oracle defects

#### Scenario: Published dependency remains private

- **WHEN** a generic `Uri` is parsed, formatted, serialized or rejected
- **THEN** its public API, exact spelling, 139,264-byte precheck and stable Identus error facade remain unchanged and expose no `fluent-uri` type or diagnostic

#### Scenario: Development evidence does not widen runtime dependencies

- **WHEN** production, minimal-feature, mobile or WASM dependency cones are built
- **THEN** `uriparse` SHALL NOT be a normal or target dependency of `identus-did` and SHALL remain development-only

#### Scenario: Method-specific parsing is not replaced accidentally

- **WHEN** `Did` and `DidUrl` parse their bounded lexical forms
- **THEN** their current cached-parts parser, errors and fuzz targets remain independent of the generic URI dependency
