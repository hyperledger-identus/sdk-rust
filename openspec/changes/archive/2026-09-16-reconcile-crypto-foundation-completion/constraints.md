# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/286
Constraint blockers: none

## Existing entries affected

- `SDK-REPO-002`: publication, release, tags, and `main` remain prohibited.
- `SDK-COMPAT-005`: the temporary Rust 1.98.1 compatibility policy remains.
- `SDK-LIM-009`: fast/slow evidence remains temporary and separate.
- `SDK-LIM-007`: #298/#299 input-resource hardening remains open and is not
  erased by a functional-delivery decision.
- IDR-011 retains immutable release/publication ownership; IDR-044/#163 retains
  bindings; downstream adoption and Apollo lifecycle remain separate.

## Introduced or changed constraints

IDR-004 MAY be marked `delivered` only when every named functional surface has
immutable implementation evidence, standards/Apollo vectors and accepted
limitations, the executable parity report contains no `gap`, JOSE children are
complete, fuzz/coverage/target evidence is explicit, and release/adoption state
is stated separately. A delivered row SHALL NOT activate publication, support,
consumer mutation, or donor deprecation.

## Introduced or changed limitations

The completion report is an evidence index, not a fresh cryptographic audit.
Coverage is line-only; portable targets are compile-only; Apollo has no
comparable performance harness; the candidate is unpublished; no supported
language binding exists; #298/#299 hardening remains open; NeoPRISM adoption is
unmerged; no Apollo lifecycle decision is made.

## Consumer and product impact

Consumers gain a durable statement that the chain-neutral functional
foundation exists and may be evaluated. They do not gain a released version,
SemVer commitment, production support, binding, certification, migration, or
permission to delete downstream code.

## Activation and rollback

Activation requires the report, backlog/spec/roadmap consistency, local gates,
signed PR, green exact-head CI, merge to `develop`, and post-merge discussion
and issue reconciliation. Rollback restores IDR-004 to `in_progress` and
removes the completion requirement/report without changing runtime code.

## Evidence

Evidence is the sources and exact revisions in `research.md`, the completion
report produced by this change, and the existing machine validators. No
constraint blocker remains.
