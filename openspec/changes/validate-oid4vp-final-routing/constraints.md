# Constraint readiness

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/447
Constraint blockers: none

## Existing entries affected

- `SDK-ARCH-001`, `SDK-ARCH-002`: authorization routing remains chain- and
  product-neutral and adds no dependency edge.
- `SDK-COMPAT-001`, `002`, `004`, `005`: Rust and portable target policy is
  unchanged.
- `SDK-SEC-001`, `002`, `003`: unsafe remains prohibited; new nonce and route
  inputs are bounded and diagnostics are redacted.
- `SDK-REL-001`: `identus-oid4vp` remains unpublished and outside the protected
  three-crate release train.
- `SDK-LIM-005`, `SDK-LIM-007`: verifier trust, consent, product policy,
  transport execution, and outer allocation remain caller-owned.

## Introduced or changed constraints

The first authorization-routing state accepts only exact `vp_token` plus
`direct_post`, a non-empty bounded Final nonce, an absolute bounded HTTPS
`response_uri`, no `redirect_uri`, and one existing valid DCQL query. It must
preserve JAR evidence and validate request semantics through one parsed map.

## Introduced or changed limitations

The state does not authorize a verifier key, prove endpoint safety or
reachability, execute HTTP, record consent, verify credentials, construct a
presentation/response, or support redirect modes, `direct_post.jwt`, SIOPv2,
DC API, HAIP, JWE, transaction data, scope expansion, or format metadata.

## Consumer and product impact

Headless consumers gain a reusable prerequisite for safe presentation-response
work without adopting a network runtime or wallet policy. No Oxid, Midnight,
Lace, NeoPRISM, mobile UI, transport, storage, or release surface changes.

## Activation and rollback

Activation requires an issue-linked reviewed merge to `develop`. Rollback
removes the additive unpublished type, transition, errors, ADR, and spec while
leaving existing JAR and DCQL-only states intact. No registry, data, consumer,
or migration action is required.

## Evidence

Issue #447 is exact material authority under IDR-024 and ADR 0004. ADR 0168
records the standards/profile decision. Clean-room protocol and misuse vectors,
resource/redaction tests, public API/error contracts, portable compiles, and an
exact-diff architecture/security review are required before delivery.
