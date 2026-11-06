# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-10-29
Source retrieval date: 2026-10-29
Research blockers: none

## Problem and existing implementation

The current implementation at protected
`develop@88efe31e3d413e6f51b0cb80f65155281f8feb3c` is the generic
`identus-did` JSON-wire validator and its public re-export. ADR 0154
deliberately established committed cargo-public-api text as the
unpublished `0.1.0-rc.1` compatibility origin. Since that snapshot was
generated, generic duplicate-aware and resource-bounded JSON validation became
part of `identus-did` and was publicly re-exported for reuse by DID method
implementations. The current product source is already merged and covered by
unit, property, fuzz, portable-target, and downstream PRISM evidence.

Consumer evidence is the already-merged DID document and resolution use of the
validator, the portable peer DID method, and the read-only NeoPRISM PRISM
integration evidence governed by #388. No consumer source is changed here.

The baseline SHA-256 is
`3ce68d571d087e78ece200154ba0e185565fbf5da9c2733eb7f70d3b1f7bbf9d`.
Review-mode qualification under #581 generated the current API and completed
archives, closure, API extraction, and SBOMs; normal mode then stopped only at
this API comparison. The resolver HTTP baseline remains SHA-256
`9c432f0d0c3d5db92da3aa88aed05164aa2adf483c11be84b9239c9973179df6`.

## Normative sources

Issues #381/#388/#582, ADR 0154, the `release-candidate-trains`
specification, `docs/release/did-candidate.toml`, and
`scripts/prepare-did-candidate.py` govern this slice. Rust public API
classification follows the Rust API Guidelines and cargo-semver-checks
lints; the exact cargo-public-api text remains the repository's committed
origin format.

Primary source URLs:

- https://rust-lang.github.io/api-guidelines/future-proofing.html
- https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute
- https://github.com/obi1kenobi/cargo-semver-checks
- https://github.com/Enselic/cargo-public-api

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Keep the stale origin | `not-adopt` | Normal candidate qualification correctly fails and cannot represent current source. | Never while this source remains the candidate. |
| Remove API comparison | `not-adopt` | Hides drift and weakens the release gate. | Never. |
| Treat workspace `0.0.0` as predecessor | `not-adopt` | ADR 0154 forbids a false released-predecessor claim. | A real published predecessor exists. |
| Rebaseline exact additive output before publication | `adopt` | The first candidate is still unpublished and the diff can be reviewed exhaustively. | Publication or a later candidate exists. |
| Rebaseline resolver HTTP too | `not-adopt` | Its generated API is byte-identical. | Its exact API changes. |
| Add new Rust behavior in this slice | `not-adopt` | Would mix product design with release-evidence repair. | A separate issue and OpenSpec authorize it. |

## Compatibility and dependency evidence

The expected diff adds one public limits struct with four `usize` fields, one
non-exhaustive error enum with six variants, and one fallible validation
function. No item is removed or changed. `#[non_exhaustive]` prevents external
exhaustive matching and leaves room for future failure categories. The
function consumes borrowed bytes and explicit finite limits; it introduces no
ambient I/O, allocation authority, secret exposure, or platform dependency.

The exact version and features remain candidate `0.1.0-rc.1`, with
`identus-did` using its no-feature public surface and
`identus-did-resolver-http` retaining its default, no-default, all-feature,
and `openapi` profiles. The direct and resolved dependency cones are unchanged
from #581. Rust 1.89.0
MSRV, Rust 1.98.1 etalon, features, Linux/macOS host lanes, WASM/iOS/Android
compile claims, license/provenance, unsafe/native-code policy, and facade
boundaries remain unchanged. NeoPRISM is read-only and is not mutated.

Because no released predecessor exists, cargo-semver-checks cannot honestly
produce a release-to-release compatibility verdict. The equivalent exact
classification is a closed set difference between the committed old API text
and newly generated cargo-public-api output: removals must be empty and
additions must equal the allowlisted JSON-wire surface. A machine receipt binds
both SHA-256 values and exact lines. The repository retains
cargo-semver-checks 0.50.0 for the first genuine predecessor comparison.

## Security, privacy and maintenance evidence

Supply-chain evidence remains the descriptor-bound staged lock, registry
checksums, deterministic archives, patched closure, CycloneDX documents,
Cargo-deny/RustSec policy, source/license checks, and the protected slow lane.
The API receipt contains names, signatures, source revision, hashes, and a
classification only. It carries no runtime values, credentials, PII, secrets,
prompts, or generated binary material. Candidate preparation remains bounded,
credential-free, outside every worktree, and atomic. Existing JSON-wire tests
and fuzz evidence remain the behavioral authority; this slice does not claim
that API text alone proves security.

Maintenance remains with the DID release train. A later API change, candidate,
or publication must use the ordinary comparison gate rather than silently
rewriting this origin. Rollback restores the stale baseline and therefore
reopens the known qualification failure.

Protocol or draft currency is not applicable because no DID/VC protocol
semantics change. Certification and product impacts are unchanged.

## Rejected or deferred candidates

Disabling comparison, self-comparison, workspace-as-release comparison,
resolver rebaseline, runtime changes, publication, downstream mutation, and a
stable API guarantee are rejected or deferred as described above.

## Open questions and blockers

None before implementation. The generated current API, exact set difference,
normal candidate receipt, factory/site/Nix evidence, and independent review
remain unrun until the issue-bound preimplementation receipt is committed.

## Evidence commands

After preflight, run review-mode candidate generation to obtain the exact API,
validate the closed additive diff, update only the DID baseline and receipt,
then run normal `nix run .#did-candidate`, both local matrix lanes,
release-candidate mutations, DID tests, docs site, factory, full Nix, and a
Claude-or-Codex exact-head review. #388 retains cross-host complete slow
evidence and independent release approval.
