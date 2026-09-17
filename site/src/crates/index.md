# Crates in the candidate

Each candidate crate has one reason to exist and a visible boundary. The
separation is designed for low coupling, high cohesion, minimal feature cones,
and independent future evolution.

![Dependency direction](../diagrams/release-train.svg)

| Crate | Kind | Runtime dependency direction |
| --- | --- | --- |
| [`identus-derive`](identus-derive.md) | Procedural macro | Build-time only |
| [`identus-core`](identus-core.md) | Foundation values and ports | Depends on `identus-derive` and Serde |
| [`identus-crypto`](identus-crypto.md) | Cryptographic primitives | Depends on `identus-core` and `identus-derive`; algorithms are feature-gated |

The release train is not a layered facade where consumers must always import
all three. A consumer normally selects the highest-level crate it needs;
Cargo resolves its exact internal closure.
