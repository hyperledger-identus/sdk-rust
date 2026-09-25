# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-25
Source retrieval date: 2026-09-25
Research blockers: none

## Problem and existing implementation

ADR 0134, `docs/release/crypto-candidate.toml`,
`scripts/check-release-train.py`, `scripts/prepare-crypto-candidate.py` and
`scripts/publish-release-train.py` intentionally bind exactly the published
derive/core/crypto train, version `0.1.0-rc.1`, and tag `v0.1.0-rc.1`. The
candidate builder already proves useful safety primitives: external VCS-free
scratch, two assemblies, safe archive inspection, exact internal dependency
requirements, local-patch closure verification, atomic evidence publication
and no remote mutation.

`identus-did` and `identus-did-resolver-http` remain canonical `0.0.0`,
`publish = false` source packages. Their manifests inherit repository/license/
MSRV policy but lack release metadata. The latter depends on the former; both
depend only on already-published Identus foundation crates plus ordinary public
Rust libraries. NeoPRISM has already supplied exact-source consumer evidence.

## Normative sources

Issue #382 and M5 coordinator #381 define scope. ADRs 0112, 0133 and 0134,
`RELEASING.md`, the active first-train OpenSpec contract, Cargo package
normalization behavior, crates.io immutable version semantics, repository
constraints and source-distribution policy govern the decision.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Reuse `v0.1.0-rc.1` | `not-adopt` | Git tags/releases are unique and the identity already belongs to the immutable crypto train. | Never for another artifact. |
| Use `did-v0.1.0-rc.1` | `not-adopt` | Short family identity does not preserve the exact public crate brand. | A repository-wide version/tag scheme replaces independent trains. |
| Use one tag per package | `not-adopt` | Splits the intentionally ordered two-package dependency train and doubles approval identities. | Packages gain truly independent versions/releases. |
| Use primary-package tag `identus-did-v0.1.0-rc.1` | `adopt` | Collision-free, Cargo-name aligned, and one immutable identity covers the ordered train. | A future accepted repository-wide release scheme. |
| Generalize the live publisher now | `not-adopt` | #344 is incomplete and no DID publication is authorized. | Candidate approval plus explicit release activation. |
| Add a closed train index plus candidate-only descriptor/builder | `adopt` | Makes future scope data-driven while keeping credentials and protected publication unchanged. | The first publisher is later safely generalized. |
| Rewrite the crypto candidate engine in this slice | `not-adopt` | Risks the published train and expands the review surface without product need. | Shared behavior demonstrably drifts or duplicates in a second activated train. |

## Compatibility and dependency evidence

The first train's descriptor, package manifests, scripts, workflow, release,
checksums and registry state remain unchanged. The new DID evidence uses staged
manifests only; canonical packages stay `0.0.0` and unpublished. The direct
candidate cone is `identus-core`, `identus-derive`, `identus-did`, Axum,
headers-accept, mediatype, serde/serde_json and optional Utoipa. Test-only Tokio,
Tower and uriparse remain test evidence rather than runtime additions.

## Security, privacy and maintenance evidence

The new assembler accepts no token and contains no publish/tag/release command.
It requires an exact current Git SHA and clean worktree by default, stages
outside every Git worktree, rejects links/special files/path traversal/
duplicates/oversize archives, uses exact train scope/order, and atomically
reveals completed evidence. Receipts contain identifiers, tool versions,
digests, sizes, file lists and limitations only.

## Full evidence matrix

- **Current implementation:** exact base
  `96cf5f577b5f3585461d34589aad465297693af0` contains source-only
  `identus-did` and `identus-did-resolver-http`, the published first-train
  implementation, and the immutable source-distribution checker.
- **Consumer evidence:** NeoPRISM source-pinned adoption is recorded under #10;
  midnight-identity, Lace ID Portal and Oxid compatibility evidence is recorded
  under #5. No consumer is mutated here.
- **Primary source URL:** Cargo manifest/package normalization and publication
  rules are documented at https://doc.rust-lang.org/cargo/reference/manifest.html
  and https://doc.rust-lang.org/cargo/reference/publishing.html; repository
  identity is https://github.com/hyperledger-identus/sdk-rust.
- **Pinned source revision:** research and implementation begin from protected
  `develop@96cf5f577b5f3585461d34589aad465297693af0`.
- **Exact version and features:** the candidate is `0.1.0-rc.1`;
  `identus-did` has no features, while the resolver proves default/no-default
  plus optional `openapi` all-feature behavior.
- **License and provenance:** repository and staged packages are Apache-2.0;
  SDK-authored source remains at its exact Git revision and no donor code is
  copied in this slice.
- **MSRV:** declared Rust 1.89.0; preparation and etalon validation use exact
  Rust/Cargo 1.98.1.
- **Target evidence:** native Linux/macOS plus existing WASM, iOS ARM64 and
  Android ARM64 compile evidence remain governed by the slow line. The Axum
  resolver is host-only and makes no portable-target claim.
- **Direct and resolved dependency cone:** direct dependencies are enumerated
  above. The exact resolved dependency cone and license/advisory receipt will
  be produced by locked Cargo/Nix implementation checks; it is currently an
  unrun check rather than an invented result.
- **Unsafe and native-code evidence:** SDK crates inherit `unsafe_code =
  "forbid"`; no native library is intentionally introduced. Resolved transitive
  unsafe/native evidence remains an unrun candidate/slow check.
- **Supply-chain evidence:** package file lists, normalized manifests, SHA-256,
  exact tool versions and immutable source revision enter the receipt; SBOM and
  attestation remain later promotion evidence.
- **Public and wire compatibility:** staged metadata and archive shape change,
  but canonical Rust public API and DID/HTTP wire behavior do not. A later API
  baseline/SemVer gate precedes release activation.
- **Facade boundary:** NeoPRISM's compatibility facade and dynamic server
  composition stay downstream; the SDK owns only generic DID and HTTP binding
  packages.
- **Rollback:** revert the additive index, descriptor, checker, assembler,
  tests and ADR; no registry or consumer state changes.
- **Maintenance, release and security posture:** M5 has a coordinator, exact
  package owners and protected-review gates. Publication and settings remain
  with human release/admin authority.
- **Protocol or draft currency:** no DID or HTTP protocol/draft profile changes;
  the current accepted DID/HTTP contracts remain frozen. This is release
  engineering only.
- **Exact commands and unrun checks:** inspected with `cargo metadata`, Cargo
  manifests, release scripts/tests, `scripts/factory check`, OpenSpec readiness
  and Git/GitHub exact-state commands. Candidate assembly, mutation, closure,
  dependency, Nix and hosted checks are deliberately unrun before preflight.

## Rejected or deferred candidates

Publisher/workflow generalization, namespace reservation, registry upload,
trusted-publisher configuration, DID binding package releases, NeoPRISM edits,
SBOM/API-baseline promotion evidence and release approval are deferred to
separate M5 slices.

## Open questions and blockers

No implementation blocker remains. The candidate will prove local archive
closure with `[patch.crates-io]`; it is not registry-resolution evidence. Name
availability must be rechecked only in an authorized release window.

## Evidence commands

Before implementation: inspect the current manifests, first-train descriptor,
candidate/checker/publisher/tests, active release OpenSpec, ADRs and M5 issues;
run factory research/constraint/strict validation and persist preflight.
Candidate assembly, mutation tests and cleaned-source validation remain
implementation tasks.
