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
empty object SHALL be accepted without selecting a flow. Every grant value
SHALL be an object. Unknown grant names with object values, unknown members
inside known grants, and all unrecognized top-level parameters SHALL be ignored
semantically while remaining present in the retained exact JSON.

A `CredentialOffer` SHALL advance to `CredentialOfferWithGrants` only by a
consuming transition under positive grant limits. The result SHALL expose
optional `authorization_code` and
`urn:ietf:params:oauth:grant-type:pre-authorized_code` alternatives without
selecting between them. Either, both, or neither known grant MAY be present.

An `authorization_code` grant value SHALL be an object with optional non-empty,
bounded string `issuer_state` and `authorization_server` members. A
Pre-Authorized Code grant value SHALL be an object with a required non-empty,
bounded string `pre-authorized_code`, optional non-empty bounded
`authorization_server`, and optional object-shaped `tx_code`. Both grant types'
Authorization Server values SHALL have RFC 8414 issuer syntax: HTTPS with a
non-empty host and no userinfo, query, or fragment.

A present `tx_code`, including `{}`, SHALL mean that a Transaction Code is
required. Its optional `input_mode` SHALL accept only `numeric` or `text`, with
an effective default of `numeric`; optional `length` SHALL be a positive
base-10 JSON integer no greater than the configured maximum; and optional
`description` SHALL be a non-empty bounded string of no more than 300 Unicode
scalar values. Unknown Transaction Code members SHALL remain opaque.

Grant limits SHALL independently bound decoded issuer-state, Pre-Authorized
Code, Authorization Server identifier, and description bytes plus advertised
Transaction Code length. Defaults SHALL be 2,048; 4,096; 2,048; 1,200; and 64
respectively. The grant pass SHALL reuse the transport byte/depth/node and
duplicate-name bounds and SHALL preserve unknown arbitrary-magnitude numbers.

Grant acceptance SHALL NOT claim that a grant is supported, selected, or
usable; that a code is fresh, secret, single-use, or replay-safe; that a
Transaction Code instruction is trustworthy; or that a syntactically valid
Authorization Server is authorized by metadata. That final agreement requires
the explicit metadata-matched state.

#### Scenario: known grant alternatives remain caller-selected

- **WHEN** an offer advertises either or both Final known grant objects under
  exact configured limits
- **THEN** typed alternatives expose their validated shape and explicit values
  without selecting or executing a flow

#### Scenario: absent and extension grants remain lossless

- **WHEN** grants are absent, empty, or contain unknown object-shaped grant
  names/members and arbitrary-magnitude unknown numbers
- **THEN** grant validation succeeds without a known alternative and the exact
  retained JSON preserves every extension

#### Scenario: Transaction Code presence and defaults are explicit

- **WHEN** a Pre-Authorized Code grant contains an empty `tx_code` object
- **THEN** the typed result records that a Transaction Code is required, has no
  stated length/description, and has effective input mode `numeric`

#### Scenario: known grant confusion fails closed

- **WHEN** a known grant or `tx_code` is not an object; a required/optional
  string is absent, empty, or oversized; an Authorization Server identifier is
  unsafe; or mode/length/description violates its contract
- **THEN** no grant-validated offer is returned and the error is static

#### Scenario: legacy null transaction code remains non-Final evidence

- **WHEN** a Lace-shaped offer carries `tx_code: null`
- **THEN** grant validation rejects it while preserving the prior core offer's
  ability to retain that JSON opaquely

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

### Requirement: Grant values and failures are redaction-safe and portable

The grant-validated offer SHALL keep known grant values, issuer state,
Pre-Authorized Code, Authorization Server identifiers, Transaction Code
descriptions, exact JSON, and errors from exposing caller-controlled content through `Debug`,
`Display`, the core error bridge, or Serde serialization. Every owned
content-bearing string SHALL be erased on drop. Public errors SHALL be
fieldless, static, and use stable `oid4vci.*` codes.

Tests SHALL cover official Final grant examples, independently reconstructed
Oxid and Lace evidence, both-known-grant and absent/empty states, exact byte,
Unicode scalar, and integer limits, every documented rejection class,
constructor/invocation transition equivalence, unknown-extension preservation,
and diagnostic canaries. The package SHALL remain portable across repository
host, Rust 1.85 MSRV, browser-WASM, Android ARM64, and iOS ARM64 gates without
HTTP, async, crypto, DID, storage, chain, or product code.

#### Scenario: sensitive canaries never enter diagnostics

- **WHEN** every grant string, retained JSON, unknown extension, and rejected
  value uses a distinct canary and all public states/errors are formatted
- **THEN** no canary appears in any diagnostic

#### Scenario: portable grant gates remain green

- **WHEN** focused, workspace, factory, target/MSRV, supply-chain, and full Nix
  gates run
- **THEN** the crate passes without expanding its normal dependency cone
### Requirement: Unsigned Credential Issuer Metadata exposes a bounded core

The SDK SHALL accept unsigned Credential Issuer Metadata only as one complete
UTF-8 JSON object within positive configured metadata byte, depth, and node
limits. Every object at every depth SHALL have unique decoded member names.
The metadata state SHALL retain the exact JSON while interpreting only the
cross-protocol core and preserving every unknown member losslessly.

