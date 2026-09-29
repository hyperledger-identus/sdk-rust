# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

At protected `develop@f7a0d58b534a0af2a5cd135fa2bab71f77195991`,
`validate_result` performs this transition:

1. reject a method-mismatched job as `JobMismatch` before state inspection;
2. require terminal `Finished`/`Failed` states without a job and non-terminal
   `Action`/`Wait` states with a job;
3. for `Finished`, check handle cardinality before DID/document identity, then
   validate the optional public document;
4. for `Failed`, require an optional DID to use the result method;
5. for `Action`, combine optional-DID method and action-continuation identity
   failures as `ActionMismatch`;
6. for `Wait`, combine optional-DID method, wait-continuation, and advisory
   duration failures as `InvalidState`;
7. after a valid state, map any document-metadata identity/shape failure to
   `MethodOrDidMismatch`; and
8. finally revalidate document-metadata extensions through bounded
   `RegistrationPublicData`, preserving its registration errors.

The public constructor validates before retaining its owned method, job, state,
registration metadata, and document metadata. Existing tests cover ordinary
terminal/job mismatch, wait bounds, handle bounds, private metadata, valid
action continuation, and request correlation, but do not freeze combined-fault
priority across all phases.

The canonical signal is 75 SLOC / cognitive 18 / cyclomatic 28. The containing
module is below 1,000 authored nonblank lines, and no production module exceeds
that threshold.

## Normative sources

Issue #459, Discussion #399, ADR 0115, the canonical DID registration
module-ownership specification, error/resource/unsafe/support constraints, and
spec-driven factory delivery are authoritative. This slice changes no DID
Registration specification profile or adapter contract, so no new external
standard revision is adopted.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| One private borrowed validator with lifecycle-specific phase methods | `adopt` | It keeps one state-machine owner while separating stable reasons to change and adding no retained state. | Characterization cannot be preserved or a replacement method crosses a threshold. |
| Keep the function intact and document an exception | `not-adopt` | Correctness outranks metrics if phase ownership obscures first-error order. | Stop/go review rejects decomposition. |
| One helper per enum variant or conditional | `not-adopt` | Mechanical forwarding fragments one lifecycle contract and displaces metrics. | Never as a metric-only technique. |
| Generic rule table or callback pipeline | `not-adopt` | Indirection would hide error precedence, add dispatch machinery, and reduce Rust exhaustiveness. | A separately approved declarative policy is needed by multiple state machines. |
| New validation/error crate | `not-adopt` | The invariants belong to `identus-did` registration and have no independent public capability. | A separately researched cross-crate contract emerges. |

## Compatibility and dependency evidence

The crate-root types, constructor, accessors, derives, debug redaction, exact
errors, state/job values, collection limits, metadata validation, manifests,
lockfile, features, MSRV, and targets remain unchanged. No normal or development
dependency is added. `registration_metadata` is already validated at its own
construction boundary and remains intentionally outside result-state checks.

## Security, privacy and maintenance evidence

All inputs are borrowed and already bounded validated SDK values except the
bounded secret-handle vector and document tree. Existing cardinality, iterative
JSON cleanup, public-document traversal, metadata limits, and static redacted
errors remain authoritative. The private validator stores only references,
adds no clone/allocation, and cannot escape the module. No unsafe code,
recursion, callback, trait object, ambient I/O, synchronization, or panic path
is introduced.

## Rejected or deferred candidates

Changing error granularity, accepting terminal jobs, accepting non-terminal
states without jobs, normalizing method/DID identity, merging request/result
validation, moving public-data validation, adopting a generic state-machine
framework, and changing registration limits are rejected or deferred.

## Open questions and blockers

There are no planning blockers. Characterization is a stop/go gate. If a
private validator cannot preserve the exact first-error matrix without helper
sprawl or repeated matching, production remains unchanged and the signal
receives a measured exception.

## Evidence commands

Planning inspected the exact protected base, issue #459, constructor and all
registration result tests, registration errors, public-data/document-metadata
validators, ADR 0115, module ownership, dependencies, and the canonical
code-health report. Before production edits, run the focused registration suite
and add the combined-fault matrix. Afterward run focused/workspace tests, strict
Clippy/format/docs, public/source/code-health/factory checks, portable targets,
relevant Nix gates, distinct exact-diff review, and protected exact-head CI.
