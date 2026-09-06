# oid4vci-token-response-core Specification

## Purpose
TBD - created by archiving change add-oid4vci-token-response-core. Update Purpose after archive.
## Requirements
### Requirement: Successful Token Response core is strict, partial, and bounded

The SDK SHALL parse one top-level JSON object into `TokenResponseCore` only
when the complete input fits positive `TokenResponseLimits`, every JSON value
fits the configured depth and aggregate-node limits, and no object at any depth
repeats a decoded member name.

The limits SHALL default to 65,536 complete JSON bytes, depth 16, 1,024 nodes,
16,384 decoded access-token bytes, 256 decoded token-type bytes, 16,384 decoded
refresh-token bytes, and 4,096 decoded scope bytes. Every maximum SHALL be
positive, and configurable depth SHALL NOT exceed the repository maximum.
Aggregate oversize SHALL be rejected before copying the input.

Success SHALL prove only the bounded OAuth core syntax. It SHALL NOT prove HTTP
status or headers, response provenance/correlation, token cryptographic
validity, token freshness, server/issuer trust, Authorization Details,
Credential Dataset semantics, replay safety, or successful issuance.

#### Scenario: minimal Final response succeeds

- **WHEN** a bounded object contains one valid `access_token` and one valid
  `token_type`
- **THEN** parsing succeeds with all optional members absent

#### Scenario: limits reject work before an unbounded state exists

- **WHEN** any configured maximum is zero, depth exceeds the repository
  maximum, or the input exceeds its aggregate byte bound
- **THEN** parsing fails with the corresponding static limit error

#### Scenario: duplicate, deep, or wide extension JSON fails

- **WHEN** any nested object repeats a decoded member name or the document
  exceeds configured depth or node count
- **THEN** the existing duplicate/depth/node error is returned without input
  content

### Requirement: OAuth core fields follow their wire grammars

`access_token` and `token_type` SHALL each be required exactly once as
non-empty JSON strings. Access and optional refresh tokens SHALL contain only
ASCII visible characters from `%x20-7E`, matching RFC 6749 `1*VSCHAR`.

Token type SHALL satisfy either the non-empty RFC `type-name` grammar or a
non-empty URI-reference, remain byte-for-byte exact, and support
ASCII-case-insensitive comparison. The parser SHALL NOT restrict the value to
Bearer or DPoP.

Optional `expires_in` SHALL be a non-negative base-10 JSON integer with no
sign, fraction, or exponent and SHALL fit `u64`. Optional `scope` SHALL be one
or more RFC `scope-token` values separated by exactly one ASCII space, with no
leading, trailing, repeated, quote, backslash, control, non-ASCII, or empty
token. Each decoded known string SHALL fit its independent configured bound.

#### Scenario: complete RFC core remains exact

- **WHEN** valid access token, mixed-case token type, expiry, refresh token,
  and scope are present
- **THEN** exact values and optional presence are exposed and token type can be
  compared case-insensitively

#### Scenario: malformed expiry is rejected

- **WHEN** `expires_in` is negative, signed, fractional, exponential,
  non-numeric, or larger than `u64`
- **THEN** parsing fails with the static invalid-expiry error

#### Scenario: invalid token or scope grammar is rejected

- **WHEN** a token is empty or contains a non-visible octet, or scope has an
  empty/invalid token
- **THEN** parsing fails without echoing the offending value

### Requirement: Extensions interoperate without widening semantic claims

Unrecognized Token Response member names SHALL be syntax-, duplicate-, depth-,
and node-checked and then ignored semantically, as required by RFC 6749 and
OID4VCI Final. The exact response SHALL remain in private zeroizing storage so
a later typed transition can validate recognized extension data without a
second unprotected copy.

When `authorization_details` is present, the core SHALL report presence but
SHALL NOT expose, validate, or interpret it in this capability. Historical
`c_nonce` or `c_nonce_expires_in` members SHALL NOT cause an otherwise valid
core to fail and SHALL NOT be exposed as current Final behavior.

#### Scenario: unknown extensions are ignored safely

- **WHEN** a valid core also contains bounded unknown nested values
- **THEN** parsing succeeds without adding their names or values to the public
  semantic surface

#### Scenario: Authorization Details remains an explicit partial state

- **WHEN** `authorization_details` is present as any structurally valid bounded
  JSON value
- **THEN** the core reports presence only and cannot claim Credential Dataset
  validation or construct a Credential Request

### Requirement: Token Response secrets and diagnostics remain protected

`TokenResponseCore` SHALL keep its exact response plus extracted access token,
token type, optional refresh token, and optional scope in zeroizing storage.
It SHALL have no Clone, Display, Serde, or raw-response accessor. Raw tokens and
scope SHALL be available only through explicitly sensitive accessors documented
against logs, telemetry, URLs, caches, generic serialization, and long-lived
storage.

Debug SHALL contain only exact response byte count and optional-member presence.
Every new error SHALL be fieldless and map to a stable static `oid4vci.*`
code/message through the existing core-error bridge. No diagnostic SHALL echo
tokens, token type, scope, Authorization Details, extension content, JSON, URI,
endpoint, or parser cause.

The change SHALL add no dependency, manifest, lockfile, feature, unsafe,
consumer, chain, or product change and SHALL preserve Rust 1.85,
browser-WASM, Android ARM64, and iOS ARM64 portability.

#### Scenario: secrets appear only at deliberate access boundaries

- **WHEN** a caller inspects response Debug, field presence, byte count,
  direct errors, or bridged errors
- **THEN** no response content appears and raw values are available only from
  explicitly sensitive accessors

#### Scenario: portable policy-neutral gates remain green

- **WHEN** focused, workspace, factory, target/MSRV, supply-chain, and full Nix
  gates run
- **THEN** the response core passes without dependency-cone, feature, target,
  or downstream drift