The metadata SHALL contain a required `credential_issuer` with Credential
Issuer Identifier syntax; required `credential_endpoint` with an HTTPS URL,
non-empty host, optional port/path/query, and no userinfo or fragment; and
required object-shaped `credential_configurations_supported`. Every
configuration value SHALL be an object containing a required non-empty bounded
string `format`. Configuration IDs SHALL be bounded and unique by JSON object
semantics. The configuration object SHALL contain at least one entry.

The caller SHALL supply the expected Credential Issuer Identifier from which a
future retrieval URL was derived. It SHALL be syntactically valid, bounded, and
identical to metadata `credential_issuer` under simple string comparison with
no normalization. No network access, media-type decision, origin
authentication, signed-metadata verification, or trust claim is made.

#### Scenario: Final and consumer-shaped unsigned metadata is retained

- **WHEN** bounded Final-, Oxid-, or Lace-shaped metadata contains the required
  core and arbitrary format-specific/unknown members
- **THEN** typed core accessors expose exact identifiers, endpoint,
  configuration IDs/formats, and exact original JSON

#### Scenario: retrieval identity is exact

- **WHEN** metadata `credential_issuer` differs from the expected identifier by
  case, escaping, slash, port, path, or any other byte-level spelling
- **THEN** metadata is rejected even if a URL parser could normalize the values

#### Scenario: invalid or ambiguous metadata fails closed

- **WHEN** JSON is malformed, oversized, too deep, too large, duplicates a
  decoded property, omits/mistypes a required member, uses an unsafe endpoint,
  has no configuration, or has a missing/empty/oversized format
- **THEN** no Credential Issuer Metadata state is returned

### Requirement: Authorization Server advertisement is exact and least-authority

The optional `authorization_servers` member SHALL be a non-empty array of
unique RFC 8414 issuer identifier strings: HTTPS with non-empty host, optional
port/path, and no userinfo, query, or fragment. When absent, the metadata
Credential Issuer Identifier SHALL be the single effective Authorization
Server. The state SHALL preserve whether the list was advertised and SHALL NOT
select, fetch, authenticate, or assert capability for any server.

Metadata limits SHALL independently bound complete JSON bytes, depth, nodes,
expected/metadata issuer bytes, credential endpoint bytes, each Authorization
Server identifier, Authorization Server count, each configuration ID, each
format string, and configuration count. Every maximum SHALL be positive and
the configured JSON depth SHALL not exceed 64.

Defaults SHALL be 131,072 JSON bytes; depth 16; 1,024 nodes; 2,048 issuer
bytes; 2,048 endpoint bytes; 2,048 bytes per Authorization Server; 16
Authorization Servers; 256 bytes per configuration ID; 128 bytes per format;
and 128 configurations.

#### Scenario: omitted list defaults without inventing advertisement

- **WHEN** valid metadata omits `authorization_servers`
- **THEN** accessors report no advertised list and the issuer identifier as the
  sole effective server

#### Scenario: explicit list remains ordered and duplicate-free

- **WHEN** valid metadata advertises one or more distinct servers
- **THEN** their exact order is retained, while empty, duplicate, unsafe, or
  excessive lists fail closed

### Requirement: Grant-validated offers require explicit metadata agreement

A `CredentialOfferWithGrants` SHALL consume itself and a validated
`CredentialIssuerMetadata` to create `CredentialOfferWithMetadata` only when
the offer issuer and metadata issuer are identical by simple string comparison
and every offered Credential Configuration ID exists in metadata.

If either known grant carries an `authorization_server` hint, the metadata
SHALL advertise multiple Authorization Servers and the hint SHALL exactly
match one list entry. A hint SHALL be rejected when the list is absent, has one
entry, or does not contain it. Agreement SHALL NOT select a grant or server,
interpret a format, or claim server capability/trust.

#### Scenario: matched inputs advance without policy

- **WHEN** issuer and configuration references match and every optional grant
  hint obeys the multiple-server rule
- **THEN** the matched state owns both validated inputs without choosing a flow

#### Scenario: cross-document confusion fails closed

- **WHEN** issuers differ, an offered configuration is absent, or a grant hint
  is unusable or unmatched
- **THEN** no metadata-matched offer is returned

### Requirement: Metadata states and failures are redaction-safe and portable

Metadata and matched states SHALL keep identifiers, endpoints, formats, exact
JSON, and errors from exposing caller-controlled content through `Debug`,
`Display`, the core error bridge, or Serde serialization. Every owned
content-bearing string SHALL be erased on drop. Public errors SHALL be
fieldless, static, and use stable `oid4vci.*` codes.

Tests SHALL cover Final-shaped and independently reconstructed Oxid/Lace
evidence, absent/single/multiple Authorization Servers, exact byte/count/depth
limits, all documented rejection classes, unknown-extension retention,
cross-document matching, and diagnostic canaries. The package SHALL remain
portable across host, Rust 1.85 MSRV, browser-WASM, Android ARM64, and iOS ARM64
gates without HTTP, async, crypto, DID, storage, chain, or product code.

#### Scenario: sensitive canaries never enter diagnostics

- **WHEN** metadata, offer, extension, and rejected strings contain distinct
  canaries and all public states/errors are formatted
- **THEN** no canary appears in any diagnostic

#### Scenario: portable metadata gates remain green

- **WHEN** focused, workspace, factory, target/MSRV, supply-chain, and full Nix
  gates run
- **THEN** the crate passes without expanding its normal dependency cone
