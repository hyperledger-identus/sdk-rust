# Start here

SDK-Rust is the generic Rust foundation shared by Identus ecosystem products.
It is not a wallet application, blockchain SDK, cloud service, or umbrella that
automatically owns every SSI protocol.

## Ownership rule

Reusable, chain-neutral domain primitives, cryptographic utilities, protocol
engines, ports, validation, and conformance evidence belong in SDK-Rust.
Method, ledger, runtime, custody, consent, trust, persistence, user experience,
deployment, and certification policy remain with their owning downstream.

That distinction allows Midnight Identity, NeoPRISM/Cardano, Lace ID Portal,
Oxid, and future products to share foundations without forcing one chain or
product model into the others.

## Maturity vocabulary

| Term | Meaning here |
| --- | --- |
| Implemented | Code and tests exist; the public surface may still change. |
| Candidate | A reproducible artifact is available for review; publication status is proven only by its registry receipt. |
| Experimental `0.x` | SemVer applies with an explicit migration policy and support window. |
| Supported | The documented compiler, targets, features, and support period are an effective promise. |
| Certified | An external assessment or certification exists; tests alone never imply this. |

The first three packages are release-activated `0.1.0-rc.1` candidates.
Every other workspace package remains version `0.0.0` with `publish = false`;
candidate archives are prepared in an isolated temporary workspace and may be
uploaded only by the protected release train.

## Branches

- `develop` is the protected integration and documentation source.
- `main` is intentionally reserved and is not populated by site publication.
- Releases require a separately approved exact revision and protected process.

For the repository-level contract, see
[Governance](https://github.com/hyperledger-identus/sdk-rust/blob/develop/GOVERNANCE.md),
[constraints](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/governance/constraints-and-limitations.md),
and the [release policy](https://github.com/hyperledger-identus/sdk-rust/blob/develop/RELEASING.md).
