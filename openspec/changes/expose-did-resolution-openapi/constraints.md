# Constraint readiness

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/205
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001` and `SDK-ARCH-002` keep the feature in the outer Axum adapter;
`identus-did` gains no HTTP/OpenAPI dependency. `SDK-COMPAT-001` through
`SDK-COMPAT-005` require stable Rust 1.98.1 and no new target promise.
`SDK-SEC-001` preserves the unsafe prohibition. `SDK-DELIVERY-001` is satisfied
by issue #205, this contract, review and hosted gates. `SDK-LIM-001`,
`SDK-LIM-003`, `SDK-LIM-005`, `SDK-LIM-006` and `SDK-LIM-009` preserve the
unpublished, host-only, consumer-owned and temporary fast/slow status.

## Introduced or changed constraints

- The additive `openapi` feature is disabled by default and exists only on
  `identus-did-resolver-http`.
- Exact `utoipa 5.5.0` is used without defaults and with only the `macros`
  compile feature required by the published crate; macros are not invoked and
  its type may cross only this explicit outer-adapter feature surface.
- The document describes only fixed `GET /{did}` behavior implemented by the
  same crate and is deterministic under serialization.
- DID Core remains independent of Utoipa and OpenAPI annotations.

No workspace compiler, release, publication, chain or product boundary changes.

## Introduced or changed limitations

- The OpenAPI artifact is available only when explicitly compiled and remains
  an experimental pre-release API.
- Method-specific string query extensions are described but not enumerated.
- A host that nests the router under a prefix owns document path composition.
- The artifact provides no UI, server, portable-target or certification claim.

## Consumer and product impact

Axum hosts can merge one typed OpenAPI document instead of copying the old
NeoPRISM description. Existing consumers and wire behavior are unchanged.
NeoPRISM remains read-only and downstream adoption is not claimed.

## Activation and rollback

The feature activates only after issue #205's PR passes local and hosted gates
and merges into `develop`. Revert removes the optional feature and function;
default consumers require no migration.

## Evidence

Issue #205 and parent #10 provide exact roadmap authority. Research records the
normative versions, dependency/public coupling, MSRV, feature cone, unsafe and
native scan, compatibility, limitations, stop conditions and rollback.
