# Design: bounded OID4VCI Pre-Authorized Token Request

## Context

OID4VCI 1.0 Final section 6.1 requires a Token Request for the
Pre-Authorized Code flow to carry the extension grant type, the offered
Pre-Authorized Code, and `tx_code` exactly when requested by the offer. It
uses the OAuth 2.0 Token Endpoint request rules, including UTF-8
`application/x-www-form-urlencoded` encoding. The predecessor state from
#123 already proves input-presence agreement and owns both secrets.

Oxid currently delegates form encoding to its HTTP client and supports only
the no-code path. Lace has historical JSON-shaped route behavior that is not
the normative Token Request representation. Both remain evidence only.

## Goals and non-goals

Goals:

- make mandatory Token Request construction a consuming typed transition;
- produce one deterministic standards-conforming form body;
- retain the previously validated HTTPS Token Endpoint;
- preflight and bound encoded output before allocation;
- erase sensitive source and encoded allocations on drop;
- make raw body access explicit and keep diagnostics data-free.

Non-goals:

- HTTP execution, headers beyond method/media-type guidance, async, retries,
  or replay protection;
- client identity or authentication;
- `authorization_details`, `scope`, `resource`, credential selection, or
  other optional parameters;
- Token Response, nonce, credential request/response, trust, or endpoint
  reachability;
- downstream adoption, publication, release, or `main` work.

## Decisions

### Consume the prepared input state into one transport value

`CredentialOfferWithPreAuthorizedTokenInput::try_into_pre_authorized_token_request`
accepts positive `PreAuthorizedTokenRequestLimits` and returns
`PreAuthorizedTokenRequest`. Construction borrows the validated endpoint and
secrets only while encoding, then consumes and drops the predecessor. The
original secret allocations are therefore erased after the one unavoidable
wire-representation copy is complete.

The result owns a separately zeroizing Token Endpoint value and form body. It
exposes endpoint, static `POST`, static media type, body byte count, and
Transaction Code presence. The exact body is available only via
`expose_sensitive_form_body`, whose name and documentation make the secret
boundary explicit. There is no Clone, Display, or Serde contract.

### Encode one canonical mandatory field sequence

Fields occur exactly once and in fixed order: `grant_type`,
`pre-authorized_code`, then optional `tx_code`. Names are fixed ASCII. Values
are first represented as UTF-8 octets; ASCII alphanumerics and `*`, `-`, `.`,
and `_` remain literal, space becomes `+`, and all other octets use uppercase
`%HH` encoding. This is the RFC 6749 Appendix B profile and matches its
space/percent/ampersand/plus/non-ASCII vector.

Deterministic order is an SDK contract for reproducibility, not an assertion
that servers may require parameter order.

### Bound exact encoded bytes before allocation

`PreAuthorizedTokenRequestLimits` contains one positive maximum form-body
byte count and defaults to 16,384 bytes. Construction uses checked arithmetic
to calculate exact output size before allocating. Zero limits fail
distinctly; overflow and output above the maximum share a static oversized
request error. The implementation adds no dependency.

### Keep transport and security claims narrow

The type proves only that mandatory parameters were constructed from the
validated predecessor and fit the configured bound. It does not prove that a
request was sent once, sent securely, authenticated, accepted, or bound to a
trusted server. The form body contains bearer-adjacent material and must not
enter URLs, logs, telemetry, caches, diagnostics, or generic serialization.

## Public and error contract

Additive public types:

- `PreAuthorizedTokenRequestLimits` with positive construction, a byte
  accessor, and a 16,384-byte default;
- `PreAuthorizedTokenRequest` with validated endpoint, static method/media
  type, byte-count/presence metadata, and one explicitly sensitive body
  accessor.

Additive consuming transition:

- `CredentialOfferWithPreAuthorizedTokenInput::try_into_pre_authorized_token_request`.

Add fieldless errors and stable codes for invalid request limits and oversized
encoded output. Debug and every error surface remain static and data-free.

## Verification

- Positive tests cover fixed order, present/absent `tx_code`, endpoint and
  transport guidance, exact limit, and canonical encoding vectors.
- Negative tests cover zero limits, one-byte-over-limit output, and checked
  diagnostic redaction with distinct canaries.
- Encoding conformance covers space, percent, ampersand, plus, reserved ASCII,
  and multibyte UTF-8 with decoded name/value assertions.
- Focused tests in all-feature and no-default modes, strict Clippy/docs,
  workspace gates, factory preservation/archive, Rust 1.85, WASM/mobile,
  supply-chain, and full Nix checks.
- Consumer HEAD/status and referenced-file hashes must match preflight.

## Provenance

Normative sources are OpenID4VCI 1.0 Final section 6.1, HTML SHA-256
`f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`,
and RFC 6749 sections 3.2, 4.1.3, 4.5, and Appendix B, RFC Editor HTML
SHA-256 `535362fa3b4ca668d4734c244c6ed8c811f2ccd7ea1612a4201e5d8922a74b4e`.

Consumer evidence is behavior-only and Apache-2.0. Exact Oxid/Lace revisions,
paths, and hashes are recorded in issue #125 and the verification receipt; no
consumer source or fixture is copied.
