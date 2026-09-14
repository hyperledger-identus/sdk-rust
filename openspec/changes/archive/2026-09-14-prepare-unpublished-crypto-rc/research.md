# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-14
Source retrieval date: 2026-09-14
Research blockers: none

## Problem and existing implementation

`identus-crypto`, `identus-core`, and `identus-derive` inherit workspace version
`0.0.0` and `publish = false`. Their path dependencies do not carry registry
version requirements, package metadata is intentionally sparse, and no
package-archive or clean archive-consumer gate exists. The crypto crate already
has Apollo capability/vector/coverage/portable compile evidence, so adding more
algorithms is not the missing work.

Issue #255 now supports exact public Git source. Issue #3 still owns crates.io
namespace and trusted publishing. `RELEASING.md` requires a separate
release-phase compiler decision before publication.

## Normative sources

- Cargo 1.98.1 `cargo package --help` and the Cargo Book package/publish
  reference define archive assembly, manifest normalization, verification, and
  the requirement for versioned registry dependencies.
- `cargo-semver-checks` 0.50.0 documents baselines from Git revisions, source
  roots, or rustdoc JSON and exit codes 0/100/101. It uses unstable rustdoc JSON
  internally and supports the stable compiler current at its release.
- `cargo-public-api` 0.52.0 produces the reviewable API rendering. Its rustdoc
  JSON operation requires a narrowly scoped, documented `RUSTC_BOOTSTRAP=1`;
  it does not change the candidate compiler or package builds.
- `cargo-cyclonedx` 0.5.9 is the OWASP CycloneDX Cargo plugin and generates
  aggregate dependency SBOMs by invoking Cargo metadata/build-system logic.
- Both tools are available from the repository's already locked nixpkgs input;
  no `cargo install` or floating download is required.
- Apollo deprecation discussion #252 defines the package-candidate outcome and
  explicitly keeps publication and bindings separate.

Sources:

- https://doc.rust-lang.org/cargo/commands/cargo-package.html
- https://doc.rust-lang.org/cargo/reference/publishing.html
- https://github.com/obi1kenobi/cargo-semver-checks/tree/v0.50.0
- https://github.com/CycloneDX/cyclonedx-rust-cargo/tree/cargo-cyclonedx-0.5.9
- https://github.com/hyperledger-identus/sdk-rust/discussions/252
- https://github.com/hyperledger-identus/sdk-rust/issues/266

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Isolated generated candidate workspace | `adopt` | Produces real normalized archives while canonical manifests remain `0.0.0`/non-publishable. | Registry release manifests become canonical. |
| Version all 20 workspace crates now | `not-adopt` | Expands release scope to placeholders and unrelated components. | Each component has an accepted release plan. |
| Make only three canonical manifests publishable | `not-adopt` | Conflicts with the current machine-enforced publication denial. | Human release activation replaces that policy. |
| `cargo-semver-checks` 0.50.0 | `adopt-candidate-gate` | Purpose-built SemVer lints, Git/source baselines, already Nix-pinned. | Tool/Rust incompatibility or unacceptable measured cost. |
| `cargo-cyclonedx` 0.5.9 | `adopt-candidate-gate` | Standard dependency SBOM, already Nix-pinned. | Proven incomplete dependency representation. |
| Hand-written API or SBOM parser | `not-adopt` | Reimplements maintained formats and risks silent omissions. | Adopted tools cannot represent a required surface. |
| Add release tools to normal fast lane | `not-adopt` | Their one-time local setup is large and the evidence is release-specific. | Release cadence and cache evidence justify it. |

## Packaging constraint

Standard `cargo package` verification resolves normalized dependencies through
the target registry. Because `identus-core` and `identus-derive` are deliberately
unpublished, the closure cannot truthfully pass crates.io resolution. The
candidate gate therefore runs Cargo archive assembly, checks the normalized
manifest, extracts all three archives into a clean directory, patches those
exact local archive contents only for internal resolution, and builds/tests the
consumer under the generated lockfile. The receipt calls this archive-closure
verification, not crates.io publication verification. A future publication
gate must run unmodified `cargo publish --dry-run` against the protected
registry workflow.

## Compatibility and dependency evidence

