# Candidate crates

Each candidate crate has one reason to exist and a visible boundary. The
separation is designed for low coupling, high cohesion, minimal feature cones,
and independent future evolution.

## Crypto foundation train

![Dependency direction](../diagrams/release-train.svg)

| Crate | Kind | Runtime dependency direction |
| --- | --- | --- |
| [`identus-derive`](identus-derive.md) | Procedural macro | Build-time only |
| [`identus-core`](identus-core.md) | Foundation values and ports | Depends on `identus-derive` and Serde |
| [`identus-crypto`](identus-crypto.md) | Cryptographic primitives | Depends on `identus-core` and `identus-derive`; algorithms are feature-gated |

The release train is not a layered facade where consumers must always import
all three. A consumer normally selects the highest-level crate it needs;
Cargo resolves its exact internal closure.

## DID candidate train

![DID candidate layering](../diagrams/did-candidate.svg)

| Crate | Kind | Runtime dependency direction |
| --- | --- | --- |
| [`identus-did`](https://github.com/hyperledger-identus/sdk-rust/tree/develop/crates/did) | Generic DID domain and ports | Depends on released `identus-core`; uses `identus-derive` at build time |
| [`identus-did-resolver-http`](https://github.com/hyperledger-identus/sdk-rust/tree/develop/crates/did-resolver-http) | Optional Axum HTTP adapter | Depends on `identus-did`, released `identus-core`, and host HTTP libraries |

These two packages are candidate-only and source-evaluated. They are not
available from crates.io merely because a staged `0.1.0-rc.1` archive and
public-API origin exist. See the [DID candidate review](../did-candidate.md).

## Portable DID methods

| Crate | Kind | Runtime dependency direction |
| --- | --- | --- |
| [`identus-did-key`](identus-did-key.md) | Strict effect-free `did:key` profile | Depends on generic DID/crypto foundations; opt-in registry composition |
| [`identus-did-jwk`](identus-did-jwk.md) | Bounded effect-free `did:jwk` profile | Depends on generic DID/crypto/JSON foundations; opt-in registry composition |

These packages are unpublished experimental source components. Their frozen
profiles, bounds, redacted errors, conformance packets, fuzzing, benchmarks,
and registry E2E routes are reviewed independently from the generic DID
candidate train.

## Portable PRISM stack

![Portable PRISM stack](../diagrams/prism-portable-stack.svg)

| Crate | Kind | Runtime dependency direction |
| --- | --- | --- |
| [`identus-prism-did`](identus-prism-did.md) | PRISM method semantics | Depends on generic DID/crypto foundations |
| [`identus-prism-vdr`](identus-prism-vdr.md) | PRISM VDR semantics | Depends on `identus-prism-did` |
| [`identus-prism-indexer`](identus-prism-indexer.md) | Ordered atomic indexing core | Depends on `identus-prism-vdr` |

These are unpublished experimental source packages, not part of either
activated release train. See the [portable-stack walkthrough](../architecture/prism-portable-stack.md)
for effect boundaries, executable evidence, and downstream adoption status.

## Selective-disclosure credential semantics

![SD-JWT VC stages](../diagrams/sd-jwt-vc-draft22.svg)

| Crate | Kind | Runtime dependency direction |
| --- | --- | --- |
| [`identus-sd-jwt`](https://github.com/hyperledger-identus/sdk-rust/tree/develop/crates/sd-jwt) | RFC 9901 mechanics | Depends on JOSE and generic credential/presentation vocabulary |
| [`identus-sd-jwt-vc`](identus-sd-jwt-vc.md) | Canonical draft-22 VC profile | Depends on the RFC engine and generic credential vocabulary |

Both crates are unpublished experimental source components. Protocol crates
are dev/test dependencies of the profile, not production foundations.
