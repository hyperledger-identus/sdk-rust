## MODIFIED Requirements

### Requirement: Grants and extensions remain lossless but least-authority

The optional `grants` member SHALL be absent or a JSON object. An absent or
empty object SHALL be accepted without selecting a flow. Unknown grant names,
unknown members inside known grants, and all unrecognized top-level parameters
SHALL be ignored semantically while remaining present in the retained exact
JSON.

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
non-empty host and no userinfo, query, or fragment. Matching those identifiers
to issuer metadata SHALL remain a later validation state.

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
Transaction Code instruction is trustworthy; or that metadata agrees with an
Authorization Server hint.

#### Scenario: known grant alternatives remain caller-selected

- **WHEN** an offer advertises either or both Final known grant objects under
  exact configured limits
- **THEN** typed alternatives expose their validated shape and explicit values
  without selecting or executing a flow

#### Scenario: absent and extension grants remain lossless

- **WHEN** grants are absent, empty, or contain unknown grant names/members and
  arbitrary-magnitude unknown numbers
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
