# Design: transport-neutral Final Credential Nonce Request

## Context

OpenID4VCI 1.0 Final section 7.1 defines a Nonce Request as an HTTP POST to the
URL advertised by the Credential Issuer Metadata `nonce_endpoint`. The endpoint
is not a protected resource, so the wallet does not supply an access token.
The Final request defines no parameters; its non-normative wire example has a
zero-length body.

The SDK already has bounded `CredentialIssuerMetadata`, a validated
`NonceEndpoint`, and a strict `CredentialNonceResponseCore`. It intentionally
does not own an HTTP client or product network policy. The next smallest useful
slice is therefore an owned request description that a caller can hand to any
transport adapter without reinterpreting metadata or inventing authorization.

Oxid independently issues a POST with no body to its advertised nonce endpoint
and tests that the body is empty. That Apache-2.0 implementation is read-only
conformance evidence, not a source port. The Final standard governs this SDK
contract.

## Goals and non-goals

Goals:

- construct a request only when validated metadata advertises a Nonce Endpoint;
- retain the exact already-bounded endpoint in independent zeroizing storage;
- expose POST, an exactly empty body, and no access-token requirement;
- keep Debug and errors free of endpoint or other remote content;
- preserve dependency, feature, target, parser and consumer boundaries.

Non-goals:

- HTTP execution, header maps, DNS, TLS, redirects, proxies or private-network
  policy;
- status, media type, cache-control or DPoP response-header validation;
- issuer trust, response provenance/correlation, nonce generation,
  unpredictability, freshness, expiry, reuse or replay state;
- proof or Credential Request construction/verification, FFI, downstream
  adoption, publication, release or `main`.

## Decisions

### Construct from validated metadata

`CredentialIssuerMetadata::try_nonce_request(&self)` returns an owned
`CredentialNonceRequest` only when `nonce_endpoint()` is present. Absence maps
to `NonceEndpointRequired`; no fallback or URL derivation is attempted. The
request duplicates the exact validated endpoint into zeroizing storage so the
transport operation does not borrow or consume the broader metadata state.

The request cannot be constructed directly from an arbitrary string. This
preserves the existing issuer-metadata byte and HTTPS URL gates without adding
a second parser or public unchecked constructor.

### Expose the minimum static transport contract

The request exposes `POST` through `NONCE_REQUEST_HTTP_METHOD`, an empty byte
slice through `NONCE_REQUEST_BODY`, and `access_token_required() == false`.
Because Final defines no request parameters, the SDK selects one canonical
zero-length body. It does not expose a content type or generic headers; callers
must not infer that other authentication or transport policy has been
validated.

This value is a description, not an executor. Its success proves only that a
validated advertised endpoint can be used to form the Final request shape.

### Keep diagnostics and compatibility narrow

`CredentialNonceRequest` Debug reports only the zero body length and the false
access-token requirement. `NonceEndpoint` Debug remains redacted. The new
fieldless error bridges to static `oid4vci.nonce_endpoint_required` metadata.
No Clone, Display, Serde, FFI, automatic HTTP, dependency, feature, manifest or
lockfile surface is added.

## Verification

- Positive tests cover exact endpoint ownership, POST, empty body and no
  access-token requirement.
- Negative tests cover metadata without an endpoint and stable direct/bridged
  error diagnostics.
- Canary tests cover request and endpoint Debug without URL leakage.
- A consumer-shaped test mirrors Oxid's POST/no-body contract without copying
  source or performing network access.
- Focused all-feature/no-default tests, strict Clippy/docs, workspace, factory,
  Rust 1.85, WASM/mobile, supply-chain and full Nix gates remain mandatory.
- Consumer HEAD/status and referenced-file hashes must match preflight.

## Provenance

- Normative source: OpenID4VCI 1.0 Final HTML SHA-256
  `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`,
  section 7.1.
- Oxid evidence: `MediaNoxLabs/oxid@5ba38b9b`; `portal.rs` SHA-256
  `d1a1d975ca8a740da1d51b6d93627a18811b96c9cccf0832ed92c86c9e7d48cb`
  and `portal_internal_tests.rs` SHA-256
  `a1160157356ef3d030993ac50b0c0ba7c6c45da1ed7ee66f634a855526c8354f`.
  Root license and workspace metadata identify Apache-2.0. No code or fixture
  is copied.
- Lace evidence: private read-only
  `input-output-hk/lace-id-portal@804de0a9`; no Final Nonce Endpoint route was
  found in the inspected Rust server surface. Its repository declares no
  detected SPDX license, so it remains behavior evidence only.
