# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/555
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001`, `SDK-ARCH-002`, `SDK-SEC-001`, `SDK-SEC-003`,
`SDK-DELIVERY-001`, and `SDK-LIM-007` remain unchanged. The repository blueprint,
ADR 0110, ADR 0169, ADR 0174, and issue #555 already authorize independently
selectable, portable, bounded DID-method crates without historical SDK domain
names or downstream effects.

## Introduced or changed constraints

The remaining methods are delivered as three independent crates. `did:key` and
`did:jwk` remain deterministic and effect-free. `did:web` owns only pure URL
transformation, document binding, and a minimal injected bounded-fetch port;
concrete networking and network-security policy remain adapters. No method
crate may become a generic DID dependency or re-export a third-party SSI type.

The first profiles and pinned revisions are explicit. A broader curve,
encoding, derivation, verification-method, redirect, transport, or trust policy
requires a later issue and profile decision rather than permissive acceptance.

## Introduced or changed limitations

This planning change does not implement or support any of the three methods.
The future first profiles deliberately exclude private keys, custody,
generation, implicit Ed25519-to-X25519 derivation, unsupported curves, JSON-LD
processing, concrete HTTP, DNS/TLS/redirect policy, persistent caches, global
registries, bindings, consumer adoption, publication, and certification.

## Consumer and product impact

None now. The child crates will be additive and optional. SDK-TS and other
language SDKs are discovery and differential evidence, not normative DTO or
error definitions. Migration adapters remain downstream or binding-layer work.
No consumer repository is modified.

## Activation and rollback

This planning decision activates after the report, ADR, OpenSpec specification,
backlog owner, and child hierarchy validate and protected CI passes. Before
merge it can be removed. After merge it can be superseded by a new ADR; no
runtime rollback is needed because no implementation or dependency is added.
Each child remains independently reversible and cannot claim support before its
own conformance and target evidence passes.

## Evidence

Issue #555 and child issues #557–#559 provide durable ownership. ADR 0187 and
the pinned research report record the architecture and candidate disposition.
The existing constraint index, blueprint, ADR 0110, ADR 0169, ADR 0174, and
canonical `IDR-040` row provide the effective authority.
