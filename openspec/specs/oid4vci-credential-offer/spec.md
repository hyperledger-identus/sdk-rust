# oid4vci-credential-offer Specification

## Purpose
TBD - created by archiving change add-oid4vci-credential-offer-transport. Update Purpose after archive.
## Requirements
### Requirement: Credential Offer invocations select exactly one bounded transport

The SDK SHALL accept an OID4VCI Credential Offer invocation only when its
ASCII-case-insensitive registered scheme and remaining wire shape are exactly
`openid-credential-offer://?<parameter>=<value>`. The parameter name SHALL be
the case-sensitive literal `credential_offer` or `credential_offer_uri`, the
value SHALL be non-empty, and the complete input SHALL not exceed the
configured invocation-byte limit.

The invocation SHALL contain no authority, path, fragment, second query pair,
duplicate parameter, simultaneous by-value/by-reference parameters, unknown
query parameter, or percent-encoded parameter-name alias. Parsing SHALL return
distinct embedded and referenced variants and SHALL perform no network access.

#### Scenario: official transport forms are distinct

- **WHEN** the final specification's by-value and by-reference invocation
  examples are parsed within configured limits
- **THEN** they produce embedded and referenced variants respectively, with
  the exact decoded value available only through an explicit accessor

#### Scenario: query smuggling fails closed

- **WHEN** an invocation has a missing/empty value, extra or duplicate pair,
  both transport names, an unknown name, an encoded name alias, authority,
  path, or fragment
- **THEN** parsing fails before any embedded or referenced public value exists

#### Scenario: consumer divergence remains visible

- **WHEN** an Oxid-shaped final invocation contains only `credential_offer`
- **THEN** it is accepted, while a Lace legacy invocation with an additional
  `issuer_origin` query parameter is rejected rather than treated as the final
  profile

### Requirement: Form decoding is strict, bounded, and UTF-8 safe

The supported query value SHALL use strict application/x-www-form-urlencoded
decoding: `+` represents a space and each `%` is followed by exactly two ASCII
hexadecimal digits that decode one byte. Raw non-ASCII query bytes, malformed
escapes, decoded NUL, and decoded invalid UTF-8 SHALL be rejected.

The decoder SHALL enforce the transport-specific decoded-byte ceiling before
each output byte is appended. Intermediate and final owned buffers SHALL be
erased on drop. Parsing SHALL fail without returning partial decoded data.

#### Scenario: encoded JSON is decoded exactly

- **WHEN** an embedded value contains valid mixed-case percent escapes and
  plus-encoded spaces within its byte ceiling
- **THEN** the retained JSON contains the corresponding exact UTF-8 characters

#### Scenario: invalid encoding does not cross the boundary

- **WHEN** a value contains a truncated/non-hex escape, raw non-ASCII byte,
  decoded invalid UTF-8, NUL, or one decoded byte beyond its ceiling
- **THEN** parsing returns a static failure without exposing partial content

### Requirement: Embedded Credential Offers are bounded unambiguous JSON objects

An embedded transport SHALL create `EmbeddedCredentialOffer` only when the
decoded value is one complete JSON object within the configured byte, maximum
depth, and aggregate node limits. Every object at every depth SHALL have
unique decoded member names; escaped and literal spellings of the same name
SHALL be duplicates. Trailing JSON and any scalar/array root SHALL be rejected.

The type SHALL retain the exact decoded JSON without interpreting required
Credential Offer members, grants, extensions, or trust. Unknown object members
SHALL remain present for a later semantic parser.

JSON number tokens SHALL be validated only against the JSON number grammar.
The transport boundary SHALL NOT convert them to a fixed-width integer or
floating-point representation, and SHALL accept any syntactically valid number
whose containing offer remains within the configured resource limits.

#### Scenario: opaque extensions survive transport validation

- **WHEN** a bounded object includes required-looking members and unknown
  nested extension members with unique names
- **THEN** transport parsing succeeds and the exact decoded object remains
  available without semantic acceptance being claimed

#### Scenario: number magnitude remains opaque

- **WHEN** an embedded object's extension contains a syntactically valid JSON
  number outside fixed-width integer or floating-point ranges