The current implementation uses version `0.0.0`; the candidate uses exact
version `0.1.0-rc.1` for all three packages with exact
`=0.1.0-rc.1` internal requirements. External versions and feature definitions
come from the canonical locked workspace. Canonical workspace manifests remain
`0.0.0`, `publish = false`, so no other crate receives a version or release
promise. `identus-apollo` is not generated.

The MSRV stays Rust 1.98.1 for candidate preparation. Cargo archives are
host-neutral source packages; existing WASM, iOS, and Android compile receipts
remain target evidence rather than archive runtime claims. The direct and
resolved dependency cone is unchanged because staging rewrites only local
package identity and metadata. Public and wire compatibility are unchanged;
the SDK-owned `identus-crypto` facade remains the public facade boundary.

The first candidate establishes the API baseline. `cargo-semver-checks` compares
the candidate surface with the exact pre-candidate protected revision to detect
accidental packaging loss; future candidates compare with the committed
candidate baseline. All feature profiles and existing portable compile receipts
remain separate evidence.

## Security, privacy and maintenance evidence

Candidate staging accepts only repository-owned paths, uses a new temporary
directory, never evaluates user-provided shell text, and excludes secret/local
state. Archive checks reject Git/path dependencies, unexpected files,
unbounded package size, missing license/README, and checksum drift. Secret-key
behavior is unchanged. No reachable unsafe or native code is added by the
pipeline. Existing transitive native/unsafe implementation evidence remains in
the lockfile, parity ledger, and dependency policy.

CycloneDX invokes Cargo and is therefore run only on this trusted repository.
Receipts contain source/tool/artifact identities and command outcomes, never
environment variables or credentials. Candidate tools stay out of the required
fast lane; they are invoked by an explicit slow command.

Supply-chain evidence includes exact source revision, Cargo-normalized package
manifests, SHA-256 archives, license/provenance fields, CycloneDX dependency
identity, tool versions, and protected CI. Maintenance, release, and security
posture remains pre-release and maintainer-owned. Protocol/draft currency is not
changed because this slice packages existing crypto behavior only.

## Rejected or deferred candidates

Registry publication, placeholder reservation, artifact signing/attestation,
foreign-language packages, and downstream migration are deferred. A local
checksum/provenance receipt is evidence for an unpublished artifact, not a
signature or GitHub artifact attestation.

Rollback is a repository revert because no public artifact or external consumer
state changes.

## Open questions and blockers

There is no candidate-preparation blocker. Publication remains blocked on issue
#3, the consumer-driven compiler matrix, assigned release manager/second
reviewer, and protected trusted publishing.

## Evidence commands

- `cargo package --help` from Rust/Cargo 1.98.1.
- locked nixpkgs search resolved `cargo-semver-checks` 0.50.0,
  `cargo-cyclonedx` 0.5.9, and `cargo-public-api` 0.52.0.
- upstream release/README metadata was retrieved from the two official GitHub
  repositories on 2026-09-14.
- Exact command `cargo test --workspace --all-features` passed locally after
  implementation.
- This exact command passed on the clean implementation commit:

  ```console
  nix run .#crypto-candidate -- --output /tmp/sdk-rust-candidate/head \
    --source-revision a87ae6d40877e1761c56f26b642fd0ed0e8e8d35
  ```
- The exact local fast Nix derivations passed: factory contract, Nix/text/TOML
  lint, Rust formatting, all-target build, Clippy, and 701 nextest cases.
- The full candidate gate completed in 44.766 seconds on the development host.
  Archive sizes were 18,526 bytes (`identus-derive`), 12,908 bytes
  (`identus-core`), and 77,913 bytes (`identus-crypto`).
- Double assembly, four feature profiles, API rendering, SemVer comparison, and
  three CycloneDX 1.5 documents passed. Exact-head hosted CI remains delivery
  evidence rather than pre-implementation research.
- Unrun checks: hosted PR policy and exact-head Linux CI; both remain mandatory
  before merge.

## Reconsideration triggers

- A protected crates.io publication is authorized.
- Rust 1.98.1 or the pinned release tools no longer generate the candidate.
- A new candidate changes public API or internal package topology.
- A named consumer requires a signed/attested binary or foreign-language
  distribution.
