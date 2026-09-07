# Design: bounded immediate Final Credential Response parsing

## Context

OpenID4VCI 1.0 Final section 8.3 defines two successful response branches. An
immediate HTTP 200 response contains a non-empty `credentials` array and may
contain `notification_id`. A deferred HTTP 202 response instead contains
`transaction_id` and a positive `interval`. The branches are mutually
exclusive. Every immediate array element is an object with a required
`credential` whose value is a string or object; format specifications define
its interpretation.

The current SDK stops after bounded request construction. Read-only Oxid at
`5ba38b9bbc9326c294b353daaf2a074eca18c22f` already consumes the Final
`credentials[].credential` string shape and declines `transaction_id`. Lace ID
Portal at `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` emits an incompatible
pre-Final singular response and is evidence not to preserve that wire shape.
No consumer source or fixture is copied.

## Goals / Non-Goals

**Goals:**

- parse only the unencrypted successful immediate JSON body;
- preserve credential order and exact JSON spelling without choosing a format;
- give string credentials a decoded sensitive accessor while retaining the
  exact JSON value for format-neutral forwarding;
- allow extension members under explicit structural and allocation limits;
- reject duplicate names and deferred-branch fields with static diagnostics;
- preserve Rust 1.85 and native/mobile/browser targets.

**Non-Goals:**

- HTTP status, media, cache, TLS, DNS, redirect, proxy, DPoP or provenance;
- Credential Error Response or Deferred Credential Endpoint processing;
- response encryption, request correlation, retries or notification execution;
- base64url/CBOR/JWT/SD-JWT/mdoc/VCDM decoding or cryptographic verification;
- issuer/schema/status trust, consent, storage, disclosure or product policy;
- chain extensions, FFI, consumer adoption, publication, release or `main`.

## Decisions

### Name the supported state honestly

`ImmediateCredentialResponseCore::parse` accepts a JSON body and
`ImmediateCredentialResponseLimits`. A response containing `transaction_id`
fails with a dedicated unsupported-deferred error rather than being called
invalid. A top-level `interval` fails as an invalid immediate response because
Final permits it only alongside the deferred branch. HTTP status and media type
remain adapter-owned, so parsing alone does not prove an HTTP 200 response.

### Retain opaque credential values without format coupling

Every `credentials` element must be an object containing exactly one member
named `credential`; other unique members are bounded extensions. The credential
value must begin as a JSON string or object. The existing scanner validates its
complete subtree under the response depth/node budget while recording the
exact source range. `IssuedCredential` owns that exact range as a
`Zeroizing<String>` and reports `CredentialValueKind::{String,Object}`.

For a string value, the scanner also returns its decoded semantic string in a
separate zeroizing allocation. This lets a wallet pass a JWT, SD-JWT or binary
base64url value to a later format adapter without parsing JSON again. Objects
remain exact JSON because their structure belongs to the selected format. Both
representations use explicitly sensitive accessors and redacted `Debug`.

### Bound parser work and retained allocations independently

`ImmediateCredentialResponseLimits` requires non-zero maxima for complete JSON
bytes, depth, nodes, top-level member count, credential count,
credential-entry member count, one exact credential JSON value, aggregate exact
credential JSON bytes, and decoded notification-ID bytes. Depth is capped by
the existing global configurable maximum. The scanner checks member/count
limits before growing vectors and uses checked addition for the aggregate.

The complete response byte limit bounds input and name/string decoding. The
node/depth limits bound discarded extensions. Per-value and aggregate limits
bound retained exact credential allocations. The decoded string credential is
necessarily no larger than its already-bounded exact JSON representation.

### Preserve Final extension behavior while rejecting smuggling

Unknown top-level and per-entry members are structurally parsed and discarded,
not rejected. Duplicate names at any object depth use the crate-wide
`DuplicateJsonProperty` error. `credentials` must be present and non-empty;
`notification_id`, when present, must be a non-empty bounded string. Any
`transaction_id` selects the unsupported deferred branch regardless of member
order, preventing ambiguous classification.

## Risks / Trade-offs

- **Exact raw JSON retains whitespace and escapes** -> this is deliberate for
  lossless format handoff; decoded string access is provided separately.
- **The caller can parse without HTTP 200 evidence** -> the type and docs call
  this body-core syntax only; a later HTTP response wrapper can bind status and
  media semantics.
- **Extension values may contain sensitive data** -> discarded extensions are
  not retained, while the caller remains responsible for erasing its input.
- **Deferred responses are valid Final responses** -> a dedicated unsupported
  error preserves that distinction until interval/polling semantics receive a
  separate contract.

## Migration Plan

Land as an additive unpublished API. Existing crates and callers remain source
compatible. A later downstream issue can replace Oxid's manual parser after an
immutable SDK candidate. Rollback is one focused revert before publication.

## Open Questions

None. HTTP, error, deferred, encryption and format layers remain explicit
issue-first work.

## Provenance

- OpenID4VCI 1.0 Final section 8.3, HTML SHA-256
  `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.
- Oxid conformance-only evidence:
  `MediaNoxLabs/oxid@5ba38b9bbc9326c294b353daaf2a074eca18c22f`,
  `crates/adapters/openid4vci/src/lib.rs` and
  `crates/adapters/openid4vci/src/laceid_portal_contract_tests.rs`, Apache-2.0.
- Lace ID Portal incompatible legacy evidence:
  `input-output-hk/lace-id-portal@804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`,
  `crates/issuer-services/src/credential.rs`; no code or fixture is used and no
  license grant is relied upon.
- No production source or fixture is copied or transformed.
