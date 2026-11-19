# Constraints and limitations

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/605
Constraint blockers: none

## Existing entries affected

- `SDK-ARCH-001`: Rust remains the semantic owner and TypeScript remains a
  language adapter.
- `SDK-SEC-003`: untrusted adapter and parser inputs retain explicit bounds.
- `SDK-LIM-002`: the TypeScript package remains private and unpublished.
- `SDK-LIM-009`: isolated TypeScript evidence remains independent from Rust
  core integration evidence.

## Introduced or changed constraints

None. Evidence must identify the exact exercised boundary. Rust-core fuzz may
not satisfy TypeScript-adapter fuzz. Native differential evidence may claim
only document fields and errors actually executed by the native runner.

## Introduced or changed limitations

TypeScript-only structural preconditions such as accessor, exotic-prototype,
cycle, and host serialization rejection remain adapter evidence, not native
Rust differential evidence. The deterministic adapter fuzz route is bounded
generative evidence, not a claim of exhaustive JavaScript-engine exploration.

## Consumer and product impact

No public or wire behavior changes. The repair improves confidence and makes
existing deviations reviewable without changing any consumer contract,
supported target, dependency, external repository, or product default.

## Activation and rollback

Activation requires the issue-bound planning receipt, focused Rust and
TypeScript evidence, registry and vector validation, full local Nix/factory
gates, distinct review, signed+DCO commits, green CI, and protected merge.
Rollback removes only assertions and evidence metadata; it must not alter the
existing peer DID engine or adapter behavior.

## Evidence

Issue #605 supplies routine decision authority. The canonical donor pin,
portable packet, quality declaration, and peer adapter specification remain
the authoritative records and are updated atomically where their claims
change.
