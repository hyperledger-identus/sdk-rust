# Design: bounded Final Credential Nonce HTTP response validation

## Context

OpenID4VCI 1.0 Final section 7.2 requires a Credential Nonce Response to use a
2xx HTTP status, the `application/json` media type, and a Cache-Control field
including `no-store`. The SDK already provides `CredentialNonceRequest` and a
strict `CredentialNonceResponseCore`, but a caller can currently parse the
body without demonstrating those mandatory transport properties.

RFC 9110 makes media type type/subtype tokens case-insensitive and defines
token, quoted-string, parameter and list syntax. RFC 9111 defines
Cache-Control as a case-insensitive list of directives and permits no argument
for `no-store`. The SDK has no HTTP dependency and must remain usable by native,
mobile and browser-WASM consumers with their own transport adapters.

No consumer implementation is needed to resolve this behavior: the Final
standard and HTTP RFCs are controlling. Oxid remains at
`5ba38b9bbc9326c294b353daaf2a074eca18c22f` with a clean worktree. Lace ID
Portal remains at `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` with its pre-existing
`.pi-subagents/`, `.pi/` and `tmp/` untracked paths. Neither consumer was read
for or modified by this slice.

## Goals / Non-Goals

**Goals:**

- reject non-2xx, non-JSON or cacheable nonce responses before body parsing;
- accept case-insensitive `application/json` with syntactically valid
  parameters and case-insensitive bare `no-store` among valid extensions;
- bound both field values independently and reuse the bounded body parser;
- keep response metadata, body and nonce absent from diagnostics;
- retain no transport input beyond the existing zeroizing nonce result;
- preserve the current dependencies, features, MSRV and supported targets.

**Non-Goals:**

- HTTP execution, DNS, TLS, redirect, proxy or private-network policy;
- compression, transfer framing, generic headers or multiple-field collection;
- DPoP nonce syntax, retention or proof construction;
- endpoint/issuer trust, response provenance beyond caller pairing, retries,
  remediation, nonce generation, freshness, expiry, reuse or replay state;
- Credential Request/Response, FFI, consumer adoption, publication, release or
  `main` promotion.

## Decisions

### Validate through the request without consuming it

`CredentialNonceRequest::validate_response` accepts caller-supplied effective
status, Content-Type, Cache-Control and body values plus
`CredentialNonceHttpResponseLimits`. It borrows the request and returns the
existing `CredentialNonceResponseCore` only after the envelope and body pass.

Borrowing allows a product-owned retry policy to reuse the request. Calling the
method records local pairing with a validated request description but does not
claim that the caller actually sent it, that the response came from its
endpoint, or that network policy passed.

Alternatives rejected:

- a built-in HTTP client would select runtime, network and redirect policy;
- a free response parser would not connect the transition to a validated
  advertised request;
- consuming the request would impose one-shot retry policy not required by
  Final;
- boolean `is_json`/`is_uncacheable` inputs would let adapters bypass the
  actual field-value grammar.

### Wrap existing body limits with bounded HTTP metadata limits

`CredentialNonceHttpResponseLimits` contains one existing
`CredentialNonceResponseLimits` plus positive maximum byte lengths for the
effective Content-Type and Cache-Control values. Its default uses the existing
body defaults and conservative 1,024-byte and 4,096-byte field bounds.

This additive wrapper avoids changing the existing body-limit constructor or
assigning header semantics to JSON limits. Limits are copied by value and no
header input is retained.

### Parse only the RFC grammar required for safe classification

A private byte parser validates:

- Content-Type as one RFC 9110 media type, after outer OWS, with token
  type/subtype and zero or more syntactically valid semicolon parameter
  elements, including empty elements permitted by the RFC grammar;
- Cache-Control as an RFC 9111 comma list of token directives with optional
  token or quoted-string arguments, permitting empty list elements as the HTTP
  list extension does;
- quoted strings with valid quoted-pair handling and no CR, LF, NUL or other
  forbidden controls.

The Content-Type succeeds only when type/subtype is case-insensitively exactly
`application/json`. Parameters are syntax-checked and ignored because they do
not change the selected type/subtype in this layer. Cache-Control succeeds only
for a case-insensitive `no-store` directive without an argument. A quoted or
token argument on a directive named `no-store` does not satisfy Final.

The parser never lowercases, copies or returns field values. This avoids
substring errors such as `x-no-store`, quoted `"no-store"`, or delimiters inside
quoted extension arguments.

### Fail before exposing or parsing body semantics

Validation order is limits, status, Content-Type, Cache-Control, then the
existing body parser. Envelope failures therefore return their fieldless static
error even when the supplied body is malicious. Successful output exposes only
the existing response length and zeroizing nonce.

Additive errors distinguish invalid limits, unsuccessful status, oversize or
invalid Content-Type, and oversize or invalid/missing Cache-Control. All bridge
to stable static invalid-input metadata and carry no input.

## Risks / Trade-offs

- **Adapters can supply the wrong effective field value** -> document that the
  caller owns HTTP field collection and response provenance; no local API can
  prove network origin without taking transport authority.
- **A hand-written parser can diverge from HTTP grammar** -> keep it private,
  narrowly scoped, byte-bounded and table-tested against delimiter, quoting,
  casing, OWS and control-character cases from RFC 9110/9111.
- **Accepting unknown media parameters may hide profile semantics** -> this
  layer validates only the required type/subtype; no parameter is interpreted
  as trust or content transformation.
- **Ignoring DPoP-Nonce loses optional information** -> leave the header with
  the caller until a separately contracted DPoP capability can own its syntax,
  proof and replay semantics.

## Migration Plan

Land as an additive unpublished API. Existing body-only callers remain source
compatible; security-sensitive consumers can opt into the request-bound
transition. Rollback is one revert before publication or downstream adoption.

## Open Questions

None. Final and the HTTP RFCs determine the required behavior, and optional
DPoP/lifecycle behavior remains explicitly deferred.

## Provenance

- OpenID4VCI 1.0 Final section 7.2, HTML SHA-256
  `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.
- RFC 9110 sections 5.5, 5.6, 8.3, 8.3.1 and 15.2, HTML SHA-256
  `d431760660ea44e130f6e919dab216df2d0b3a490567a98089267523368fe1e5`.
- RFC 9111 sections 5.2 and 5.2.2.5, HTML SHA-256
  `ce91ee9848d2b9ac46386b0f0cbd4bfd9c0cd1948ebc363834cadc7e0997f3d7`.
- No production source or fixture is copied or transformed.
