# SIOPv2 draft-13 protocol core

Issue [#514](https://github.com/hyperledger-identus/sdk-rust/issues/514)
introduces an experimental, unpublished `identus-siopv2` crate. The source of
truth is Self-Issued OpenID Provider v2 draft 13 (2023-11-28), not SDK-TS,
Oxid, or a third-party Rust DTO. [ADR 0182](../adr/0182-pin-siopv2-draft13-behind-identus-protocol-boundaries.md)
records the profile and dependency decision.

## Boundary

```text
caller transport / request retrieval / RP trust
                   |
                   v
       UnverifiedSiopRequest
                   |
     caller-authorized JOSE verification key
                   |
                   v
       ValidatedSiopRequest
                   |
        explicit approval + DID subject
                   |
                   v
        SiopResponsePlan -- injected JwsSigner
                   |                  |
                   +--------+---------+
                            v
                 DirectPostResponse
                            |
                 caller transport/delivery
```

The SDK owns bounded wire validation and deterministic state transitions. The adopter
owns HTTP, DNS/TLS and redirect execution, DID resolution, relying-party trust,
durable replay defense, key custody, user authentication, consent UI,
persistence, telemetry, and delivery recovery.

## Accepted profile

- signed `oauth-authz-req+jwt` request object;
- exact audience `https://self-issued.me/v2`;
- a closed duplicate-free request-claim set with scalar audience; broader JWT
  claim shapes require a separately versioned profile;
- exact `response_type=id_token`, `scope=openid`,
  `id_token_type=subject_signed_id_token`, and `response_mode=direct_post`;
- HTTPS client and redirect identifiers without user-info or fragments;
- DID `iss == sub`, DID URL `kid`, exact audience and nonce in the response;
- caller-selected Ed25519 or ES256 capability already implemented by
  `identus-jose`;
- explicit time, event identity, approval, refusal, cancellation, timeout,
  accepted delivery, and unknown delivery outcome.

Approval takes a distinct caller-supplied evaluation time and rechecks request
expiry immediately before response planning or signer invocation. Token
`iat`/`exp` values cannot be backdated to bypass that transition guard.

Unknown delivery outcome is terminal. The core never automatically resends an
identity assertion whose external delivery status is uncertain.

The response-signing algorithm is selected from the caller-authorized subject
DID method and enforced by the injected signer. It is intentionally independent
from the algorithm used by the relying party to sign its request object.

## Adapter outline

```rust,ignore
let retrieved = RequestUriReference::new(
    outer_client_id,
    request_uri,
    RequestUriMethod::Get,
    RequestRetrievalLimits::default(),
)?.prepare(RequestRetrievalInput::get(), RequestRetrievalLimits::default())?
  .bind_response(status, content_type, body)?;

let request = UnverifiedSiopRequest::from_retrieved(
    &retrieved,
    Siopv2Limits::default(),
)?.verify(&allowed_suites, &caller_authorized_rp_key, now)?;

let mut flow = SiopFlow::new(request);
flow.approve(
    approval_event,
    subject_did,
    subject_key_id,
    algorithm,
    now,
    now,
    token_expiry,
    &custody_signer,
)?;
```

The names `caller_authorized_rp_key` and `custody_signer` are intentional: the
SDK verifies and invokes supplied capabilities but does not discover, trust,
unlock or persist either key.

## Evidence route

The primary E2E test signs an RP request, independently verifies it, approves a
DID subject, signs the ID Token through `identus-jose`, decodes the form body,
and independently verifies every response claim. Negative evidence rejects
duplicate JSON members, wrong profile values, outer client or POST nonce
mismatch, unsafe destinations, invalid time windows, invalid DID method
relationships and illegal lifecycle transitions without discarding prior state.

- E2E: `cargo test -p identus-siopv2 --test e2e`
- property: `cargo test -p identus-siopv2 --test property`
- conformance: `cargo test -p identus-siopv2 --test conformance`
- vector: [`docs/conformance/fixtures/siopv2/v1/vectors.json`](../conformance/fixtures/siopv2/v1/vectors.json)
- quality contract: [`quality-evidence-plan.md`](quality-evidence-plan.md#siopv2did-authenticationqualityv1)

Oxid revision `3ceea617557dce795a247fdd9c6180a873f1611e` is
conformance and migration evidence only. Its `response_uri`, loopback fixture,
storage, activity model, Dioxus UI and custody behavior do not enter this SDK
crate. A later Oxid adapter can translate those product concerns explicitly.
