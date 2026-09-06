# oid4vci-credential-nonce-response-core Specification

## Purpose
TBD - created by archiving change add-oid4vci-credential-nonce-response-core. Update Purpose after archive.
## Requirements
### Requirement: Credential Nonce Response core is strict, partial, and bounded

The SDK SHALL parse one top-level JSON object into
`CredentialNonceResponseCore` only when the complete input fits positive
`CredentialNonceResponseLimits`, every JSON value fits configured depth and
aggregate-node limits, and no object at any depth repeats a decoded member
name.

Limits SHALL default to 16,384 complete JSON bytes, depth 16, 256 nodes, and
4,096 decoded nonce bytes. Every maximum SHALL be positive, configurable depth
SHALL NOT exceed the repository maximum, and aggregate oversize SHALL fail
before input is copied.

Success SHALL prove only bounded OID4VCI Final Nonce Response body syntax. It
SHALL NOT prove HTTP status, media type, cache headers, DPoP headers, endpoint
or response provenance, Issuer identity/trust, nonce unpredictability,
freshness, expiry, correlation, uniqueness, or replay safety.

#### Scenario: minimal Final response succeeds

- **WHEN** a bounded object contains `{"c_nonce":"wKI4LT17ac15ES9bw8ac4"}`
- **THEN** parsing succeeds and exposes only the response byte count and the
  deliberately sensitive nonce boundary

#### Scenario: limits reject unbounded work

- **WHEN** any configured maximum is zero, depth exceeds the repository
  maximum, or input exceeds its aggregate byte bound
- **THEN** parsing fails with the corresponding static limit error

#### Scenario: duplicate, deep, or wide extension JSON fails

- **WHEN** any nested object repeats a decoded member name or the document
  exceeds configured depth or node count
- **THEN** the existing duplicate/depth/node error is returned without input
  content

### Requirement: The Credential Nonce remains opaque and exact

`c_nonce` SHALL be required exactly once as a non-empty JSON string and SHALL
fit its independent decoded UTF-8 byte bound. `CredentialNonce` SHALL retain
the exact decoded string, including valid Unicode, without imposing ASCII,
base64url, entropy, fixed-length, or format-profile restrictions absent from
OID4VCI Final.

The nonce SHALL be held in zeroizing storage and available only through
`expose_sensitive_nonce`, documented for immediate proof construction and
against logs, telemetry, URLs, caches, generic serialization, and unrelated or
long-lived storage.

#### Scenario: opaque Unicode challenge remains exact

- **WHEN** `c_nonce` is a bounded non-empty JSON string containing Unicode
- **THEN** the exact decoded value is retained without re-encoding or profile
  rejection

#### Scenario: invalid nonce fails closed

- **WHEN** `c_nonce` is absent, duplicated, empty, mistyped, or exceeds its
  decoded byte bound
- **THEN** parsing fails with a static response/nonce error and never echoes the
  value

### Requirement: Extensions and diagnostics remain isolated

Unknown member names and values SHALL be syntax-, duplicate-, depth-, and
node-checked and then discarded semantically. Historical
`c_nonce_expires_in` SHALL NOT be exposed as OID4VCI Final behavior.

The response SHALL retain only its exact input byte count and nonce; it SHALL
NOT retain raw JSON or unknown values. Response and nonce types SHALL have no
Clone, Display, Serde, raw JSON, automatic network, or FFI surface. Debug SHALL
expose only response byte count. Every new error SHALL be fieldless and map to
a stable static `oid4vci.*` code/message through the existing core bridge.

The change SHALL add no dependency, manifest, lockfile, feature, unsafe,
consumer, chain, or product change and SHALL preserve Rust 1.85,
browser-WASM, Android ARM64, and iOS ARM64 portability.

#### Scenario: bounded extensions are ignored safely

- **WHEN** a valid response also contains bounded unknown nested values or a
  historical `c_nonce_expires_in` member
- **THEN** parsing succeeds without retaining or exposing their names or values

#### Scenario: diagnostics contain no remote content

- **WHEN** a caller inspects response/nonce Debug, response byte count, direct
  errors, or bridged errors
- **THEN** no nonce, response content, parser cause, extension data, endpoint,
  or header value appears

#### Scenario: portable policy-neutral gates remain green

- **WHEN** focused, workspace, factory, target/MSRV, supply-chain, and full Nix
  gates run
- **THEN** the nonce response core passes without dependency-cone, feature,
  target, or downstream drift
