# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-28
Source retrieval date: 2026-09-28
Research blockers: none

## Problem and existing implementation

`identus-jose` contains generic compact/header/signature modules plus roughly
1,200 authored nonblank lines in `oid4vci.rs` and `oid4vci_verifier.rs`.
Those profile modules introduce the crate's only DID dependency and own errors
for proof claims, key resolution, X.509 providers, federation chains, key
attestation, proof policy, clocks, freshness, and replay. `identus-oid4vci`
already depends on `identus-jose`, and it is the cohesive protocol owner.

The workspace dependency graph is acyclic. Before this change the material
edges are:

```text
identus-jose    -> identus-core + identus-crypto + identus-did
identus-oid4vci -> identus-core + identus-crypto + identus-jose
```

The target moves the DID edge outward to the protocol crate.

The current implementation has consumer evidence in both `identus-oid4vci`
and `identus-oid4vp`: each needs generic compact/signature mechanics, while
only OID4VCI needs the proof profile, DID resolution, trust, and replay policy.

## Normative sources

The relevant normative sources are RFC 7515 JWS, RFC 7517 JWK, RFC 9864 JOSE
fully specified algorithms, and OpenID4VCI 1.0 Final Appendix F.1. ADR 0034,
ADR 0035, ADR 0061, ADR 0110, and the repository architecture rules constrain
the facade and dependency direction. Primary source URLs are listed below.

## Candidate decisions

The assessment used published manifests/docs and exact release source where
available. Standalone dependency-cone counts include the probe root and are
comparative, not the incremental workspace cone.

| Candidate | Assessed release/revision | Evidence | Decision |
|---|---|---|---|
| `jsonwebtoken` | 11.1.0 / `4c0ae752` | MSRV 1.88; RustCrypto backend; raw JWS API; 86-package standalone probe; algorithm model exposes legacy `EdDSA`, not RFC 9864 `Ed25519` | `oracle` / private operation candidate |
| `jose` | 0.0.2 / `edec7927` | MSRV 1.84; type-state/no_std design; 103-package probe; no RFC 9864 `Ed25519` identifier and no exact bounded-header contract | `oracle` / private operation candidate |
| RustCrypto `jose-jws` | 0.1.2 / `e0212678` | MSRV 1.65; no_std; forbids unsafe; young 0.1 API includes legacy `EdDSA` and unsecured-algorithm surface; no SDK-equivalent limits/duplicate contract | `oracle` / private operation candidate |
| `jose-rs` | 0.7.0 / `dcdf1506` | MSRV 1.88; pure Rust and unsafe-forbidden; broad JWS/JWE/JWK/JWT surface and default PKCS#11 feature; Ed25519 primitive is exposed as legacy `EdDSA` | `not-adopt` wholesale |
| `jwt-compact` | 0.8.0 | MSRV 1.65; type-safe/no_std; 60-package selected-feature probe; JWT-oriented and legacy `EdDSA` surface | `oracle` / private operation candidate |
| `josekit` | 0.10.3 | broad mature JOSE coverage; 47-package probe; OpenSSL/native coupling | `not-adopt` |
| `identity_jose` | 1.5.1 / `7dd52708` | 180-package standalone probe and IOTA identity model coupling | `not-adopt` |

None replaces the public facade without losing at least one required property:
RFC 9864 naming, strict canonical compact parsing, duplicate-member rejection,
pre-allocation bounds, static redacted errors, exact algorithm/key binding,
portable targets, or Identus-owned parsed/verified states.

## Compatibility and dependency evidence

The extraction changes unpublished Rust import paths but preserves public wire
bytes and every accepted/rejected proof invariant. Exact versions and features
for any future engine remain outside the production graph. The direct and
resolved dependency cone is improved immediately by moving DID out of JOSE;
standalone candidate probe counts above are comparative upper bounds. Primary,
MSRV, WASM, iOS, and Android target evidence remains required before merge.

## Security, privacy and maintenance evidence

The current facade forbids unsafe code and uses pure-Rust crypto operations.
Candidate unsafe and native-code evidence ranges from explicitly
unsafe-forbidden (`jose-jws`, `jose-rs`) to native OpenSSL reach (`josekit`).
License and provenance must be rechecked from exact registry archives and
pinned source revisions before adoption. Supply-chain evidence must include
advisories, licenses, source, features, lockfile cone, and target builds.

The public and wire compatibility rule is no semantic drift; the facade
boundary keeps external types private. Maintenance, release and security
posture must be current at adoption time. Protocol or draft currency is a hard
gate: RFC 9864 and OpenID4VCI Final semantics cannot be replaced by a legacy or
draft-only surface.

## Rejected or deferred candidates

The decision is `retain-local`: retain the public `identus-jose` facade and its existing private RustCrypto
operations. Do not implement broad JOSE features speculatively. A future issue
may adopt a candidate for one private operation when it:

1. implements the exact selected RFC/profile and algorithm spelling;
2. passes existing positive, negative, boundary, mutation, and target tests;
3. does not leak dependency types or defaults;
4. preserves bounds, duplicate rejection, redaction, and state transitions;
5. reduces maintained risky code after adapter and dependency-cone cost; and
6. records version, features, MSRV, unsafe/native reach, licenses, rollback,
   and reconsideration evidence.

Every candidate is therefore rejected as a wholesale public replacement and
deferred as an operation-level oracle/private engine. The reconsideration trigger
is exact facade parity plus a net reduction in maintained risky code.

Rollback removes a newly adopted private engine and restores the existing
operation behind the unchanged Identus facade.

## Open questions and blockers

There are no blockers for the ownership extraction. Whether a future candidate
can replace one compact/header/signature operation remains intentionally open
and requires fresh release, security, provenance, target, and cone evidence.

## Sources

- https://docs.rs/crate/jsonwebtoken/11.1.0
- https://docs.rs/crate/jose/0.0.2
- https://docs.rs/crate/jose-jws/0.1.2
- https://docs.rs/crate/jose-rs/0.7.0
- https://docs.rs/crate/jwt-compact/0.8.0
- https://docs.rs/crate/josekit/0.10.3
- ADR 0061 and the Rust library not-adopted ledger

## Evidence commands

Workspace metadata, source inspection, docs.rs manifests, locked candidate
source, and isolated `cargo metadata --format-version 1` probes were used.
Commands included `cargo metadata`, `cargo tree`, source `rg`, strict OpenSpec
validation, and the repository factory checks. Unrun checks at planning time
are the post-move workspace tests, Clippy/docs, dependency policy, MSRV,
primary, WASM, iOS, and Android gates; tasks.md requires them before delivery.
