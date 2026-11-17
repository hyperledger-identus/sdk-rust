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

## Success-criteria constraint addendum

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/594
Constraint blockers: none

### Existing entries affected

The TS1 target is already an effective roadmap program, but its machine and
human records do not yet define the complete multi-runtime TypeScript outcome
or quality exit criteria. Existing supported-target claims remain unchanged.
`SDK-LIM-002` still prohibits a supported FFI claim, `SDK-LIM-003` still says
the browser WASM surface is not runtime-certified, and `SDK-LIM-009` still
preserves one required Linux `fast` signal.

### Introduced or changed constraints

TS1 SHALL maintain a side-by-side feature record for every normalized SDK-TS
capability. Current Rust behavior, canonical-inventory TypeScript behavior,
target boundary, known deviation, missing proof, and evidence state SHALL remain
separate facts.

The target TypeScript SDK SHALL use a thin standards-oriented facade over the
portable Rust core and SHALL NOT expose the historical component names as new
public exports, package paths, or primary documentation categories.

Browser, Node.js, React web, and React Native SHALL each have target-specific
build, load, package, E2E, and rollback evidence before support is claimed.
React Native SHALL NOT be inferred from browser WASM evidence; it may use a
dedicated New Architecture/TurboModule package only after its native Rust
binding adapter is qualified under ADR 0163.

CI SHALL run the Rust-core job before isolated binding/runtime target jobs. A
fail-closed exact-head impact plan SHALL select target jobs from exported
contract/API fingerprints, transitive bound crates, lockfiles, generated
mappings, packages, target contracts, and changed paths. A target failure SHALL
NOT cancel, overwrite, or invalidate the completed Rust-core job result or
sibling target results. The existing single required `fast` signal SHALL fail
when a selected target fails and SHALL remain the protected integration verdict
under `SDK-LIM-009`; changing that policy requires a measured CI-policy
decision. Core-only changes SHALL NOT install or depend on TypeScript tools when
the impact plan proves every target inapplicable.

Portable core crates and the `core-rust` CI package set SHALL have no inward
dependency on WASM, native binding, JavaScript, TypeScript, npm, React, or
platform package code. Binding and adapter packages SHALL depend inward on the
portable core only. Unknown impact SHALL select all dependent target jobs.

Applicable SDK-TS E2E journeys SHALL run against the Rust-backed route with the
same TypeScript assertions. Exclusions and adaptations SHALL be explicit and
reviewable. Additional or intentionally different portable behavior SHALL be
covered by Rust-owned E2E/conformance evidence and selected TypeScript boundary
tests.

Owned changed executable code SHALL have reproducible measured coverage, strict type-safety,
mapping-drift, reusable-core, target budget, diagnostics, observability,
fallback, and rollback evidence according to the machine success contract.

### Introduced or changed limitations

These criteria do not activate runtime support. No whole-SDK WASM artifact,
Node.js package, React integration, React Native bridge, shared E2E inventory,
coverage baseline, performance budget, or hosted target workflow is currently
inferred.

The initial comparison descriptions summarize an immutable SDK-TS inventory
and current SDK-Rust evidence. Capability trains must update the rows whenever
implementation, profile, target, or consumer evidence changes.

### Consumer and product impact

There is no runtime impact. The roadmap becomes clearer for implementers,
reviewers, and downstream consumers by distinguishing shared semantics from
platform code and by defining what evidence is required before a route or
target is called successful.

### Activation and rollback

This documentation/checker contract activates after planning preflight,
implementation, focused and factory validation, distinct local review,
archival, signed/DCO commits, and protected integration. Runtime support is
activated only by later capability/target changes. Rollback reverts the roadmap
and checker update; it does not alter a consumer or package.

### Evidence

Issue #594 and its exact sponsor-direction comment 6699761315, the immutable
24-capability SDK-TS inventory, existing TS1 matrix, binding ADRs, and
capability-owned standards establish the bounded decision.
