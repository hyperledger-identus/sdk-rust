## ADDED Requirements

### Requirement: Embedded transport advances through explicit bounded semantics

The SDK SHALL create a validated `CredentialOffer` only by consuming an
`EmbeddedCredentialOffer` and applying a positive semantic limit set. Semantic
validation SHALL retain the exact decoded JSON owned by the transport state and
SHALL preserve every unrecognized top-level parameter for later processing.

The semantic limit set SHALL independently bound Credential Issuer Identifier
bytes, each Credential Configuration ID's decoded UTF-8 bytes, and the number
of offered Credential Configuration IDs. Defaults SHALL be 2,048 bytes, 256
bytes, and 32 IDs respectively. Semantic parsing SHALL reuse the transport's
already enforced complete-input, JSON depth, JSON node, and duplicate-member
invariants rather than silently replacing them with unbounded parsing.

#### Scenario: semantic state consumes validated transport

- **WHEN** a bounded embedded transport containing all required semantic fields
  is advanced under the default semantic limits
- **THEN** the result exposes the validated issuer and ordered configuration IDs
  through explicit accessors and retains the exact original JSON

#### Scenario: semantic limits are independent and exact

- **WHEN** issuer bytes, one decoded configuration ID, or configuration count is
  exactly at its configured maximum
- **THEN** semantic parsing succeeds if every other invariant holds, while the
  smallest one-unit excess fails with a static bounded error

#### Scenario: invalid limits cannot disable semantic enforcement

- **WHEN** any semantic maximum is zero
- **THEN** limit construction fails before an embedded offer can be consumed

### Requirement: Credential Offer core members follow OID4VCI 1.0 Final

A semantic Credential Offer SHALL require exactly one `credential_issuer`
member whose value is a string and exactly one
`credential_configuration_ids` member whose value is a non-empty array of
unique strings. Each string MAY be empty because the Final specification does
not forbid an empty issuer-metadata key, but it SHALL remain subject to the
configured byte limit. Array order SHALL be retained.

The Credential Issuer Identifier SHALL be a case-sensitive URL value using the
HTTPS scheme with a non-empty host and optional port/path, but no userinfo,
query, or fragment. Validation proves identifier syntax only and SHALL NOT
perform network access, metadata lookup, origin authentication, or issuer trust.

#### Scenario: official required members are accepted

- **WHEN** the Final example's HTTPS issuer and non-empty unique configuration
  ID array are semantically parsed
- **THEN** their exact decoded string values and order are retained

#### Scenario: required-member confusion fails closed

- **WHEN** either required member is absent, has the wrong JSON type, the ID
  array is empty or duplicated, or a value exceeds a semantic limit
- **THEN** no semantic Credential Offer is returned

#### Scenario: unsafe issuer identifiers fail closed

- **WHEN** an issuer identifier is relative, non-HTTPS, hostless, contains
  userinfo, query, or fragment, or is malformed
- **THEN** semantic parsing returns a static issuer-identifier error

### Requirement: Grants and extensions remain lossless but least-authority

The optional `grants` member SHALL be absent or a JSON object. An absent or
empty object SHALL be accepted without selecting a flow. Grant member names and
values SHALL remain opaque, and all unrecognized top-level parameters SHALL be
ignored semantically while remaining present in the retained exact JSON.

Semantic acceptance SHALL NOT claim that a grant is supported or usable, that
a Pre-Authorized Code is fresh or secret, that transaction-code instructions
are safe, or that metadata agrees with the offer.

#### Scenario: unknown extension and opaque grants survive

- **WHEN** an otherwise valid offer contains an unknown top-level member and an
  object-shaped grant with bearer-adjacent values
- **THEN** parsing succeeds, reports only whether `grants` was present, and the
  exact retained JSON still contains both values

#### Scenario: grants type confusion is rejected

- **WHEN** `grants` is present as null, a scalar, or an array
- **THEN** semantic parsing fails before a Credential Offer exists

### Requirement: Semantic values and failures are redaction-safe and portable

The semantic offer, issuer, configuration IDs, retained JSON, and errors SHALL
not expose caller-controlled content through `Debug`, `Display`, the core error
bridge, or Serde serialization. Owned semantic strings and retained JSON SHALL
be erased on drop. Public errors SHALL be fieldless, static, and use stable
`oid4vci.*` codes.

Tests SHALL cover the official Final object, independently reconstructed Oxid
and Lace shapes, escaped strings, arbitrary-magnitude numbers in ignored
extensions, exact bounds, every semantic rejection class, constructor/parser
equivalence, and diagnostic canaries. The crate SHALL remain portable across
the repository's host, Rust 1.85 MSRV, browser-WASM, Android ARM64, and iOS
ARM64 gates without HTTP, async, crypto, DID, storage, chain, or product code.

#### Scenario: sensitive canaries never enter diagnostics

- **WHEN** issuer, configuration ID, raw JSON, extension, and grant values use
  distinct canaries and semantic values/errors are formatted or bridged
- **THEN** no canary appears in any diagnostic

#### Scenario: ignored numeric extensions remain transport-compatible

- **WHEN** a valid offer includes an ignored extension number outside fixed
  machine numeric ranges
- **THEN** semantic parsing accepts it without converting or exposing the number
