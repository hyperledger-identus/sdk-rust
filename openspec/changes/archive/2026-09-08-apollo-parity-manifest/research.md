# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-09
Source retrieval date: 2026-09-09
Research blockers: none

## Problem and existing implementation

Discussion #178 compares Apollo with sdk-rust at immutable revisions and issue
#211 defines the required evidence fields. The report currently has no
repository-local source that proves all audited capabilities remain present,
uses only the four accepted dispositions, references existing SDK tests, or
keeps vector and CI links bound to the declared baselines.

Apollo was inspected read-only at
`ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c`. Its checkout has a pre-existing
dirty nested `secp256k1` submodule and was not changed. The sdk-rust base is
`9ff2fa87337d3f25d67f38486c3a4d0a656bd9d6`; its fast CI run 34262060203 is
green at that exact SHA.

## Normative sources

- Apollo `main@ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c`, Apache-2.0,
  including its common/platform source and test trees.
- sdk-rust `develop@9ff2fa87337d3f25d67f38486c3a4d0a656bd9d6`,
  Apache-2.0.
- Issue #211 for the manifest contract and M2 issue #9 for parity semantics.
- Discussion #178 for the audited capability, dependency, vector and target
  inventory.
- BIP-32, BIP-39, SLIP-0010, RFC 4648 and RFC 8037 vector sources already
  pinned by the delivered crypto tests and their archived OpenSpec evidence.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| TOML manifest plus Python standard-library validator | `adopt` | TOML matches existing architecture ledgers, is readable, requires no dependency, and is available through Python 3.11 `tomllib`. | The repository adopts one signed evidence schema across all conformance ledgers. |
| CSV | `not-adopt` | Nested vector references, rationale and consumer impact become fragile or duplicated. | The contract is reduced to a flat table without provenance relationships. |
| JSON/YAML | `not-adopt` | JSON is noisier for review and YAML would add a parser dependency to the factory. | An existing mandatory factory format supplies stronger schema validation without a new cone. |
| Discussion table as canonical source | `not-adopt` | Remote prose cannot fail the local factory and mutable comments are weaker than versioned evidence. | Never as the sole source; Discussion remains the rendered human view. |
| Build a general evidence framework | `not-adopt` | Issue #211 needs one bounded parity ledger; a generalized framework expands scope before a second use case. | A second independent parity program proves the common schema. |

## Compatibility and dependency evidence

The change adds documentation and Python tooling only. It changes no Rust
public API, wire value, feature, MSRV, target tier, runtime dependency, Cargo
manifest or lockfile. Python uses `tomllib`, `pathlib`, `re` and `argparse`
from the standard library already present in the Nix factory environment.

Capability identifiers are deliberately fixed to the audited Apollo baseline.
Changing that baseline or capability set requires a reviewed manifest/checker
update, making completeness changes visible rather than silently permissive.
The renderer is derived from the same parsed and validated model; it is not a
second hand-maintained table.

## Security, privacy and maintenance evidence

The validator reads bounded repository-local text and performs no network
access, code execution or secret handling. It rejects absolute/traversing SDK
test paths, duplicate IDs, unknown keys/statuses, missing local selectors,
inconsistent immutable links, unknown vector references and summary drift.
Manifest strings are evidence metadata only and are emitted as Markdown text;
they do not enter runtime or protocol surfaces.

The dependency decision is `not-applicable`: no third-party runtime or build
library is introduced. The maintained surface is one small standard-library
checker with mutation-based regression tests. Hosted link availability cannot
be proven offline; link structure and revision binding are enforced, while CI
and review confirm the referenced public artifacts.

## Rejected or deferred candidates

No algorithm, parser, crypto library or donor source is implemented or copied.
Coverage, benchmark execution, target receipts and production bindings remain
#212, #214, #213 and #163/#215 respectively. Monitor-only issue #179 remains
non-blocking and is not consulted by this change.

## Open questions and blockers

None. The four dispositions are already directed by issue #9. Accepted
differences remain explicit and reopen only through their named consumer
trigger; no manifest row is allowed to imply Apollo deprecation or SDK
distribution parity.

## Evidence commands

- `scripts/factory doctor` passed before edits at base `9ff2fa8`.
- `gh run view 34262060203` proved the recorded fast receipt is green on the
  exact sdk-rust baseline.
- `git ls-tree`, `git grep` and `git show` inspected Apollo source/test paths at
  the pinned revision without changing the donor checkout.
- Implementation-dependent checker tests, factory checks, formatting/text
  lint, exact-diff review and hosted CI remain tasks and are not claimed here.
