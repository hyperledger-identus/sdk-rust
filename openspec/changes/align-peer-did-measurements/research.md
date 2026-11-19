# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-11-19
Source retrieval date: 2026-11-19
Research blockers: none

## Problem and existing implementation

Issue #607 owns confirmed findings F-010, F-011, and F-013 from the review of
PR #604. The current Node harness statically imports `dist/node.js` before its
timer, reports only absolute artifact totals, and times provider construction
as initialization. Its quality declaration names browser and React web even
though no performance harness executes there. The parity ledger calls three
private targets delivered while the peer quality record still excludes
external adoption, accepted budgets, rollback rehearsal, publication, and
support.

## Normative sources

- Issue #607 and the immutable review comment on PR #604 define the repair.
- PR #604 compares pre-peer base `bd1981372fcb08772d1196131a0cf26291fad9cb`
  with peer candidate `afd637809e28e02a7369b8b1748d362f74113eb9`.
- The canonical `sdk-ts-peer-did-method` contract requires source-bound,
  measurement-only quality evidence and retains all product gates.
- The quality-evidence and parity-conformance schemas require exact selectors,
  targets, revisions, limitations, missing dimensions, and non-claims.

## Candidate decisions

| Candidate | Decision | Reason |
| --- | --- | --- |
| Keep absolute candidate bytes only | `not-adopt` | It cannot attribute peer-facade artifact change. |
| Compare independently produced totals | `not-adopt` | Different toolchains or compression definitions are not comparable. |
| Compare exact PR base and candidate with one toolchain and canonical file inventory | `adopt` | It produces a reviewable matched delta. |
| Call provider construction cold start | `not-adopt` | The binding is already imported and initialized. |
| Use isolated Node processes for cold import plus cached import for warm initialization | `adopt` | It separates import-inclusive initialization from provider construction. |
| Retain browser/React benchmark labels | `not-adopt` | Their receipts are functional, not performance measurements. |
| Remove browser/React functional receipts | `not-adopt` | They remain valid private target evidence. |

## Compatibility and dependency evidence

The comparison reuses the current package, manifest, and Node runtime without
adding a dependency or changing a declaration, generated artifact, package
entrypoint, provider, or consumer contract. Baseline and candidate are built
with the same pinned Nix/Node/Rust/WASM toolchain.

## Security, privacy and maintenance evidence

The measurement runner reads public build artifacts and writes a private local
JSON receipt. It uses deterministic file ordering and compression settings,
bounded sample counts, no network, no secrets, and no runtime fallback. The
change adds no dependency and does not alter package or WASM behavior.

## Rejected or deferred candidates

Browser/React performance harnesses, accepted budgets, external adoption,
publication, support, release, React Native, and mdoc remain deferred. Their
absence is recorded rather than inferred from functional target receipts.

## Open questions and blockers

None. Hosted CI and protected integration are later gates, not research
blockers. Accepted budgets, external adoption, and product support remain
separate decisions.

## Evidence commands

Planning evidence uses factory doctor, strict OpenSpec validation, research
readiness, constraint readiness, factory check, and issue-bound preflight.
Implementation evidence will run the matched measurement command, receipt
schema checks, parity and quality validators, isolated TypeScript targets,
factory check, and full Nix validation. Final exact-revision timing evidence is
intentionally unrun at planning time.

## Selected approach

Define a versioned receipt schema containing exact baseline/candidate source
revisions, toolchain/environment identity, per-target raw and deterministic
gzip byte totals, signed deltas, import-inclusive cold/warm Node measurements,
provider construction, existing operation measurements, aggregation rules,
and limitations. Generate the final source-bound receipt locally from the two
exact revisions and retain its digest with the PR evidence.

## Provenance

Repository-owned files are Apache-2.0. No external source or fixture bytes are
copied by this change.
