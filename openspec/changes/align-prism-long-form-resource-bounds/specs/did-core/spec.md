# did-core Specification

## MODIFIED Requirements

### Requirement: Bounded single-pass parsing and component views

`Did` SHALL reject input larger than 131,072 bytes and `DidUrl` SHALL reject
input larger than 139,264 bytes before scanning or allocation. A DID URL SHALL
also enforce the embedded bare-DID ceiling independently. Validation SHALL
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
not widen the generic method-name surface. Validation SHALL check the byte
ceiling before traversing method characters and SHALL NOT retain or render
rejected caller input in validator-produced errors.

#### Scenario: Exact standalone method ceiling is accepted

- **WHEN** a method name contains exactly 2,042 valid ASCII bytes
- **THEN** borrowed, owned and serde construction SHALL accept it
- **AND** it SHALL remain usable in a syntactically valid bare DID with a one-byte method-specific identifier

#### Scenario: One-over method is rejected before grammar traversal

- **WHEN** a method name exceeds 2,042 bytes, including one whose later byte also violates the grammar
- **THEN** every validated construction path SHALL reject it as over-limit
- **AND** validator-produced `Debug`, `Display`, serde and stable error output SHALL NOT contain caller-controlled rejection text

#### Scenario: Existing method consumers remain compatible

- **WHEN** an accepted method is used by the immutable method registry or a DID Registration request
- **THEN** exact equality, hashing, display, serialization and dispatch behavior SHALL remain unchanged
