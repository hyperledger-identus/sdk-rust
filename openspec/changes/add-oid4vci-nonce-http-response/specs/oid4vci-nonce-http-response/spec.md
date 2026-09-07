## ADDED Requirements

### Requirement: Final Credential Nonce response envelope is mandatory and bounded

The SDK SHALL expose positive `CredentialNonceHttpResponseLimits` that combine
one `CredentialNonceResponseLimits` body policy with independent maximum byte
lengths for caller-supplied effective Content-Type and Cache-Control field
values. Zero field limits SHALL fail with a fieldless static invalid-limits
error. Defaults SHALL retain the existing body defaults and use finite field
bounds.

`CredentialNonceRequest::validate_response` SHALL borrow the validated request
and accept an HTTP status code, effective Content-Type, effective Cache-Control,
UTF-8 JSON body and those limits. It SHALL return the existing
`CredentialNonceResponseCore` only after every transport and body requirement
passes. It SHALL retain no status, field value, endpoint or raw body and SHALL
NOT execute HTTP.

#### Scenario: bounded Final response reaches nonce semantics

- **WHEN** a request is paired with a 2xx response, syntactically valid
  `application/json`, valid Cache-Control containing bare `no-store`, and a
  bounded valid nonce body
- **THEN** validation returns the existing response core and exact zeroizing
  nonce

#### Scenario: invalid limits fail before response processing

- **WHEN** either HTTP field byte limit is zero
- **THEN** limit construction fails with static invalid-input metadata and no
  response value is inspected or retained

#### Scenario: transport failure does not parse the body

- **WHEN** status or a required field is invalid and the body is independently
  malformed or oversized
- **THEN** validation returns the transport error without exposing, retaining
  or reporting body content

### Requirement: Final success status and JSON media type fail closed

The response status SHALL be an HTTP success code from 200 through 299
inclusive. Every other `u16`, including values outside the valid HTTP status
range, SHALL fail with one fieldless static status error.

Content-Type SHALL be present, within its byte bound and parse as exactly one
RFC 9110 media type after outer optional whitespace. Its type and subtype SHALL
compare case-insensitively as exactly `application` and `json`. Syntactically
valid semicolon parameters SHALL be accepted and ignored; empty, missing,
different, ambiguous/comma-combined, malformed, injected or oversized values
SHALL fail without MIME sniffing.

#### Scenario: every 2xx boundary is successful

- **WHEN** status is 200 or 299 and all other response inputs are valid
- **THEN** the response reaches bounded body parsing

#### Scenario: non-success and non-HTTP codes fail

- **WHEN** status is 199, 300, zero, or greater than 599
- **THEN** validation returns the same static status error before body parsing

#### Scenario: JSON media type casing and parameters interoperate

- **WHEN** Content-Type uses any ASCII casing of `application/json` with valid
  token or quoted-string parameters
- **THEN** its media type passes without retaining or interpreting parameters

#### Scenario: malformed or ambiguous media type fails

- **WHEN** Content-Type is absent/empty, not `application/json`, contains a
  second comma-combined media type, invalid token/parameter syntax, unterminated
  quoting, forbidden controls, injection bytes, or exceeds its bound
- **THEN** validation returns the corresponding static media error and does
  not sniff or parse the body

### Requirement: Final no-store directive is parsed rather than searched

Cache-Control SHALL be present, within its byte bound and parse as an RFC 9111
comma list of case-insensitive token directives with optional token or
quoted-string arguments. RFC list-compatible empty elements and unknown valid
extensions SHALL be ignored. A response SHALL pass only when at least one
directive is case-insensitively exactly `no-store` and that occurrence has no
argument.

Substring matches, quoted values, directive names such as `x-no-store`, and
`no-store` occurrences with token or quoted-string arguments SHALL NOT satisfy
the requirement. Empty, missing, malformed, injected or oversized values SHALL
fail with fieldless static cache-control errors.

#### Scenario: bare no-store among extensions succeeds

- **WHEN** a valid combined Cache-Control value contains an unqualified
  case-insensitive `no-store` among valid known or unknown directives
- **THEN** uncacheability validation passes regardless of directive order

#### Scenario: quoted delimiters do not create directives

- **WHEN** an extension argument contains commas, semicolons or the text
  `no-store` inside a valid quoted string but no bare `no-store` directive
- **THEN** validation fails as missing the required directive

#### Scenario: malformed or qualified no-store fails

- **WHEN** Cache-Control contains only `x-no-store`, `no-store=value`,
  `no-store="value"`, invalid tokens, unterminated quoting, invalid escapes,
  forbidden controls, injection bytes, or an oversized value
- **THEN** validation returns the corresponding static cache-control error

### Requirement: HTTP response validation remains policy-neutral and redacted

Every added error SHALL be fieldless and bridge to stable static `oid4vci.*`
code, kind, capability and message metadata. Debug, Display and bridged errors
SHALL contain no status-derived remote content, field value, body, nonce,
endpoint or parser cause. The validation method and limits SHALL expose no
generic header map, raw retained input, Clone of secret state, Serde, FFI or
network executor.

Success SHALL prove only caller-paired Final section 7.2 status/media/cache
metadata and bounded body syntax. It SHALL NOT prove actual request execution,
DNS/TLS/redirect/private-network safety, endpoint reachability or issuer trust,
response origin, compression/framing safety, DPoP behavior, nonce generation,
unpredictability, freshness, expiry, reuse or replay safety, proof correctness,
Credential Request/Response correctness or retry policy.

The change SHALL add no dependency, manifest, lockfile, feature, unsafe,
consumer, chain or product mutation. The runtime cone SHALL remain
`identus-core`, `serde_json`, `uriparse` and `zeroize`, and the change SHALL
preserve Rust 1.85, browser-WASM, Android ARM64 and iOS ARM64 portability.

#### Scenario: diagnostic canaries remain absent

- **WHEN** each limits, status, media, cache and body error plus request/core
  Debug is formatted with unique canaries in every caller-supplied input
- **THEN** no canary or parser cause appears in direct or bridged diagnostics

#### Scenario: consumer adapter retains transport authority

- **WHEN** a native, mobile or browser adapter supplies effective response
  metadata to the validation method
- **THEN** the SDK returns syntax evidence only and the adapter still owns
  network, header collection, DPoP, trust, lifecycle and retry policy

#### Scenario: portable dependency-neutral gates remain green

- **WHEN** focused, workspace, factory, target/MSRV, supply-chain and full Nix
  gates run
- **THEN** the response transition passes without dependency-cone, feature,
  target or downstream drift
