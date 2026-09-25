# Hyperledger Identus SDK for Rust

Chain-neutral Rust foundations for decentralized identity, verifiable
credentials, and wallet products.

> **Pre-release software:** the active `develop` line and `0.1.0-rc.1` trains
> are experimental. A registry receipt—not this handbook—proves whether a crate
> is published. No crate is production-supported, certified, or a final SemVer
> commitment.

This handbook answers three practical questions:

1. What reusable capabilities belong in SDK-Rust?
2. Why are the crypto foundation and DID capabilities separate release trains?
3. What evidence and human decisions remain before each candidate can advance?

![Three-crate release train](diagrams/release-train.svg)

## The short version

| Crate | One responsibility | Deliberately outside |
| --- | --- | --- |
| `identus-derive` | Generate recurring validated-newtype and port-marker scaffolding at compile time. | Runtime SSI, protocol, chain, storage, and product policy. |
| `identus-core` | Provide chain-neutral errors, time, URL, component metadata, and foundational capability types. | Cryptographic algorithms, DID methods, transports, custody, and wallet policy. |
| `identus-crypto` | Provide reusable bytes-in/bytes-out cryptographic primitives and explicit entropy injection. | Key custody, hardware/KMS policy, JOSE/protocol policy, ledgers, and wallet decisions. |

The dependency direction is intentional: crypto builds on core; core uses the
derive tooling; neither foundation may depend on a blockchain, wallet product,
or downstream repository.

The next independent train is candidate-only. `identus-did` supplies generic
DID values, document models, and resolver ports. `identus-did-resolver-http`
is the optional Axum host adapter. It adds no DID method, ledger, chain, wallet,
or product policy.

![DID candidate layering](diagrams/did-candidate.svg)

## Navigate

- [Start here](start-here.md) for project maturity and terminology.
- [The first crypto release train](release-train.md) for the initial product
  decision and protected publication mechanics.
- [DID candidate train](did-candidate.md) for the source-only review surface.
- [Crates](crates/index.md) for focused responsibilities and examples.
- [Architecture](architecture/index.md) for dependency and ownership diagrams.
- [Release readiness](release-readiness.md) for the live approval checklist.
- [Adoption](adoption.md) for exact-revision source evaluation before release.
