# Design: Final Nonce Endpoint metadata exposure

## Context

OpenID4VCI 1.0 Final sections 7 and 12.2.4 define an optional
`nonce_endpoint` Credential Issuer Metadata member. When present, it is the
HTTPS URL to which a wallet sends an unprotected POST Nonce Request. When
omitted, the Credential Issuer does not require a `c_nonce` value. The URL may
include a port, path and query component.

The existing issuer-metadata core already validates and retains the required
Credential Endpoint with an exact bounded HTTPS URL policy. The nonce endpoint
has the same URL constraints. Expanding the existing ten-argument public
limits constructor solely to name a second equal endpoint budget would create
avoidable source incompatibility.

Oxid provides an immutable OID4VCI Final metadata fixture and adapter evidence.
Lace ID Portal provides older issuer behavior evidence but does not currently
advertise the Final metadata member. These sources remain read-only; the Final
standard governs this SDK boundary and no source or fixture is copied.

## Goals and non-goals

Goals:

- recognize the optional `nonce_endpoint` exactly once in strict bounded
  issuer metadata JSON;
- retain the exact decoded HTTPS URL, including valid port, path and query;
- reject empty, mistyped, malformed, non-HTTPS, userinfo-bearing,
  fragment-bearing and oversized values with static field-specific errors;
- expose omission as `None` and document its Final semantic meaning;
- preserve redaction, dependency, feature, target and consumer boundaries.

Non-goals:

- Nonce Request construction, POST execution, redirects, status/media/cache or
  DPoP header validation;
- Credential Nonce Response parsing changes, issuer nonce generation,
  unpredictability, origin, correlation, freshness, expiry or replay state;
- proof construction or verification, Credential Requests/Responses, trust,
  FFI, consumer adoption, publication, release or `main`.

## Decisions

### Model an optional exact endpoint value

`CredentialIssuerMetadata` owns `Option<NonceEndpoint>`. `NonceEndpoint`
retains the exact decoded string in zeroizing storage and offers only an
`as_str` borrow. Its Debug implementation prints the type shape without the
remote URL. The metadata accessor returns `Option<&NonceEndpoint>`; absence is
not an error and means the issuer does not require `c_nonce` under Final.

The type proves bounded metadata syntax and local URL validation only. It does
not prove that the endpoint is reachable, issuer-controlled, trusted or safe
to contact in a particular network environment.

### Apply the existing HTTPS endpoint validator

The established endpoint validator requires an absolute `https` URL with a
host and rejects credentials/userinfo and fragments. It already admits the
Final-permitted port, path and query components. Reusing it keeps the endpoint
policy consistent without expanding the dependency cone.

### Share the existing endpoint byte budget without breaking callers

`CredentialIssuerMetadataLimits::max_credential_endpoint_bytes` governs each
decoded Credential or Nonce Endpoint URL independently. The constructor and
field retain their current names for source compatibility; documentation makes
the shared policy explicit. This is stricter than an unbounded extension and
does not reduce the budget for the required Credential Endpoint.

### Preserve strict parser and diagnostic behavior

The known member is parsed through the shared bounded-string path and therefore
consumes the aggregate JSON node budget and participates in decoded duplicate
name rejection. Empty or mistyped values map to `InvalidMetadata`; a value over
the shared URL budget maps to `NonceEndpointTooLarge`; an invalid URL maps to
`UnsafeNonceEndpoint`. New diagnostics remain fieldless and bridge to stable
static `oid4vci.*` codes without remote content.

## Verification

- Positive tests cover omission, the Final example shape, and exact retention
  of HTTPS URLs with port, path and query.
- Negative tests cover empty, mistyped, duplicate, non-HTTPS, malformed,
  userinfo-bearing, fragment-bearing and exactly oversized values.
- Canary tests cover endpoint and metadata Debug plus direct and bridged
  errors.
- Focused tests run with all features and no default features; workspace,
  strict Clippy/docs, factory, Rust 1.85, WASM/mobile, supply-chain and full Nix
  gates remain mandatory.
- Consumer HEAD/status and referenced-file hashes must match preflight.

## Provenance

- Normative source: OpenID4VCI 1.0 Final HTML SHA-256
  `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`,
  sections 7 and 12.2.4.
- Oxid evidence: `MediaNoxLabs/oxid@5ba38b9b`; metadata fixture SHA-256
  `7c8562a13310722ba2b554daaa1fd5f9e44e7757c6c2689ada0f85425e39aa71`
  and adapter SHA-256
  `79122b8fc78251e50773a7effeeaf8162747411b9b89283d977e48a2b7f164af`.
- Lace evidence: private read-only
  `input-output-hk/lace-id-portal@804de0a9`; well-known handler SHA-256
  `4dff93d21e02b221598c325a8afdc9ff7311b3c5041727652cea9ecac81419b1`
  and route evidence SHA-256
  `573004e23f015a7592dbab71675bef4ee0bf9c5eb70f0f31a44d600082ca4690`.
  Its repository declares no detected SPDX license, so it is behavior evidence
  only and no code or fixture is copied.
