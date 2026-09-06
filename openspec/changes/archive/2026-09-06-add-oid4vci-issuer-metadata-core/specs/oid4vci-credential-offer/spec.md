## MODIFIED Requirements

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

## ADDED Requirements

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
