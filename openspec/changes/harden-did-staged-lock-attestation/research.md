# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

The current implementation at protected `develop` revision
`f44bda2cd2ee0aac404e4c38d4c51a36a6e437e6` binds the staged DID lock to
SHA-256 `1f1d4206e2ced5bd74675d654876684536dd8f82bf79cd6de4c2fa67db904447`.
Archive assembly and matrix lanes install those bytes before locked package,
test, and check commands. The aggregator currently proves only that the four
lane hashes agree, so four consistently forged receipts could disagree with
the descriptor. Candidate output prints the descriptor value rather than the
digest returned after copying the lock. Rustdoc and CycloneDX run against the
staged workspace without `--locked`. Static policy recognizes only literal
`run(["cargo", "generate-lockfile", ...])` calls, leaving equivalent helper or
variable construction outside its model.

The extracted archive verification workspace is intentionally different: it
patches two packaged crates together and therefore owns one closure-local
generated lock. That lock is not candidate source identity and is never
compared across lanes.

The only consumer of this tooling is the repository release-evidence process;
SDK application consumers receive no new API or behavior.

## Normative sources

- Issue #484 and the exact-head discovery review on PR #483 define the bounded
  gaps and non-goals.
- ADR 0155, `docs/release/did-candidate.toml`, and the canonical
  `release-candidate-trains` specification define candidate identity and
  matrix evidence.
- Cargo documents `--locked` as requiring Cargo to use the existing lockfile
  and fail rather than change it:
  https://doc.rust-lang.org/cargo/commands/cargo-build.html#manifest-options
- Cargo documents `generate-lockfile` as creating or rebuilding dependency
  resolution and allows `--offline`/`--locked` behavior to be selected
  explicitly:
  https://doc.rust-lang.org/cargo/commands/cargo-generate-lockfile.html
- Cargo recommends reviewing version-controlled lock changes for deterministic
  CI and MSRV evidence:
  https://doc.rust-lang.org/cargo/faq.html#why-have-cargolock-in-version-control

No donor code, fixture, protocol interpretation, or new third-party library is
required. All affected source remains Apache-2.0 repository tooling.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| Keep peer-only aggregate comparison | #483 | `not-adopt` | Equal forged/stale lane hashes can still disagree with source identity. | Never for attested candidate evidence. |
| Recompute the repository lock hash during aggregation only | SHA-256 | `not-adopt` | It duplicates descriptor validation and still fails to state the expected identity explicitly. | Descriptor ceases to own candidate identity. |
| Compare each lane to `staged_lock_sha256` | descriptor schema 1 | `adopt` | Direct, deterministic, and preserves existing receipt schema. | Receipt schema carries a signed input manifest. |
| Keep substring or literal-list AST detection | Python 3 AST | `not-adopt` | Alternate helpers and variable-built commands evade the model. | Never as the only control. |
| Runtime-reject generation and statically constrain one purpose helper | Python 3 AST | `adopt` | Resolved commands are checked at execution while AST policy limits the escape hatch to reviewed callers. | Python tooling is replaced by a typed capability runner. |
| Refresh the committed lock in place | Cargo 1.98.1 | `not-adopt` | A convenience command could silently update source identity and descriptor state. | A protected release bot owns an atomic reviewed update PR. |
| Emit a proposed lock plus drift report to a new external output directory | Cargo 1.98.1 | `adopt` | Reviewers receive exact bytes and cone changes; ordinary evidence and repository state stay unchanged. | A signed immutable dependency-input service replaces local refresh. |
| Add another Nix app | pinned flake revision | `not-adopt` | The existing pinned DID candidate app already supplies Rust 1.98.1 and can expose a mutually exclusive mode. | Refresh needs materially different tools. |

## Compatibility and dependency evidence

The current lock bytes, manifests, package versions, features, direct and
resolved dependency cone, APIs,
wire behavior, Rust 1.89.0 MSRV, Rust 1.98.1 primary compiler, profiles, hosts,
and portable target claims remain unchanged. `--locked` changes failure
behavior only when tooling would otherwise rewrite source identity. The refresh
report compares closed `(name, version, source, checksum)` identities and is
review evidence, not a public dependency promise. Public and wire compatibility
are unchanged, and the Identus facade boundary continues to hide Cargo tooling
from SDK APIs.

## Security, privacy and maintenance evidence

The runtime command boundary fails closed after variable construction; AST
policy makes its only generation escape hatch visible. Refresh requires a clean
exact HEAD, an absent external output path, bounded generated lock bytes, and
the same source/checksum validation as ordinary evidence. It never writes the
repository lock or descriptor and has no publishing, Git, GitHub, credential,
network-policy, or workflow authority. Cargo may access the registry while
resolving a proposed refresh; the resulting identities/checksums are the only
retained data. No secret, unsafe code, native dependency, or protocol input is
introduced. The exact repository revision, Apache-2.0 license, and provenance
remain local. Existing Cargo.lock, deny, advisory, SBOM, and checksum checks
remain the supply-chain evidence. Protocol/draft currency is not applicable to
this build-provenance slice; no SSI protocol or draft changes.

## Rejected or deferred candidates

Peer-only aggregation, static literal matching, in-place refresh, and a second
Nix app are rejected as recorded above. Dependency upgrades, publication,
cross-host execution, and a protected bot-driven refresh PR are deferred to
their existing authority paths. Rollback is a normal revert to #483, with the
documented attestation gaps restored.

## Evidence commands

Planned exact commands are `python3 scripts/tests/release-candidates.py`,
`python3 scripts/check-release-candidates.py`, the pinned
`nix run .#did-candidate` build and refresh modes, primary/MSRV candidate matrix
apps, `./scripts/factory check`, strict OpenSpec validation, and
`nix flake check`. Mutation coverage will exercise alternate helper calls, variable-built lock
generation, descriptor/aggregate mismatch, returned/receipt mismatch, and each
staged Cargo command without `--locked`. Focused tests will prove a no-drift
refresh, deterministic report shape, pre-existing-output rejection, and no
repository mutation. Candidate preparation and primary/MSRV lane evidence must
retain the frozen digest. Factory, Nix, strict OpenSpec, local review, and
exact-head CI remain required. The unrun cross-host slow execution stays with
#388; no workflow is dispatched by this change.

## Open questions and blockers

None. The two lock topologies, allowed generation purposes, output ownership,
and protected release boundary are explicit.
