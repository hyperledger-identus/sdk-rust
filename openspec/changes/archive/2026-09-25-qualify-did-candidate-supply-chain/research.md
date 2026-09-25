# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-25
Source retrieval date: 2026-09-25
Research blockers: none

## Problem and existing implementation

The #382 candidate builder produces byte-identical archives for `identus-did`
and `identus-did-resolver-http`, inspects normalized manifests and builds/tests
12 feature-profile operations from extracted archives. The committed
descriptor records exact package metadata and dependency classes. It does not
yet retain an API baseline or software bill of materials.

The first crypto train already uses exact `cargo-public-api 0.52.0`,
`cargo-semver-checks 0.50.0`, `cargo-cyclonedx 0.5.9` and CycloneDX 1.5.
Reusing those exact versions avoids a second repository tool policy. Its API
baseline is generated from explicit rustdoc JSON under Rust 1.98.1 so ambient
nightly selection cannot change the result.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Compare against workspace `0.0.0` | `not-adopt` | That unpublished development identity is not a released compatibility baseline and contains the same current source. | Never as a release baseline. |
| Run SemVer comparison against the candidate itself | `not-adopt` | A self-comparison is green but proves nothing. | Never. |
| Record first-candidate SemVer status as `not-applicable` | `adopt` | Honest for an initial public API origin; the committed snapshots become future baseline evidence. | A later candidate or release has a real predecessor. |
| Reuse crypto train tool versions | `adopt` | One repository policy, already exercised and reviewed. | Tool incompatibility with a DID surface is evidenced. |
| Generate rustdoc JSON explicitly | `adopt` | Binds public-api rendering to the etalon compiler rather than ambient nightly discovery. | Stable Rust directly supports the required JSON path. |
| Retain one CycloneDX JSON document per package | `adopt` | Machine-readable component/dependency/license evidence with exact package identity. | Repository adopts another governed SBOM standard. |
| Add a second advisory database run inside the candidate | `not-adopt` | Duplicates repository Cargo-deny/Nix security policy and adds mutable network state to deterministic assembly. | Candidate dependency graph diverges from the locked workspace. |
| Claim SBOM as provenance attestation | `not-adopt` | A local JSON artifact is evidence, not a signed registry/GitHub attestation. | Protected release workflow creates signed provenance. |

## Normative sources

Issues #381/#384, ADRs 0113, 0134 and 0153, the canonical
`release-candidate-trains` specification, the #382 descriptor/receipt and
repository Cargo-deny policy govern this slice. Primary source URL references
are https://doc.rust-lang.org/rustdoc/unstable-features.html#json-output and
the official tool repositories at https://github.com/Enselic/cargo-public-api
and https://github.com/CycloneDX/cyclonedx-rust-cargo.

## Compatibility and dependency evidence

Consumer evidence already exists under #5/#10 and is not regenerated or
mutated here. The direct and resolved dependency cone is taken from the exact
staged manifests/lock plus CycloneDX output; candidate qualification rejects
Git sources. Public and wire compatibility are unchanged because this slice
adds release evidence only. NeoPRISM's facade boundary remains downstream.

## Full evidence matrix

- **Current implementation:** protected base
  `0fed1ec7f002fbf9ca5d7b84eaa0e7b62b068183` contains the archived #382
  contract, descriptor, assembler and exact candidate hashes.
- **Primary sources:** Cargo rustdoc JSON, cargo-public-api and
  cargo-cyclonedx behavior; official tool repositories and Cargo metadata are
  the implementation references.
- **Exact tools:** Rust/Cargo 1.98.1, cargo-public-api 0.52.0,
  cargo-semver-checks 0.50.0 (recorded but not executed without a predecessor),
  cargo-cyclonedx 0.5.9 and CycloneDX 1.5.
- **Features:** `identus-did` uses its only surface; the resolver API/SBOM uses
  all features including optional `openapi`.
- **License/provenance:** SDK source is Apache-2.0 at one exact Git revision;
  normalized archive and SBOM identities must agree. Transitive license policy
  remains the locked repository Cargo-deny/slow evidence.
- **MSRV/etalon:** declared Rust 1.89.0 is unchanged; evidence generation uses
  exact Rust/Cargo 1.98.1.
- **Targets:** no new target claim. Host/API/SBOM evidence is target-neutral;
  portable platform proof remains a separate M5 slice.
- **Dependency cone:** staged manifests require exact published
  `identus-core`/`identus-derive` and exact candidate `identus-did`; normalized
  manifests and SBOMs must contain no Git source.
- **Unsafe/native code:** first-party unsafe remains forbidden; this slice adds
  no runtime dependency or native code. Transitive unsafe remains advisory
  evidence rather than absence proof.
- **Supply chain:** API/SBOM files are bounded, regular, non-symlink outputs;
  digests enter an atomic receipt. No credential or remote mutation exists.
- **Compatibility:** no Rust/wire behavior changes. The new snapshots establish
  the comparison origin; they do not grant stable compatibility.
- **Rollback:** remove descriptor tool/baseline fields, generated baselines,
  evidence generation and tests. Candidate archives and remote state remain.
- **Maintenance/security:** #381 owns promotion; #344 owns admin publishing.
  The existing protected crypto train stays unchanged.
- **Protocol currency:** no W3C or HTTP behavior changes.
- **Commands still unrun:** evidence tool installation, baseline generation,
  candidate rerun, mutation/factory/Taplo/hosted checks occur only after the
  planning receipt.

## Security, privacy and maintenance evidence

Evidence generation runs only after the existing closed candidate checker.
The subprocess allowlist gains only local Cargo rustdoc/public-api/cyclonedx
operations. Generated files are size-bounded, JSON is structurally validated,
component identity/version must match the descriptor, and every retained file
receives SHA-256. No advisory database or credential is accepted.

No secret, PII, FFI or runtime surface is added. Maintenance stays with the
release-candidate tooling owner; exact tools and committed baselines make drift
reviewable. Supply-chain evidence is local and does not become an attestation.

## Rejected or deferred candidates

SemVer self-comparison, workspace `0.0.0` comparison, a second mutable advisory
database run, signed provenance, platform qualification, publication and
consumer changes are rejected or deferred as recorded above. Protocol or draft
currency review is not triggered because no protocol behavior changes.

## Open questions and blockers

None. The first-candidate compatibility limitation is explicit rather than
papered over with a meaningless self-check.

## Evidence commands

Before implementation, inspect descriptor/assembler/checker/tests and run
`scripts/factory research-ready`, `constraints-ready`, strict OpenSpec
validation and preflight. After preflight, run exact tool version checks,
baseline initialization, normal clean candidate assembly, mutation suites,
Taplo, factory tests and exact-head hosted CI. These implementation checks are
currently unrun rather than represented as results.
