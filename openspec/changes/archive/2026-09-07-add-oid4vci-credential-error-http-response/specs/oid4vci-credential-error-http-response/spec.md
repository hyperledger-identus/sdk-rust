## ADDED Requirements

### Requirement: Credential payload-error HTTP metadata is exact and bounded

The SDK SHALL expose positive `CredentialErrorHttpResponseLimits` composed of
existing `CredentialErrorResponseLimits` and an independent maximum for the
effective Content-Type field value. The Content-Type maximum SHALL be non-zero
and SHALL default to 1,024 bytes.

`CredentialErrorResponseCore::parse_http_response` SHALL accept only status
400. Every other status SHALL return one fieldless invalid-status error before
inspecting Content-Type or body input.

For status 400, the method SHALL bound the complete caller-supplied effective
Content-Type field value and require one case-insensitive `application/json`
media type with RFC-shaped optional whitespace and parameters. It SHALL reject
missing, combined, different, malformed, injection-shaped, duplicate-parameter,
or oversized values before body parsing. It SHALL then delegate to
`CredentialErrorResponseCore::parse` under the supplied body limits without
changing that parser's retained values or errors.

#### Scenario: exact payload-error envelope reaches the bounded body

- **WHEN** status is 400, Content-Type is an RFC-shaped `application/json`
  value, and the body is valid and bounded
- **THEN** validation returns the existing bounded Credential Error Response
  core without retaining status or media input

#### Scenario: another status fails before remote fields

- **WHEN** status is not 400 and Content-Type/body contain unique canaries
- **THEN** validation returns the invalid-status error without parsing or
  exposing either canary

#### Scenario: media input is invalid or excessive

- **WHEN** Content-Type is absent, combined, different, malformed,
  injection-shaped, repeats a parameter case-insensitively, or exceeds its
  positive byte bound
- **THEN** validation returns the corresponding static media error before body
  parsing

### Requirement: Payload errors exclude generic invalid_request safely

After envelope and body validation, the method SHALL reject the exact
case-sensitive error code `invalid_request` with one fieldless static error,
because the Final requires Credential Request payload errors to use the more
specific parameters from section 8.3.1.2 instead of that generic RFC 6750
value.

The seven known Final codes SHALL retain their existing classifications. Every
other syntactically valid code SHALL remain exact and classified as
`Extension`. The method SHALL NOT interpret authorization errors, retryability,
blame, remediation, nonce freshness, proof validity, denial finality, or UI
behavior.

#### Scenario: known payload error succeeds

- **WHEN** a valid status/media envelope contains any known Final Credential
  Endpoint payload-error code
- **THEN** the existing exact code and known classification are returned

#### Scenario: generic invalid_request is not a payload error

- **WHEN** a valid status/media envelope contains exact `invalid_request`
- **THEN** validation fails with the static generic-code-forbidden error

#### Scenario: another extension remains interoperable

- **WHEN** a valid status/media envelope contains another bounded valid
  extension code
- **THEN** its exact value survives and classification remains `Extension`

### Requirement: HTTP validation remains least-authority and portable

The implementation SHALL reuse the private RFC-shaped media parser and SHALL
NOT add a Cache-Control input or requirement. The Final example containing
`Cache-Control: no-store` SHALL NOT be treated as normative protocol behavior.
RFC 6750 Authorization Error Responses and authentication challenges SHALL
remain outside this type.

Success SHALL prove only syntax of caller-supplied status, effective media
value, and bounded payload-error body. It SHALL NOT prove HTTP execution,
transport confidentiality/authenticity, endpoint or issuer provenance,
request correlation, issuer truth, retry or replay safety, remediation,
localization, display safety, credential validity, trust, storage, or product
policy.

Every new error SHALL be fieldless and map to a stable static `oid4vci.*`
code/message. No status, media, body, code, description, parser cause, or
canary SHALL appear in direct or bridged diagnostics. No dependency, manifest,
lockfile, feature, unsafe-code, consumer, chain, or product change SHALL occur,
and Rust 1.85, browser-WASM, Android ARM64, and iOS ARM64 portability SHALL
remain green.

#### Scenario: body errors and redaction properties survive composition

- **WHEN** a valid status/media envelope reaches an invalid, excessive, or
  canary-bearing body
- **THEN** the existing body error is returned and all diagnostics remain free
  of remote content

#### Scenario: example-only header is not required

- **WHEN** the caller supplies no Cache-Control input
- **THEN** an otherwise valid payload-error response can succeed

#### Scenario: portable dependency gates remain green

- **WHEN** focused, workspace, factory, target/MSRV, supply-chain, and full Nix
  gates run
- **THEN** HTTP validation passes without dependency, feature, target, or
  downstream drift
