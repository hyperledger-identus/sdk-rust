# Design: bounded successful Token Response core

## Context

OID4VCI 1.0 Final section 6.2 delegates the successful Token Response core to
OAuth 2.0 and adds optional Authorization Details. RFC 6749 section 5.1
requires `access_token` and `token_type`, defines optional `expires_in`,
`refresh_token`, and `scope`, requires clients to ignore unrecognized member
names, and leaves token sizes undefined. The SDK therefore needs explicit
resource policy without turning a syntactically valid response into a trust or
issuance claim.

Oxid currently accepts exactly three Final fields and rejects extensions. Lace
currently emits historical nonce fields that Final moved to the Nonce Endpoint.
These independent Apache-2.0 implementations are behavior evidence only;
standards requirements control this design.

## Goals and non-goals

Goals:

- parse one bounded top-level JSON object with duplicate rejection;
- validate the RFC 6749 core and preserve exact token-type spelling;
- enforce independent decoded-value bounds in addition to aggregate JSON
  depth, node, and byte bounds;
- keep the exact response and extracted sensitive values zeroizing and private;
- ignore unknown members semantically while recording Authorization Details
  presence for the next typed transition;
- keep every diagnostic static and data-free.

Non-goals:

- HTTP status, headers, media type, caching, endpoint execution, or correlation
  with a request;
- Token Error Responses;
- Authorization Details or Credential Dataset validation;
- token cryptographic validation, bearer/DPoP policy, time evaluation, refresh,
  replay, retry, trust, or storage;
- nonce or Credential Request/Response behavior;
- downstream adoption, publication, release, or `main` work.

## Decisions

### Expose a deliberately partial response state

`TokenResponseCore::parse` accepts borrowed UTF-8 JSON and a positive
`TokenResponseLimits`. It checks the aggregate byte limit before making one
zeroizing owned copy, parses only the known core into separately zeroizing
values, and retains the full exact response privately for a later
Authorization Details transition. There is no public raw-response accessor.

The type proves only bounded syntax for the OAuth core. A boolean records
whether `authorization_details` occurred. That field remains structurally
valid JSON but semantically unvalidated, so a later Credential Request builder
must consume a stronger state before it can use credential identifiers.

### Validate RFC field syntax without choosing token policy

Access and refresh tokens must be non-empty ASCII visible strings (`1*VSCHAR`)
under independent bounds. Token type must be non-empty and satisfy either the
RFC `type-name` grammar or a non-empty URI-reference; its exact spelling is
retained and callers receive an ASCII-case-insensitive comparison helper.

`expires_in` is optional and, when present, must be a non-negative base-10 JSON
integer fitting `u64`. Fractions, exponents, signs, overflow, strings, booleans,
and null fail. Scope is optional and must satisfy the RFC space-delimited
`scope-token` grammar without leading, trailing, or repeated spaces. No token
type or scope value is selected as policy by this capability.

### Bound all work and retain no unprotected internal copy

Defaults are 65,536 total JSON bytes, depth 16, 1,024 nodes, 16,384 access-token
bytes, 256 token-type bytes, 16,384 refresh-token bytes, and 4,096 scope bytes.
Every configurable maximum is positive and depth cannot exceed the repository
maximum. Aggregate structure is scanned before success; decoded known strings
receive their own bounds. The caller still owns and must erase its input.

The exact response, access token, token type, refresh token, and scope use
zeroizing storage. The type has no Clone, Display, or Serde contract. Debug
shows only response bytes, optional-field presence, and Authorization Details
presence. Sensitive values are available only through explicitly named
accessors documented against logs, telemetry, URLs, caches, and long-lived
storage.

### Preserve extension interoperability

Unknown top-level and nested values are fully syntax-, duplicate-, depth-, and
node-checked, then ignored semantically as RFC 6749 and OID4VCI require. This
includes historical `c_nonce` fields. Ignoring a member is not a claim that the
member is current, trusted, or supported.

## Public and error contract

Additive public types:

- `TokenResponseLimits`;
- `TokenType` with exact `as_str` and ASCII-case-insensitive comparison;
- `TokenResponseCore` with response-byte count, optional metadata, presence
  signals, and explicitly sensitive token/scope accessors.

Add static fieldless errors and stable `oid4vci.*` codes for invalid response
limits, aggregate oversize, invalid response/core fields, and independently
oversized known values. Existing duplicate/depth/node errors remain shared.

## Verification

- Positive tests cover the minimal Final response, every optional RFC field,
  case-insensitive token type, ignored unknown and legacy nonce members, and
  Authorization Details presence without semantic exposure.
- Negative tests cover missing/duplicate/wrong-type fields, each grammar,
  zero limits, each independent size bound, aggregate bytes, depth, nodes, and
  negative/fraction/exponent/overflow expiry forms.
- Canary tests cover response Debug plus direct and bridged error Debug/Display.
- Focused tests run in both feature modes with strict Clippy/docs; workspace,
  factory, Rust 1.85, WASM/mobile, supply-chain, and full Nix gates remain
  mandatory.
- Consumer HEAD/status and referenced-file hashes must match preflight.

## Provenance

Normative sources are OpenID4VCI 1.0 Final sections 6.2 and 13.10, immutable
HTML SHA-256
`f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`,
and RFC 6749 section 5.1 plus Appendix A.4/A.12-A.17, RFC Editor HTML SHA-256
`535362fa3b4ca668d4734c244c6ed8c811f2ccd7ea1612a4201e5d8922a74b4e`.

Read-only Apache-2.0 behavior evidence is pinned in issue #127 at Oxid
`5ba38b9b` and Lace ID Portal `804de0a9`; no production source is copied.
