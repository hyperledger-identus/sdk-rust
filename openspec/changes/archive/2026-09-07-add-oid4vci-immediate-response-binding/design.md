# Design: request-bound immediate Credential HTTP response validation

## Context

OpenID4VCI 1.0 Final section 8.3 requires an immediate Credential Response to
use HTTP status 200. An unencrypted response uses `application/json`. Its
`credentials` array contains no more credentials than the number of holder
keys supplied through request proofs, although an issuer may return fewer.

The SDK already constructs a `JwtCredentialRequest` with a positive bounded
proof count and parses the immediate body into
`ImmediateCredentialResponseCore`. The remaining reusable seam is validation
of caller-supplied HTTP metadata followed by the necessary count upper bound.
This does not establish actual transport provenance or prove that proof JWTs
identify distinct keys.

Read-only Oxid at `5ba38b9bbc9326c294b353daaf2a074eca18c22f`
expects exactly one `credentials[].credential` string and rejects
`transaction_id`. Lace ID Portal at
`804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` emits an incompatible pre-Final
singular response and remains evidence not to copy that shape. No consumer
source or fixture is copied.

## Goals / Non-Goals

**Goals:**

- validate the caller-supplied immediate status and effective media type;
- parse only after envelope validation under independent positive limits;
- enforce the necessary `credential_count <= proof_count` condition;
- return an honest owned state without retaining headers or request secrets;
- reuse the reviewed private HTTP grammar and preserve its regression suite;
- preserve Rust 1.85 and native/mobile/browser targets.

**Non-Goals:**

- HTTP execution, TLS, DNS, redirect, proxy, origin or endpoint provenance;
- Cache-Control requirements inferred from non-normative examples;
- Credential Error Responses, deferred status 202/polling or encryption;
- Authorization Details, credential identifiers, DPoP or generic headers;
- proof uniqueness, proof-key/credential correlation or credential format
  verification;
- token/proof/issuer/schema/status trust, storage, notification execution,
  retry/replay or product policy;
- chain extensions, FFI, consumer adoption, publication, release or `main`.

## Decisions

### Bind the envelope through the existing request

`JwtCredentialRequest::validate_immediate_response` borrows the request and
accepts status, effective Content-Type, body and
`ImmediateCredentialHttpResponseLimits`. It returns
`RequestBoundImmediateCredentialResponse`, which owns the parsed body and the
non-secret request proof count. Borrowing does not claim execution, single-use
or retry safety.

Status 200 is the only accepted immediate success status. Status 202 returns
the existing explicit unsupported-deferred error without parsing remote body
content. Every other value returns a new invalid-status error. This preserves
the distinction between a supported immediate outcome, a valid-but-unsupported
deferred branch, and malformed or non-success HTTP outcomes.

### Reuse strict media grammar without promoting examples

Move the existing private `application/json` and Cache-Control grammar helpers
from the Nonce response module into a private `http_field` module. The Nonce
contract continues using both helpers; the Credential response uses only the
media helper. The parser accepts case-insensitive type/subtype, RFC-shaped
parameters and optional whitespace, and rejects combined, malformed or
injection-shaped field values.

Final section 8.3 requires `application/json` for an unencrypted response but
does not normatively require Cache-Control. Several examples show `no-store`,
and one immediate example omits it. This slice therefore does not accept a
Cache-Control input or promote example text into a conformance requirement.

### Enforce only the cardinality fact this request can prove

After bounded body parsing, the method compares the non-empty response
credential count with the request's proof count. Equal or fewer credentials
pass; more fail with a static count-exceeds-proofs error. This is a necessary
Final condition, not sufficient proof of key cardinality: `Oid4vciProofJwt`
is intentionally opaque here, and the method does not claim proofs use unique
keys or returned credentials are bound to corresponding keys.

The returned state exposes the request proof count, borrows the parsed response
and can consume itself back into that response. It does not implement Clone,
Display or Serde. Debug delegates only to non-secret counts and presence.

### Validate cheap public metadata before sensitive body work

Validation order is status, Content-Type byte bound, Content-Type grammar,
bounded body parsing, then request-cardinality comparison. Invalid metadata
cannot cause a credential body to be parsed. No remote field or body is
retained in errors.

## Risks / Trade-offs

- **Proof count can exceed distinct key count** -> the type and docs call the
  check a necessary upper bound only; unique-key and credential binding remain
  format/proof verification work.
- **No Cache-Control enforcement** -> this follows normative Final text rather
  than examples; product adapters may impose a stricter cache policy.
- **Caller supplies status and media** -> the contract explicitly does not
  establish that an HTTP exchange occurred or came from the request endpoint.
- **Status 202 body is not inspected** -> the explicit deferred error avoids
  parsing unsupported content and leaves deferred semantics to its own slice.

## Migration Plan

Land as an additive unpublished API. Existing callers remain source compatible.
A later downstream issue can replace Oxid's manual envelope parsing after an
immutable SDK candidate. Rollback is one focused revert before publication.

## Open Questions

None. Error, deferred, encryption, execution and format layers remain explicit
issue-first work.

## Provenance

- OpenID4VCI 1.0 Final section 8.3, HTML SHA-256
  `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.
- RFC 9110 HTML SHA-256
  `d431760660ea44e130f6e919dab216df2d0b3a490567a98089267523368fe1e5`.
- Oxid conformance-only evidence:
  `MediaNoxLabs/oxid@5ba38b9bbc9326c294b353daaf2a074eca18c22f`,
  `crates/adapters/openid4vci/src/lib.rs` and
  `crates/adapters/openid4vci/src/laceid_portal_contract_tests.rs`, Apache-2.0.
- Lace ID Portal incompatible legacy evidence:
  `input-output-hk/lace-id-portal@804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`,
  `crates/issuer-services/src/credential.rs`; no code or fixture is used and no
  license grant is relied upon.
- No production source or fixture is copied or transformed.
