# DID candidate train

M5 prepares two chain-neutral packages as an independent `0.1.0-rc.1`
candidate:

```text
identus-did-resolver-http =0.1.0-rc.1
    ├── identus-did =0.1.0-rc.1
    └── identus-core =0.1.0-rc.1

identus-did =0.1.0-rc.1
    ├── identus-core =0.1.0-rc.1
    └── identus-derive =0.1.0-rc.1 (build-time)
```

![DID candidate layering](diagrams/did-candidate.svg)

## Responsibilities and boundary

| Package | Owns | Does not own |
| --- | --- | --- |
| `identus-did` | Validated DID and DID URL values, DID documents, verification methods, services, resolver-facing models, and generic resolver ports. | DID methods, ledger access, networking, persistence, key custody, trust, consent, or wallet policy. |
| `identus-did-resolver-http` | Axum request/response mapping, DID resolution content negotiation, and optional `openapi` schema types. | A resolver implementation, DID method, deployment topology, authentication, rate limiting, or product policy. |

The HTTP package is an optional host adapter, not a required facade. Consumers
that need only domain values or implement their own transport select
`identus-did` directly. DID method and chain adapters depend on the generic
ports from outside this candidate boundary.

## Candidate state

The canonical workspace manifests are still version `0.0.0` with
`publish = false`. The candidate tool stages release-shaped archives in an
external scratch workspace without changing those manifests. Its reserved
future tag is `identus-did-v0.1.0-rc.1`; no tag, GitHub release, publication
workflow, or crates.io artifact is claimed here.

The exact protected source revision for this review is:

```text
cda086f3e7fe72d251c1f896bccdcf5dd1bc8c16
```

Use the [source-adoption instructions](adoption.md) to evaluate it. A library
that will itself be published to crates.io cannot retain a Git dependency and
must wait for registry-published packages.

## Reproducible evidence

Two clean preparations reproduced the same candidate archives and evidence:

| Evidence | `identus-did` SHA-256 | `identus-did-resolver-http` SHA-256 |
| --- | --- | --- |
| Cargo archive | `ec9ed5be2f8d716d1395cdea347ec71f19def355617579a7ad26cfc14681f7a7` | `875801cbcd2bb5b1e378ca4f84903f1f58949da7447cea7e586f172845eb8434` |
| Public API origin | `3412a7dce4b5699afda75ad5c9a0bb4f6f5fbb5cb3256e187a2d107fb26763dd` | `9c432f0d0c3d5db92da3aa88aed05164aa2adf483c11be84b9239c9973179df6` |
| Normalized CycloneDX 1.5 | `371096ba00a08803548b67afcff8df15c10a228fbcd0bfbcf6297e742e475175` | `9f1734f3e400850c58dddc13ed36ea52d71050be899788ec988926b0ca1b5349` |

The committed API snapshots establish the origin for later SemVer comparison.
Because this is the first candidate, compatibility is explicitly
`not-applicable-first-candidate`; no self-comparison was used to manufacture a
green result. The SBOMs describe dependencies and licenses but do not replace
Cargo-deny, advisory review, provenance, or security assessment.

Primary evidence:

- [DID candidate descriptor](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/release/did-candidate.toml)
- [Release-train registry](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/release/release-trains.toml)
- [ADR 0153: independent train tags](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/adr/0153-use-primary-package-tags-for-independent-release-trains.md)
- [ADR 0154: first-candidate API origin](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/adr/0154-use-first-candidate-api-snapshots-as-semver-origin.md)
- [ADR 0155: staged compiler and target matrix](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/adr/0155-qualify-staged-did-candidate-matrix.md)
- [Supply-chain qualification issue #384](https://github.com/hyperledger-identus/sdk-rust/issues/384)

## Compiler and target contract

| Package | Linux + macOS | Browser WASM | Android ARM64 | iOS ARM64 |
| --- | --- | --- | --- | --- |
| `identus-did` | Primary tests + MSRV compile checks | Compile-checked | Compile-checked | Compile-checked |
| `identus-did-resolver-http` | Primary tests + MSRV compile checks | Not supported | Not supported | Not supported |

The matrix always exercises the staged `0.1.0-rc.1` sources and their exact
internal dependency versions. Portable rows use `cargo check` with Rust 1.98.1
and 1.89.0; they make no browser, device, simulator, binding, packaging,
performance, or certification claim. The HTTP adapter is host-only by design.
Weekly/manual slow CI produces four attempt-scoped lane receipts and accepts
their aggregate only when source revision, compiler identities, target rows,
outcomes, and staged lockfile agree.

## Promotion gates

Candidate assembly and documentation do not authorize publication. The matrix
implementation is tracked in [#387](https://github.com/hyperledger-identus/sdk-rust/issues/387),
but only a natural or explicitly approved manual slow run at the final revision
constitutes release evidence. M5 then requires an independent exact-SHA
decision in [#388](https://github.com/hyperledger-identus/sdk-rust/issues/388).
Administrator-owned trusted-publishing hardening remains tracked by
[#344](https://github.com/hyperledger-identus/sdk-rust/issues/344).

See [release readiness](release-readiness.md) for the consolidated gate table.
