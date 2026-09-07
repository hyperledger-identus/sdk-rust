# Design: bounded Final JWT Credential Request construction

## Context

OpenID4VCI 1.0 Final section 8.2 defines a Credential Request as an HTTP POST
to the Credential Endpoint. An unencrypted request uses `application/json`,
identifies the requested dataset with exactly one of
`credential_configuration_id` or `credential_identifier`, may contain a
non-empty proof-type array, and presents the Access Token. The proof array is
mandatory when the selected metadata configuration advertises proof support.

The current SDK can prove offer/issuer-metadata agreement, expose the validated
Credential Endpoint, parse a bounded Token Response core, obtain and validate a
nonce response, and produce a signed `Oid4vciProofJwt`. It deliberately does
not yet interpret Token Response Authorization Details or the complete opaque
Credential Configuration metadata. The smallest safe next state therefore
supports only the configuration-ID route when Authorization Details are absent
and requires at least one already-produced JWT proof.

Read-only consumer evidence confirms this interoperable shape. Oxid at
`5ba38b9bbc9326c294b353daaf2a074eca18c22f` emits
`credential_configuration_id` with `proofs.jwt`; Lace ID Portal at
`804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` supplies issuer-side configuration
and bearer-token evidence. Both are Apache-2.0 conformance sources; no source or
fixture is copied.

## Goals / Non-Goals

**Goals:**

- construct one deterministic, bounded unencrypted JSON request using a
  configuration ID already present in the matched offer;
- accept only a non-empty bounded ordered slice of holder-produced OID4VCI JWT
  proofs;
- reject the configuration-ID route when unvalidated Authorization Details are
  present;
- accept only the currently supported Bearer token use and own the complete
  Authorization value and JSON body in zeroizing storage;
- expose a validated endpoint and exact static transport guidance without an
  HTTP runtime;
- preserve static diagnostics, Rust 1.85 and native/mobile/browser targets.

**Non-Goals:**

- HTTP, TLS, DNS, redirects, proxies, DPoP or private-network policy;
- Token Response transport, provenance, trust, expiry or refresh policy;
- Authorization Details, Credential identifiers or authorization selection;
- proof construction, validation, nonce freshness/replay or key policy;
- proofless, non-JWT or format-specific requests;
- request/response encryption, Credential Response/error/deferred processing;
- chain/product extensions, FFI, consumer adoption, publication, release or
  `main` promotion.

## Decisions

### Construct through the matched offer/metadata state

`CredentialOfferWithMetadata::try_create_jwt_credential_request` borrows the
matched state and Token Response, selects one offered Credential Configuration
by zero-based index, and borrows a non-empty proof slice. An invalid index fails
before any secret is copied. Selecting from the offer rather than accepting a
raw identifier prevents construction for an unoffered configuration, while the
existing matched state already proves that every offered ID exists in metadata.

The constructor rejects a Token Response whenever
`authorization_details_present()` is true. The Final rules then require
Credential Dataset identifiers whose structure the current core intentionally
does not parse. Refusing that state prevents silently using the mutually
exclusive configuration-ID route.

### Depend on the holder-produced proof type

The request API accepts `&[Oid4vciProofJwt]`. This creates the dependency edge
already shown in the SDK blueprint (`identus-oid4vci -> identus-jose`) and
prevents arbitrary text from being represented as a locally produced JWT key
proof. It does not claim issuer verification, trust, freshness or replay
safety: those depend on caller policy and current nonce context.

Proof order and compact spelling are retained exactly in the encoded body.
Independent positive limits bound proof count, each compact value and the
complete body. JSON escaping is delegated to `serde_json`; no proof is parsed,
normalized or logged in the protocol crate.

### Keep transport data owned, minimal and explicitly sensitive

`JwtCredentialRequest` duplicates the validated HTTPS Credential Endpoint and
stores only an Authorization field value, JSON body and proof count. The Token
Response must advertise `Bearer` case-insensitively. The emitted field uses the
canonical `Bearer ` prefix followed by the exact validated Access Token.

The value exposes `POST`, `application/json`, endpoint, byte counts and proof
count through ordinary accessors. Authorization and body are available only
through explicitly sensitive accessors. Both are `Zeroizing<String>` and
`Debug` reports only lengths/counts. The request has no `Clone`, `Display`,
Serde, generic header map or executor.

### Bound before and after serialization

`JwtCredentialRequestLimits` requires non-zero maximum proof count,
per-proof bytes, body bytes and Authorization bytes. The constructor checks
selection, Authorization Details absence, Bearer type, proof count and each
proof before allocation. Checked arithmetic bounds the Authorization value;
the serialized body is rejected if it exceeds the final body budget.

Fieldless errors distinguish invalid limits, selection, incompatible Token
Response route/type, empty/excessive/oversized proofs, oversized authorization,
serialization failure and oversized body. No variant carries caller data.

## Risks / Trade-offs

- **The metadata parser does not expose proof requirements** -> this narrow
  constructor always requires proofs; a later metadata-profile slice can add a
  separately typed proofless decision if interoperable evidence requires it.
- **Authorization Details could contain no Credential identifiers** -> the core
  does not inspect them, so fail closed and defer the whole route rather than
  guess from an opaque member.
- **Non-Bearer token types exist** -> constructing their authorization syntax
  is type-specific; reject them until a separate token-type capability exists.
- **The proof can be stale for the request context** -> the request preserves a
  holder-produced proof type but intentionally leaves audience/nonce/time and
  replay policy to proof construction and caller state.
- **Owned request data duplicates secrets** -> ownership simplifies headless
  async transport but uses zeroization, explicit accessors, bounded lifetime
  guidance and redacted diagnostics.

## Migration Plan

Land as an additive unpublished API. Existing crates and callers remain source
compatible. A later downstream adoption issue can replace manual JSON only
after a release candidate and consumer compatibility evidence. Rollback is one
focused revert before publication.

## Open Questions

None. Unsupported Final alternatives remain explicit issue-first work.

## Provenance

- OpenID4VCI 1.0 Final section 8.2, HTML SHA-256
  `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.
- Oxid conformance-only source:
  `MediaNoxLabs/oxid@5ba38b9bbc9326c294b353daaf2a074eca18c22f`,
  `crates/adapters/openid4vci/src/portal.rs` and
  `crates/adapters/openid4vci/src/lib.rs`, Apache-2.0.
- Lace ID Portal conformance-only source:
  `input-output-hk/lace-id-portal@804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`,
  `crates/issuer-http/src/routes_issuer.rs` and
  `crates/issuer-integration/tests/http_integration.rs`, Apache-2.0.
- No production source or fixture is copied or transformed.
