# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/594
Constraint blockers: none

## Existing entries affected

The M5 readiness manifest and handbook still predate the final exact-SHA slow
receipt and merged NeoPRISM adoption. The SSI backlog still marks IDR-005 as
`in_progress` and IDR-006 as `specified`. The platform milestone registry still
marks completed A0/A1 work as unfinished and has no independently steerable
SDK-Rust/SDK-TS parity contract.

## Introduced or changed constraints

Engineering qualification, downstream adoption, release authorization,
publication, and stable API status SHALL be separate facts. Completed exact-
revision and consumer evidence SHALL NOT imply tag, registry, `main`, support,
or stability authority.

Every normalized SDK-TS capability SHALL have one parity relation and one
conformance state. Platform-owned and intentionally different capabilities
SHALL NOT be forced into semantic equality. Deferred, excluded, blocked, or
not-applicable evidence SHALL remain visible and SHALL NOT be converted into a
zero score or hidden from a denominator.

The milestone SHALL NOT publish one repository-wide parity percentage. Human
and machine reports SHALL group exact capability records by relation and state
and retain each record's rationale, evidence, deviations, next issue, and
non-claims.

Implementation SHALL be organized as local-first capability trains. A train
SHOULD target 5,000-10,000 meaningful changed lines and SHALL remain one
cohesive capability or tightly coupled capability cluster with one rollback
boundary. Generated, vendored, lockfile, and evidence-artifact churn SHALL NOT
be used to satisfy the target. Work SHALL split when ownership, release or
security authority, independent rollback, or reviewability requires it.

Hosted CI SHALL remain mandatory final confirmation, but SHALL NOT be used as
the primary edit/test loop. Targeted local gates SHALL run at reviewable signed
checkpoints and the complete applicable local gate SHALL pass before the
candidate push. The delivery policy SHALL NOT weaken protected checks, merge a
red revision, or treat the preferred PR size as a waiver for evidence.

## Introduced or changed limitations

The initial matrix is a roadmap baseline, not proof that any SDK-TS route has
already switched to Rust. No mdoc capability is added. No missing engine,
binding, browser runtime, mobile runtime, or consumer receipt is inferred from
existing Rust code.

The planning/reconciliation PR itself may be smaller than the implementation
train target because it defines rather than implements the batching contract.

## Consumer and product impact

There is no runtime impact. SDK-TS remains unchanged. Future canaries gain a
clear contract for what must match, what may be adapted, what remains platform
owned, and what intentionally differs.

## Activation and rollback

Activation requires a planning-only commit and preflight receipt, machine and
human roadmap changes, validator and mutation evidence, exact-diff review,
signed/DCO commits, and protected CI. Rollback reverts documentation and
planning metadata only; no package, wire behavior, consumer, or release object
changes.

## Evidence

Issue #594, SDK-Rust slow run 40888621367, NeoPRISM PRs #324/#327/#346,
SDK-TS `main@4bf86ebf69d5e96616a148e4c973f831f95fa38e`, the existing capability
inventory, and ADRs 0162/0163/0164/0169/0170 establish the decision and its
bounded non-runtime impact.