- **THEN** transport validation accepts and retains the exact number token
- **AND** malformed number grammar is still rejected

#### Scenario: ambiguous JSON fails closed

- **WHEN** any object duplicates a decoded member name, exceeds depth or node
  limits, is malformed, has trailing input, or is not an object at the root
- **THEN** the SDK rejects it through a static JSON/resource error

#### Scenario: exact JSON bounds are deterministic

- **WHEN** object bytes, depth, and node counts are exactly at each configured
  maximum and all other invariants hold
- **THEN** parsing succeeds, while the smallest one-unit excess fails

### Requirement: Referenced Credential Offers prove only safe URI syntax

A referenced transport SHALL create `CredentialOfferReference` only when the
decoded value is a syntactically valid absolute HTTPS URI within its configured
byte limit, has a non-empty host, and contains no username, password, or
fragment. Ports, path, and query SHALL be allowed. Scheme comparison SHALL be
ASCII-case-insensitive.

The type SHALL NOT fetch, resolve, redirect, cache, or classify the destination
and SHALL NOT claim issuer trust or SSRF safety. Those authorities remain with
an outer adapter and later protocol state.

#### Scenario: HTTPS reference remains least-authority data

- **WHEN** a bounded HTTPS reference with host, path, port, and query is parsed
- **THEN** the exact decoded URI is retained and no network operation occurs

#### Scenario: unsafe reference syntax is rejected

- **WHEN** the reference is relative, non-HTTPS, hostless, includes userinfo or
  a fragment, is malformed, or exceeds its byte ceiling
- **THEN** parsing fails before a reference capability is returned

### Requirement: Limits and diagnostics protect bearer-adjacent values

`CredentialOfferLimits` SHALL require positive maxima for invocation bytes,
decoded embedded JSON bytes, decoded reference URI bytes, JSON depth, and JSON
nodes. Defaults SHALL be 32,768; 16,384; 2,048; 16; and 128 respectively.
Callers MAY provide different positive limits and inspect them, except the
configured JSON depth SHALL NOT exceed 64. This hard ceiling SHALL remain below
the underlying JSON parser's recursion limit so every accepted configuration
can be honored deterministically.

Embedded JSON and reference URI types SHALL erase owned content on drop and
SHALL omit it from `Debug`. They SHALL implement neither `Display` nor Serde
serialization. Every public error SHALL carry no caller-controlled data, use a
static message and stable `oid4vci.*` core error code, and omit the invocation,
query value, JSON, reference URI, and any Pre-Authorized Code.

#### Scenario: invalid limits cannot disable enforcement

- **WHEN** any configured maximum is zero
- **THEN** construction fails and no parser can use that limit set

#### Scenario: JSON depth configuration matches parser capability

- **WHEN** JSON depth is configured at 64 or at the smallest larger value
- **THEN** 64 is accepted and honored while the larger configuration is
  rejected before parsing

#### Scenario: sensitive canaries never enter diagnostics

- **WHEN** successful values and rejected input contain distinct offer,
  Pre-Authorized Code, and reference-URI canaries and all states/errors are
  formatted
- **THEN** no canary appears in Display, Debug, or the core error bridge

### Requirement: Credential Offer transport evidence is portable and observable

The test suite SHALL cover official final examples, independently reconstructed
Oxid and Lace shapes, exact lower/upper bounds, every documented rejection
class, and constructor/parser-equivalent embedded JSON validation. The package
SHALL have no default features and SHALL compile in the existing host-tested
MSRV/etalon and compile-checked browser-WASM, Android ARM64, and iOS ARM64
lanes.

An ignored release diagnostic SHALL repeatedly parse representative by-value
and by-reference invocations, consume the validated variant, and print elapsed
time plus operations per second without a machine-specific threshold.

#### Scenario: portable package gates remain green

- **WHEN** repository Cargo, factory, conformance, MSRV, WASM/mobile, and full
  Nix gates run
- **THEN** `identus-oid4vci` builds without crypto, async, HTTP, chain, product,
  or donor dependencies

#### Scenario: maintainer observes parser cost

- **WHEN** the ignored release diagnostic runs
- **THEN** it validates both transport variants and reports throughput without
  weakening any correctness or resource assertion

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
