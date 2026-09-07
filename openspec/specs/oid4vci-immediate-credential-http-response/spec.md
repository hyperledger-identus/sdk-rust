# oid4vci-immediate-credential-http-response Specification

## Purpose
TBD - created by archiving change add-oid4vci-immediate-response-binding. Update Purpose after archive.
## Requirements
### Requirement: Immediate Credential HTTP metadata is exact and bounded

The SDK SHALL expose positive `ImmediateCredentialHttpResponseLimits` composed
of existing `ImmediateCredentialResponseLimits` and an independent maximum for
the effective Content-Type field value. Every maximum SHALL be non-zero,
invalid limits SHALL fail with one fieldless static invalid-limits error, and
defaults SHALL be finite.

`JwtCredentialRequest::validate_immediate_response` SHALL accept only status
200 for an immediate response. Status 202 SHALL return the existing explicit
unsupported-deferred error before inspecting Content-Type or body input. Every
other status SHALL return a fieldless invalid-status error before inspecting
Content-Type or body input.

For status 200, the method SHALL bound and parse the complete caller-supplied
effective Content-Type field value and require one case-insensitive
`application/json` media type with RFC-shaped optional whitespace and
parameters. It SHALL reject missing, combined, different, malformed,
injection-shaped or oversized values before parsing the body. It SHALL then
parse the body through `ImmediateCredentialResponseCore` under the supplied
body limits without changing that parser's errors or retained values.

#### Scenario: exact immediate envelope reaches the bounded body

- **WHEN** status is 200, Content-Type is an RFC-shaped `application/json`
  value with valid optional parameters, and the body is valid and bounded
- **THEN** validation reaches request-cardinality binding and retains no status
  or header input

#### Scenario: deferred status is explicitly unsupported

- **WHEN** status is 202 with arbitrary Content-Type and body canaries
- **THEN** validation returns the unsupported-deferred error without parsing or
  exposing either canary

#### Scenario: another status fails before remote fields

- **WHEN** status is not 200 or 202
- **THEN** validation returns the invalid-status error without parsing or
  exposing Content-Type or body input

#### Scenario: media input is invalid or excessive

- **WHEN** Content-Type is absent, combined, different, malformed,
  injection-shaped or exceeds its positive byte bound
- **THEN** validation returns the corresponding static media error before body
  parsing

### Requirement: Immediate response count is bounded by its request proofs

After envelope and body validation, the method SHALL require the non-empty
response credential count to be no greater than the originating
`JwtCredentialRequest::proof_count`. Equal or fewer credentials SHALL pass;
more credentials SHALL fail with one fieldless count-exceeds-proofs error and
return no partial state.

Success SHALL return `RequestBoundImmediateCredentialResponse`, which SHALL own
the parsed response, expose the non-secret request proof count, borrow the
response, and support consuming itself into that response. It SHALL NOT
implement Clone, Display or Serde. This state proves only the necessary count
upper bound available from the opaque request proofs. It SHALL NOT prove proof
key uniqueness, exact Final key cardinality, or credential-to-key binding.

#### Scenario: equal credential and proof counts pass

- **WHEN** a bounded immediate response contains one credential per request
  proof
- **THEN** the request-bound state preserves the response and reports the
  originating proof count

#### Scenario: issuer returns fewer credentials

- **WHEN** a valid immediate response contains fewer credentials than request
  proofs
- **THEN** validation succeeds as Final permits fewer issued credentials

#### Scenario: response exceeds request proof count

- **WHEN** a valid immediate response contains more credentials than request
  proofs
- **THEN** validation fails with the static count-exceeds-proofs error

### Requirement: Request-bound response validation stays least-authority

`Debug` for the request-bound state SHALL contain only response byte/count/
presence metadata and the request proof count. Direct and bridged errors SHALL
be fieldless and SHALL NOT retain status, headers, body, credential,
notification, bearer or proof content. The request SHALL be borrowed and
remain reusable after success or failure; this SHALL NOT claim HTTP execution,
single-use, retry or replay safety.

The implementation SHALL reuse the existing private RFC-shaped media parser.
Existing Credential Nonce HTTP behavior and tests SHALL remain unchanged.
Cache-Control SHALL NOT become an input or requirement because Final section
8.3 does not normatively require it for Credential Responses.

Success SHALL NOT prove HTTP execution, endpoint/issuer provenance,
transport authenticity/confidentiality, DPoP, response encryption, error or
deferred processing, proof/token trust or freshness, proof-key uniqueness,
credential format validity, cryptographic verification, issuer/schema/status
trust, notification execution, consent, storage, disclosure, retry or replay
safety. No dependency, feature, target, unsafe code, consumer, chain or product
mutation SHALL occur, and Rust 1.85, browser-WASM, Android ARM64 and iOS ARM64
portability SHALL remain green.

#### Scenario: diagnostics remain redacted and request remains reusable

- **WHEN** state Debug and every direct/bridged error are formatted around
  unique header, body, credential, notification, bearer and proof canaries
- **THEN** no canary or input fragment appears and the request can validate a
  later response

#### Scenario: private grammar extraction preserves Nonce behavior

- **WHEN** the existing Credential Nonce HTTP response suite runs after helper
  extraction
- **THEN** all accepted and rejected media/cache cases remain unchanged

#### Scenario: portable dependency gates remain green

- **WHEN** focused, workspace, factory, target/MSRV, supply-chain and full Nix
  gates run
- **THEN** response binding passes without dependency, feature, target or
  downstream drift
