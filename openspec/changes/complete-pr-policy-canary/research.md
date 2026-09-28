# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

The current implementation temporarily declares both events for the documented
two-integration rollout. The protected base now owns the trusted event.

## Normative sources

Issue #432, issue #341, ADR 0166, and the archived base-owned policy OpenSpec
record are the authority.

## Candidate decisions

Remove the legacy event now: `adopt`. Retain both events: `not-adopt` because it
leaves a PR-controlled workflow path active.

## Compatibility and dependency evidence

No dependency, MSRV, target, API, wire, or consumer compatibility changes.

## Security, privacy and maintenance evidence

The change removes authority and executes no head content. No user data,
secret, credential, native code, or unsafe code is introduced.

## Rejected or deferred candidates

Policy monotonicity remains deferred to #339.

## Open questions and blockers

None.

## Evidence commands

Factory readiness, Node source tests, actionlint, yamllint, exact-diff review,
and natural hosted CI. Hosted evidence is unrun at planning time.
