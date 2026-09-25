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
