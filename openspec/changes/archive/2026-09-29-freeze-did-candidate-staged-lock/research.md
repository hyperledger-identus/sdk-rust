# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

The current implementation at protected `develop` revision
`7c7c8e9a02bebd65d63c00631208283778f12fe4` renders release-shaped
`0.1.0-rc.1` manifests in a VCS-independent scratch workspace. Each matrix lane
then creates an empty Cargo home, runs `cargo generate-lockfile`, executes only
`--locked` commands, and reports the resulting SHA. Aggregation rejects four
otherwise successful lanes unless their lock hashes are identical.

This normally converges, but it is time-dependent consumer evidence: Linux and
macOS jobs and their primary/MSRV operations resolve against registry state at
different moments. The SDK source revision alone therefore does not completely
determine its candidate dependency cone.

## Normative sources

- Issue #482, final coordinator #388, and the post-green discovery review on
  PR #481 define the required outcome and non-goals.
- ADR 0155, `docs/release/did-candidate.toml`, and
  `scripts/prepare-did-candidate.py` define staged identity and the exact
  four-lane compiler/host/target evidence contract.
- Cargo states that `generate-lockfile` rebuilds an existing lock with the
  latest available versions, while `--locked` fails if a lock is absent or
  would change: https://doc.rust-lang.org/cargo/commands/cargo-generate-lockfile.html
- Cargo recommends version-controlling `Cargo.lock` when CI and MSRV checks
  must use exact versions across time and systems:
  https://doc.rust-lang.org/cargo/faq.html#why-have-cargolock-in-version-control
- Cargo gives lockfile versions priority during dependency resolution:
  https://doc.rust-lang.org/cargo/reference/resolver.html#lock-file
- Cargo 1.89 creates lockfile format v4 and Rust 1.78+ can consume it, so the
  selected Rust 1.89.0 MSRV and 1.98.1 primary compilers share a supported
  format: https://doc.rust-lang.org/cargo/CHANGELOG.html

All imported source and manifests retain repository Apache-2.0 license and
provenance. The generated lock contains registry package identities/checksums,
not donor code.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Keep four independent `generate-lockfile` calls | `not-adopt` | Registry timing can change a lane's valid resolution after source freeze. | Never for one aggregate receipt. |
| Add a preparation job and pass its generated lock to native jobs | `not-adopt` | Adds orchestration/artifact trust and resolves mutable input only after merge. | A general hosted release orchestrator owns signed build inputs. |
| Adapt the canonical workspace `Cargo.lock` | `not-adopt` | Its unpublished workspace topology and `0.0.0` package identities differ from staged release manifests. | Canonical and staged manifests become identical. |
| Commit one descriptor-bound staged lock and copy it into every stage | `adopt` | Makes exact source plus reviewed lock determine all lane dependency versions and works with both selected Cargo versions. | Cargo provides a signed immutable registry snapshot primitive with equivalent reviewability. |
| Vendor the full dependency source closure | `not-adopt` | Greatly expands repository size, update surface, and provenance duties for a candidate-only train. | Offline/air-gapped release becomes a product requirement. |
| Use unstable `--publish-time` resolution | `not-adopt` | Requires nightly/unstable behavior and remains best-effort, conflicting with stable-only candidate policy. | The option stabilizes with semantics at least as strict as a committed lock. |

## Compatibility and dependency evidence

The exact candidate remains version `0.1.0-rc.1` with its existing default,
no-default, all-features, and `openapi` profiles. No direct manifest dependency
or feature changes. The resolved dependency cone becomes explicit and immutable
for candidate evidence; consumers of published library crates still resolve
from manifests, so this lock is evidence rather than a public dependency
promise. Public API, wire behavior, facade boundaries, MSRV, native hosts, and
portable target claims do not change.

## Security, privacy and maintenance evidence

The lock adds package versions, sources, dependency edges, and registry
checksums only; it contains no credentials, prompts, private data, or runtime
values. No unsafe or native-code dependency is added—the existing cone is
frozen—and license/advisory/deny/SBOM checks remain supply-chain evidence. The
checker rejects path/Git sources or descriptor/hash drift through existing and
new policy. Maintenance requires an explicit reviewed lock refresh whenever a
candidate manifest/dependency intentionally changes. Release and security
posture otherwise remain M5 policy. Protocol or draft currency is not
applicable because this is build provenance, not SSI behavior. Rollback removes
the staged lock binding but knowingly restores time-dependent evidence.

## Rejected or deferred candidates

Independent resolution, workflow-transferred locks, canonical-lock rewriting,
vendoring, and unstable publish-time filtering are rejected as recorded above.
Dependency upgrades, final publication, registry adoption, and slow dispatch
remain deferred to their existing issues and human authority.

## Evidence commands

Exact planned commands are focused checker/mutations, candidate preparation,
primary and MSRV matrix lanes against the same lock SHA,
`nix flake check`, `./scripts/factory check`, strict OpenSpec validation,
workflow lint, local exact-diff review, and hosted required CI. The intentionally
unrun evidence is the complete Linux/macOS slow matrix; #388 runs it naturally
or with separate explicit authority after this change merges.

## Open questions and blockers

None. The staged manifest identity, compiler formats, ownership, and update
boundary are explicit.
