## ADDED Requirements

### Requirement: Credential Error Response core is strict, partial, and bounded

The SDK SHALL parse one top-level JSON object into `CredentialErrorResponseCore`
only when the complete input fits positive `CredentialErrorResponseLimits`,
every JSON value fits configured depth and aggregate-node limits, and no object
at any depth repeats a decoded member name.

Limits SHALL default to 32,768 complete JSON bytes, depth 16, 512 nodes, 256
decoded error-code bytes, and 4,096 decoded description bytes. Every maximum
SHALL be positive, and configurable depth SHALL NOT exceed the repository
maximum. Aggregate oversize SHALL fail before input is copied.

Success SHALL prove only bounded Credential Error Response body syntax. It
SHALL NOT prove HTTP status, headers, media type, authentication challenge,
response provenance/correlation, issuer identity or truth, trust, retryability,
blame, remediation, localization, or a truthful error classification.

#### Scenario: minimal Final error succeeds

- **WHEN** a bounded object contains `{"error":"invalid_proof"}`
- **THEN** parsing succeeds with the optional description absent

#### Scenario: limits reject unbounded work

- **WHEN** any configured maximum is zero, depth exceeds the repository
  maximum, or input exceeds its aggregate byte bound
- **THEN** parsing fails with the corresponding static limit error

#### Scenario: duplicate, deep, or wide extension JSON fails

- **WHEN** any nested object repeats a decoded member name or the document
  exceeds configured depth or node count
- **THEN** the existing duplicate/depth/node error is returned without input
  content

### Requirement: Error codes preserve extensions and classify Final values

`error` SHALL be required exactly once as a non-empty JSON string satisfying
the Final NQSCHAR-compatible ASCII grammar and its independent decoded-byte
bound. The exact code SHALL be retained in `CredentialEndpointErrorCode`.

`CredentialEndpointErrorKind` SHALL classify exact case-sensitive values
`invalid_credential_request`, `unknown_credential_configuration`,
`unknown_credential_identifier`, `invalid_proof`, `invalid_nonce`,
`invalid_encryption_parameters`, and `credential_request_denied`; every other
valid code SHALL be classified as `Extension`. Classification SHALL NOT decide
retry, blame, remediation, user messaging, proof state, or response truth.

#### Scenario: all standard classes are recognized exactly

- **WHEN** each Final Credential Endpoint error code is parsed
- **THEN** its exact corresponding known classification is returned

#### Scenario: extension code remains interoperable

- **WHEN** a bounded valid unregistered error code is parsed
- **THEN** its exact value is retained and its classification is `Extension`

#### Scenario: invalid code grammar fails closed

- **WHEN** `error` is missing, empty, mistyped, non-ASCII, or contains quote,
  backslash, or control characters
- **THEN** parsing fails with a static code/response error

### Requirement: Optional description is bounded and explicitly untrusted

`error_description`, when present, SHALL be a non-empty JSON string satisfying
the Final NQSCHAR-compatible grammar and its independent bound. It SHALL be
retained in zeroizing storage and available only through
`expose_untrusted_description`, documented as remote developer information
that is not localized, trusted, or inherently safe for display.

The parser SHALL NOT recognize `c_nonce`, `error_uri`, or other fields from
different endpoints or superseded drafts as semantic members. Such names SHALL
be treated only as bounded unknown extensions and discarded.

#### Scenario: valid description is retained behind an untrusted boundary

- **WHEN** a valid bounded `error_description` is present
- **THEN** its exact value and presence are available only through the
  deliberately untrusted access boundary

#### Scenario: unsafe optional description fails

- **WHEN** the description is empty, mistyped, oversized, non-ASCII, or
  contains excluded characters
- **THEN** parsing fails without echoing the offending value

#### Scenario: legacy and foreign fields gain no semantics

- **WHEN** bounded `c_nonce`, `error_uri`, or other unknown members are present
- **THEN** parsing may succeed but no value or behavior for them is retained

### Requirement: Extensions and diagnostics remain isolated

Unknown member names SHALL be syntax-, duplicate-, depth-, and node-checked
and then discarded semantically. The response SHALL retain only byte count,
known bounded strings, and classification; it SHALL NOT retain raw JSON or
unknown values.

All retained strings SHALL use zeroizing storage. Response and code types SHALL
have no Clone, Display, Serde, raw JSON, automatic network, or FFI surface.
Debug SHALL expose only response byte count, known classification, and optional
description presence. Every new error SHALL be fieldless and map to a stable
static `oid4vci.*` code/message through the existing core bridge.

The change SHALL add no dependency, manifest, lockfile, feature, unsafe,
consumer, chain, or product change and SHALL preserve Rust 1.85,
browser-WASM, Android ARM64, and iOS ARM64 portability.

#### Scenario: bounded extensions are ignored safely

- **WHEN** a valid response also contains bounded unknown nested values
- **THEN** parsing succeeds without retaining or exposing their names or values

#### Scenario: diagnostics contain no remote content

- **WHEN** a caller inspects response/code Debug, field presence, byte count,
  direct errors, or bridged errors
- **THEN** no response content, description, parser cause, or extension data
  appears

#### Scenario: portable policy-neutral gates remain green

- **WHEN** focused, workspace, factory, target/MSRV, supply-chain, and full Nix
  gates run
- **THEN** the error core passes without dependency-cone, feature, target, or
  downstream drift
