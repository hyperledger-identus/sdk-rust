# Design: bounded Credential Nonce Response core

## Context

OpenID4VCI 1.0 Final section 7 defines an optional, unprotected Nonce Endpoint.
Its successful JSON body has one required `c_nonce` string containing a
challenge for subsequent proof-of-possession construction. The Final text
defines no wire grammar or lifetime field. Unpredictability is an Issuer
generation requirement, while 2xx status, `application/json`, `Cache-Control:
no-store`, and optional `DPoP-Nonce` are HTTP-layer facts.

Oxid currently consumes a Portal-shaped nonce response and pins a positive
fixture. Lace ID Portal has historical nonce generation and token-embedded
nonce behavior. These implementations are evidence only: the Final standard
governs this SDK boundary, and no source is copied.

## Goals and non-goals

Goals:

- parse one bounded top-level JSON object with duplicate rejection;
- require one non-empty opaque `c_nonce` string under an independent decoded
  UTF-8 byte limit;
- accept the full JSON string value space rather than inventing base64url,
  ASCII, entropy, or fixed-length policy;
- zeroize the retained value and expose it only at an explicitly sensitive
  proof-construction boundary;
- keep Debug, direct errors, and bridged errors free of response content;
- fully validate bounded extensions before discarding them.

Non-goals:

- `nonce_endpoint` metadata exposure, endpoint selection, request construction,
  HTTP execution, status/media/cache/header validation, redirects, or DPoP;
- Issuer generation, randomness, unpredictability proof, freshness, expiry,
  reuse, correlation, replay state, trust, or storage;
- proof construction/verification, Credential Request/Response, FFI, consumer
  adoption, publication, release, or `main`.

## Decisions

### Preserve the challenge as an opaque JSON string

`CredentialNonce` owns the decoded value exactly in zeroizing storage. The
only validation beyond JSON string syntax is non-empty content and a decoded
UTF-8 byte bound. The Final specification does not require base64url, ASCII, a
fixed length, or a `c_nonce_expires_in` member, so donor-specific profiles do
not enter the generic type.

`expose_sensitive_nonce` is deliberately named for immediate proof
construction. The core does not claim that the challenge is unpredictable,
fresh, Issuer-originated, single-use, or accepted by a Credential Endpoint.

### Bound all parsing and discard extensions

Defaults are 16,384 complete JSON bytes, depth 16, 256 aggregate nodes, and
4,096 decoded nonce bytes. Every maximum is positive and depth cannot exceed
the repository maximum. Aggregate size fails before copying. The shared strict
scanner checks complete syntax, decoded duplicate member names, depth, nodes,
and known-field type and size. Unknown members are then discarded.

The response retains only its byte count and nonce. It does not retain raw JSON
because no later transition in this slice consumes response extensions.

### Keep transport claims out of the parsed state

`CredentialNonceResponseCore` proves only bounded JSON body semantics. It has
no status, media type, `Cache-Control`, `DPoP-Nonce`, URL, request, network, or
authentication API. The later endpoint/request slice must explicitly carry
POST-with-empty-body and unprotected-resource guidance from Final section 7.1.

### Use static content-free diagnostics

Five fieldless errors distinguish invalid limits, aggregate oversize, invalid
response shape/syntax, invalid empty or mistyped nonce, and decoded nonce
oversize. They bridge to stable `oid4vci.*` codes without offsets, parser
causes, values, JSON, or endpoint data. Response and nonce Debug reveal only
type/response byte count and never implement Clone, Display, Serde, raw JSON,
FFI, or network behavior.

## Verification

- Positive tests cover the Final example, exact Unicode values, surrounding
  JSON whitespace, and ignored bounded nested extensions.
- Negative tests cover missing/duplicate/mistyped/empty nonce, decoded nonce
  size, total bytes, zero/unsupported limits, depth, nodes, malformed/trailing
  JSON, non-object roots, and duplicates inside extensions.
- Canary tests cover response/nonce Debug plus direct and bridged errors.
- Focused tests run with all features and no default features; workspace,
  strict Clippy/docs, factory, Rust 1.85, WASM/mobile, supply-chain, and full
  Nix gates remain mandatory.
- Consumer HEAD/status and referenced-file hashes must match preflight.

## Provenance

- Normative source: OpenID4VCI 1.0 Final HTML SHA-256
  `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.
- Oxid evidence: `MediaNoxLabs/oxid@5ba38b9b`; positive fixture SHA-256
  `20c1d1252e39dc09ada8398584d194f38f55b7f5f4b144099ae7daa1ccb3fbdc`
  and parser SHA-256
  `80ae940e1f74783f406922d90d0239ae1ddb86e01655ca56b2af8a2cd1545b05`.
- Lace evidence: private read-only
  `input-output-hk/lace-id-portal@804de0a9`; token port SHA-256
  `23d356308e7c432f573d92dccc3d408c1218b66b53155aa48c58add5c7545174`.
  Its repository declares no detected SPDX license, so it is behavior evidence
  only and no code or fixture is copied.
