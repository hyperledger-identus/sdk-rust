# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/594
Constraint blockers: none

## Existing entries affected

The TS1 target is already an effective roadmap program, but its machine and
human records do not yet define the complete multi-runtime TypeScript outcome
or quality exit criteria. Existing supported-target claims remain unchanged.
`SDK-LIM-002` still prohibits a supported FFI claim, `SDK-LIM-003` still says
the browser WASM surface is not runtime-certified, and `SDK-LIM-009` still
preserves one required Linux `fast` signal.

## Introduced or changed constraints

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

## Introduced or changed limitations

These criteria do not activate runtime support. No whole-SDK WASM artifact,
Node.js package, React integration, React Native bridge, shared E2E inventory,
coverage baseline, performance budget, or hosted target workflow is currently
inferred.

The initial comparison descriptions summarize an immutable SDK-TS inventory
and current SDK-Rust evidence. Capability trains must update the rows whenever
implementation, profile, target, or consumer evidence changes.

## Consumer and product impact

There is no runtime impact. The roadmap becomes clearer for implementers,
reviewers, and downstream consumers by distinguishing shared semantics from
platform code and by defining what evidence is required before a route or
target is called successful.

## Activation and rollback

This documentation/checker contract activates after planning preflight,
implementation, focused and factory validation, distinct local review,
archival, signed/DCO commits, and protected integration. Runtime support is
activated only by later capability/target changes. Rollback reverts the roadmap
and checker update; it does not alter a consumer or package.

## Evidence

Issue #594 and its exact sponsor-direction comment 6699761315, the immutable
24-capability SDK-TS inventory, existing TS1 matrix, binding ADRs, and
capability-owned standards establish the bounded decision.
