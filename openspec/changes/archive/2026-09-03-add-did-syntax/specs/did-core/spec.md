## ADDED Requirements

### Requirement: Validated absolute DID

The DID Core capability SHALL provide an immutable owned `Did` value that
accepts exactly the generic absolute DID grammar from W3C DID Core 1.0. A DID
SHALL contain lowercase literal `did:`, a non-empty method of lowercase ASCII
letters or digits, and a non-empty method-specific identifier made from ASCII
letters, digits, `.`, `-`, `_`, `:`, or complete percent-encoded octets. The
method-specific identifier SHALL NOT end with `:`. A bare `Did` SHALL reject
path, query and fragment delimiters. Parsing SHALL preserve the exact valid
representation without percent-decoding or normalization.

#### Scenario: chain-neutral method shapes are accepted

- **WHEN** syntactically valid `did:prism`, `did:midnight`, `did:web` and
  `did:key` shaped identifiers are parsed
- **THEN** each SHALL be accepted without the generic layer asserting that
  its method is registered, resolvable or semantically valid

#### Scenario: malformed DID grammar is rejected

- **WHEN** a DID has a missing component, uppercase or punctuation-bearing
  method, trailing method-specific colon, malformed percent escape, raw
  non-ASCII byte, whitespace, path, query or fragment
- **THEN** construction SHALL fail without preserving or displaying the
  rejected identifier in the error

### Requirement: Validated absolute DID URL

The DID Core capability SHALL provide an immutable owned `DidUrl` value that
accepts a valid `Did` followed by RFC 3986 `path-abempty`, optional query and
optional fragment components as composed by W3C DID Core. Paths SHALL be empty
or slash-led. Path bytes SHALL be RFC 3986 `pchar`; query and fragment bytes
SHALL be `pchar`, `/` or `?`; every percent escape SHALL contain two ASCII hex
digits. Empty present query and fragment components SHALL remain
distinguishable from absent components. Parsing SHALL preserve the exact valid
representation without decoding or normalization.

#### Scenario: a DID alone is also a DID URL

- **WHEN** a valid bare DID is parsed as `DidUrl`
- **THEN** it SHALL be accepted with no path, query or fragment

#### Scenario: DID URL components are exposed precisely

- **WHEN** `did:example:123/a/b?version=1#key-1` is parsed
- **THEN** the DID view SHALL be `did:example:123`, path SHALL be `/a/b`,
  query SHALL be `version=1`, and fragment SHALL be `key-1`

#### Scenario: malformed RFC 3986 components are rejected

- **WHEN** a DID URL contains an invalid path/query/fragment byte, incomplete
  percent escape, raw whitespace or a second fragment delimiter
- **THEN** construction SHALL fail with a redacted error

### Requirement: Bounded single-pass parsing and component views

`Did` SHALL reject input larger than 2,048 bytes and `DidUrl` SHALL reject
input larger than 4,096 bytes before scanning or allocation. Validation SHALL
perform one linear ASCII byte pass without regex, URL or external DID parser
dependencies. Successful values SHALL cache byte offsets so method,
method-specific identifier, DID, path, query and fragment reads return borrowed
slices without allocation. `TryFrom<String>` SHALL reuse the supplied string
allocation; converting `Did` into the equivalent `DidUrl` SHALL move it.

#### Scenario: oversized input is rejected before semantic parsing

- **WHEN** a bare DID or DID URL exceeds its public SDK byte limit
- **THEN** it SHALL be rejected as too long without scanning the full grammar
  or reflecting the input in an error

#### Scenario: repeated component access does not allocate

- **WHEN** any component accessor is called repeatedly on a valid value
- **THEN** it SHALL return a borrowed slice from the one owned representation

### Requirement: Equivalent native and wire validation

`Did` and `DidUrl` SHALL implement `FromStr`, `TryFrom<String>`, transparent
string serialization and validating deserialization. Every construction path
SHALL enforce the same grammar and limits. Failures SHALL map to stable,
redaction-safe `IdentusError` codes `did.invalid_did` and
`did.invalid_did_url` under capability `did` with `InvalidInput` kind. Neither
local nor public errors SHALL contain caller-supplied identifier text.

#### Scenario: native and serde construction agree

- **WHEN** the same valid or invalid string is supplied through native parsing
  and JSON deserialization
- **THEN** both paths SHALL make the same accept/reject decision and valid
  serialization SHALL preserve the exact string

#### Scenario: public error is stable and redacted

- **WHEN** a DID or DID URL parse failure is mapped and displayed
- **THEN** its stable code and capability SHALL identify the failed boundary
  while the rejected input SHALL NOT appear
