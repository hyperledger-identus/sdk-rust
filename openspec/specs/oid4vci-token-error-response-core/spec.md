# oid4vci-token-error-response-core Specification

## Purpose
TBD - created by archiving change add-oid4vci-token-error-response-core. Update Purpose after archive.
## Requirements
### Requirement: Token Error Response core is strict, partial, and bounded

The SDK SHALL parse one top-level JSON object into `TokenErrorResponseCore`
only when the complete input fits positive `TokenErrorResponseLimits`, every
JSON value fits configured depth and aggregate-node limits, and no object at
any depth repeats a decoded member name.

Limits SHALL default to 32,768 complete JSON bytes, depth 16, 512 nodes, 256
decoded error-code bytes, 4,096 decoded description bytes, and 2,048 decoded
URI bytes. Every maximum SHALL be positive, and configurable depth SHALL NOT
exceed the repository maximum. Aggregate oversize SHALL fail before input is
copied.

Success SHALL prove only bounded OAuth Token Error Response syntax. It SHALL
NOT prove HTTP status, headers, media type, cache behavior, authentication
challenge, response provenance/correlation, server/client identity, trust,
retryability, remediation, or a truthful error classification.

#### Scenario: minimal Final error succeeds

- **WHEN** a bounded object contains `{"error":"invalid_request"}`
- **THEN** parsing succeeds with both optional members absent

#### Scenario: limits reject unbounded work

- **WHEN** any configured maximum is zero, depth exceeds the repository
  maximum, or input exceeds its aggregate byte bound
- **THEN** parsing fails with the corresponding static limit error

#### Scenario: duplicate, deep, or wide extension JSON fails

- **WHEN** any nested object repeats a decoded member name or the document
  exceeds configured depth or node count
- **THEN** the existing duplicate/depth/node error is returned without input
  content

### Requirement: Error codes preserve extensions and classify RFC values

`error` SHALL be required exactly once as a non-empty JSON string satisfying
RFC 6749 `1*NQSCHAR` and its independent decoded-byte bound. The exact code
SHALL be retained in `TokenEndpointErrorCode`.

`TokenEndpointErrorKind` SHALL classify exact case-sensitive values
`invalid_request`, `invalid_client`, `invalid_grant`, `unauthorized_client`,
`unsupported_grant_type`, and `invalid_scope`; every other valid code SHALL be
classified as `Extension`. Classification SHALL NOT decide retry, blame,
remediation, Transaction Code state, or response truth.

#### Scenario: all standard classes are recognized exactly

- **WHEN** each RFC token-endpoint error code is parsed
- **THEN** its exact corresponding known classification is returned

#### Scenario: extension code remains interoperable

- **WHEN** a bounded valid unregistered error code is parsed
- **THEN** its exact value is retained and its classification is `Extension`

#### Scenario: invalid code grammar fails closed

- **WHEN** `error` is missing, empty, mistyped, non-ASCII, or contains quote,
  backslash, or control characters
- **THEN** parsing fails with a static code/response error

### Requirement: Optional error metadata is bounded and explicitly untrusted

`error_description`, when present, SHALL be a non-empty JSON string satisfying
RFC `1*NQSCHAR` and its independent bound. It SHALL be retained in zeroizing
storage and available only through `expose_untrusted_description`, documented
as developer information that is not localized, trusted, or inherently safe
for display.

`error_uri`, when present, SHALL be a non-empty JSON string satisfying the RFC
character set and URI-reference syntax under its independent bound. It SHALL
be retained by a redacted `TokenErrorUri` wrapper that exposes the exact string
but provides no resolution, dereference, navigation, or network operation.
Relative references SHALL remain valid.

#### Scenario: complete response retains validated metadata

- **WHEN** valid description and URI-reference fields are present
- **THEN** exact values and presence are available through the deliberately
  untrusted/validated access boundaries

#### Scenario: unsafe optional metadata fails

- **WHEN** description or URI is empty, mistyped, oversized, non-ASCII,
  contains excluded characters, or the URI is syntactically invalid
- **THEN** parsing fails without echoing the offending value

### Requirement: Extensions and diagnostics remain isolated

Unknown member names SHALL be syntax-, duplicate-, depth-, and node-checked
and then discarded semantically. The response SHALL retain only byte count,
known bounded strings, and classification; it SHALL NOT retain raw JSON or
unknown values.

All retained strings SHALL use zeroizing storage. Response, code, and URI types
SHALL have no Clone, Display, Serde, raw JSON, automatic network, or FFI
surface. Debug SHALL expose only response byte count, known classification, and
optional-member presence. Every new error SHALL be fieldless and map to a
stable static `oid4vci.*` code/message through the existing core bridge.

The change SHALL add no dependency, manifest, lockfile, feature, unsafe,
consumer, chain, or product change and SHALL preserve Rust 1.85,
browser-WASM, Android ARM64, and iOS ARM64 portability.

#### Scenario: bounded extensions are ignored safely

- **WHEN** a valid response also contains bounded unknown nested values
- **THEN** parsing succeeds without retaining or exposing their names or values

#### Scenario: diagnostics contain no remote content

- **WHEN** a caller inspects response/code/URI Debug, field presence, byte
  count, direct errors, or bridged errors
- **THEN** no response content, URI, description, parser cause, or extension
  data appears

#### Scenario: portable policy-neutral gates remain green

- **WHEN** focused, workspace, factory, target/MSRV, supply-chain, and full Nix
  gates run
- **THEN** the error core passes without dependency-cone, feature, target, or
  downstream drift
