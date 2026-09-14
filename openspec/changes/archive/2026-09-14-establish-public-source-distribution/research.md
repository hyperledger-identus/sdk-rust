# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-14
Source retrieval date: 2026-09-14
Research blockers: none

## Problem and existing implementation

- The repository is publicly readable at
  `https://github.com/hyperledger-identus/sdk-rust`.
- Every workspace package remains `publish = false` and version `0.0.0`.
- NeoPRISM PR #324 consumes `identus-core`, `identus-derive`,
  `identus-crypto`, `identus-did`, and `identus-did-resolver-http` through
  Cargo Git dependencies pinned to exact revision
  `04b45b7fceb094ae601e3cf7a3291de0a0248b57`.
- On NeoPRISM exact head `757531e2d7e26ab5a0c950b619c5f5cdfb0a0735`,
  anonymous source resolution and its Checks, Coverage, and PRISM specification
  conformance workflows passed without repository-specific credentials.
- Current repository documentation identifies `develop` as a pre-release line
  but supplies no normative source-consumption contract.

## Normative sources

The Cargo Book permits Git dependencies to use a commit through `rev`; Cargo
records the resolved commit in `Cargo.lock` and does not move it until an
explicit update. A Git dependency without `rev`, or one that names a branch,
does not meet this SDK's immutable alpha boundary. Cargo also does not permit a
published crates.io package to retain Git dependencies, so this channel is for
source consumers and must be replaced before registry publication.

The Nix reference manual records GitHub flake inputs by exact `rev` and
`narHash` in `flake.lock`. sdk-rust is not offered as a supported Nix package or
flake output; Nix consumers build their own Cargo graph and preserve the Cargo
Git revision plus their normal fixed source/lock evidence.

Sources:

- https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
- https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
- https://releases.nixos.org/nix/nix-2.34.1/manual/command-ref/new-cli/nix3-flake.html
- https://github.com/hyperledger-identus/sdk-rust/issues/255
- https://github.com/hyperledger-identus/neoprism/pull/324

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| Public HTTPS Git dependency with exact 40-hex `rev`, committed Cargo lock and protected-commit receipt | GitHub exact commit | `adopt` | Anonymous, exact source identity with no new distribution service; already proven by NeoPRISM. | A protected registry candidate is accepted. |
| Registry prerelease | future `0.x` | `conditional-adopt` | Registry checksum and SemVer are desirable, but namespace, release matrix, protected publishing and release authority are incomplete. | Issue #3 ownership and the release-candidate gates are complete. |
| Authenticated private Git preview | repository credential | `not-adopt` | Personal or repository credentials leak distribution policy into every consumer. | Never for public downstream readiness. |
| Floating `develop`, tag or PR ref | mutable reference | `not-adopt` | Resolution can move or disappear and cannot identify immutable evidence. | Never for an accepted alpha source artifact. |
| Vendored SDK source in each downstream | copied source | `not-adopt` | Duplicates provenance, patches and security maintenance. | A legally required offline distribution model is accepted. |
| SDK-owned Nix package or overlay | none | `not-adopt` | Creates a second package surface before Rust packaging stabilizes and has no named consumer need. | A named consumer requires an SDK-owned Nix artifact. |

## Compatibility and dependency evidence

The five proven package names are the identities declared by their Cargo
manifests. `identus-apollo` remains NeoPRISM's compatibility facade and is not
an alias for `identus-crypto`. Source consumers omit `version` because the Git
revision, not the workspace `0.0.0`, selects the artifact. Features remain
explicit and consumer-owned.

The current implementation changes documentation and an offline checker only.
Public and wire compatibility are unchanged. The exact version and feature
metadata of every SDK crate remains as-is, and the MSRV/temporary compiler floor
remains Rust 1.98.1. Both the direct and resolved dependency cone are unchanged;
no dependency is added or removed. The `identus-crypto` facade boundary stays
SDK-owned while the `identus-apollo` facade remains downstream-owned.

## Security, privacy and maintenance evidence

| Threat | Control |
| --- | --- |
| Mutable branch or tag silently changes code | exact full commit in every manifest and resolved lockfile |
| Similar package name selects a registry crate | explicit `git` URL, exact package identity and no `version` key |
| Consumer updates without review | dependency-update PR records old/new SHA, features, gates and provenance |
| Private credential becomes an undeclared prerequisite | public HTTPS URL and anonymous downstream canary evidence |
| Git source is treated as a release | alpha/WIP wording, `publish = false`, `0.0.0`, and explicit no-release limitation |
| Nix build fetches mutable source | committed Cargo.lock and ordinary Nix fixed/locked source evidence |

No credential is added, stored, logged or required. The public repository is
the access boundary; consumers still verify the selected exact revision and
retain lock evidence. The documented update procedure prevents silent
dependency movement. Apache-2.0 remains the SDK source license, and dependency
licenses remain governed by existing Cargo/Nix gates.

The channel adds maintenance work only when a consumer deliberately advances
its revision. It avoids a new registry, service, token or signing key. It does
not weaken secret handling, unsafe-code policy or input-boundary controls.

Unsafe and native-code evidence is unchanged because the implementation adds no
Rust, dependency, build script, FFI or native artifact. Supply-chain evidence is
the canonical public repository, the protected exact commit, its signed/DCO PR
history, Cargo lock record, Nix locked/fixed source evidence, and the existing
license and provenance gates. No SSI protocol or draft version is selected or
changed by this distribution decision.

### Dependency and target impact

No workspace dependency, feature, API, wire type, error contract, compiler or
target changes. The consumer floor remains Rust 1.98.1. No new runtime or build
dependency enters an SDK package.

## Rejected or deferred candidates

Authenticated previews, floating references, vendoring and an SDK-owned Nix
package are not adopted for the reasons in the candidate table. Registry
distribution is conditionally selected as the successor but remains deferred
until protected namespace, compiler-matrix and release evidence exist.

## Open questions and blockers

No implementation blocker remains. GitHub's enterprise policy currently
prevents repository-level activation of secret scanning, but this source-only
documentation change introduces no credential and does not depend on that
control. The settings deviation remains tracked under issue #26.

Rollback is a focused revert of the documentation, capability specification and
offline checker; it does not rewrite or invalidate an already pinned consumer
commit.

## Evidence commands

- `gh issue view 255` retrieved the scope and downstream receipt.
- `gh pr diff 324 --repo hyperledger-identus/neoprism` verified the five exact
  package declarations and immutable SDK revision.
- `gh api` verified current public visibility and repository controls on
  2026-09-14.
- Cargo and Nix primary documentation was retrieved on 2026-09-14.
- `scripts/factory doctor` passed at exact base
  `e197810b7d94cfa2e9182b7413110c5edac944db`.
- Exact implementation commands and their outputs will be recorded in
  `verification.md`. Focused checker tests, factory readiness, Cargo quality
  gates and `nix flake check` are currently unrun checks and remain explicit
  tasks rather than claimed evidence.

## Reconsideration triggers

- A protected, publishable SDK release candidate is accepted.
- crates.io ownership and trusted publishing are operational.
- Cargo changes Git dependency or lockfile semantics materially.
- A named consumer needs an SDK-owned Nix package rather than a Cargo source
  dependency.
