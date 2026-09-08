# Constraint readiness

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/201
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001` and `SDK-ARCH-002` apply: the chain-neutral transport adapter
is a new outer-boundary crate and depends inward on `identus-did`; DID Core
does not depend on Axum. `SDK-SEC-003` applies to attacker-controlled header,
path and query inputs and is made concrete by pre-parse byte/range ceilings.
`SDK-DELIVERY-001` is satisfied by child issue #201 under #10 and this
specification-first change. `SDK-LIM-003`, `SDK-LIM-005`, `SDK-LIM-006`,
`SDK-LIM-007` and `SDK-LIM-009` keep W3C draft currency, host-only validation,
unpublished status, unresolved downstream adoption and release/certification
claims explicit.

## Introduced or changed constraints

- All repeated `Accept` values together are at most 8 KiB and contain at most
  32 media ranges before negotiation.
- The adapter supports exactly `application/did-resolution`,
  `application/did` and `application/json`; missing `Accept` defaults to
  `application/did`.
- Quality values follow the RFC 9110 three-decimal form. Malformed or duplicate
  `q` parameters fail with W3C `invalidOptions`/400.
- Non-empty query strings fail with `invalidOptions`/400 until a separately
  specified bounded decoder exists.
- Document-only success requires matching resolver metadata `contentType`;
  failure, mismatch, invalid state and serialization failure do not emit a
  successful document response.
- The public route is fixed and state-closed. A consumer chooses its external
  prefix by nesting the returned router.

No repository-wide Rust, target, publication or release constraint changes.

## Introduced or changed limitations

- The component provides only DID Resolution `GET`, not dereferencing,
  registration, discovery, OpenAPI or client behavior.
- Resolution query parameters are rejected rather than implemented.
- The crate is tied deliberately to Axum as an optional host adapter; the core
  resolver port stays framework-neutral.
- Host deployment must add its own timeout, concurrency, request-body, TLS,
  authentication, authorization, abuse prevention, telemetry and shutdown
  policy.
- Portable WASM/iOS/Android support is not claimed for this server adapter.
- The W3C baseline is a Candidate Recommendation Draft and requires currency
  review before publication or certification.

## Consumer and product impact

Consumers gain a reusable router that can be nested without implementing
negotiation and status semantics. The injected resolver and all DID values
remain unchanged. This is an experimental source-level API in an unpublished
crate; neither Oxid nor midnight-identity adoption is claimed by repository
tests.

## Activation and rollback

The decision activates only when issue #201's PR passes local and hosted gates
and merges into `develop`. Reverting that focused PR removes the new crate,
dependencies, inventory/rulebook entry and capability specification. No stored
data or wire migration is required. Parent #10 remains open for the excluded
surface.

## Evidence

Research records the named consumer, current donor and normative revisions,
candidate matrix, features/cone, maintenance and license posture, unsafe
surface, byte/range bounds, host-only target plan, compatibility facade,
rollback and explicit stop conditions.
