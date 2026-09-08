# Constraint and limitation impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/189
Constraint blockers: none

## Existing entries affected

`SDK-SEC-001` remains the default unsafe-code prohibition and gains fail-closed
validation for direct item syntax emitted by first-party procedural-macro
templates. `SDK-LIM-008` is narrowed to external macros, nested expansion
output and other generated syntax not inspected by the first-party guard.
`SDK-COMPAT-004` and `SDK-COMPAT-005` remain exact Rust 1.98.1.
`SDK-DELIVERY-001` is satisfied by issue #189 and this specification-first
change.

## Introduced or changed constraints

No unsafe exception or new consumer prohibition is introduced. Completed
direct first-party derive output must parse as Rust items and contain none of
the unsafe constructs or attributes enumerated by ADR 0088. A future direct
generator that cannot pass this check requires a material exception decision;
it cannot silently weaken the assurance.

## Introduced or changed limitations

The machine claim remains evidence-bounded. It does not cover syntax created
later by nested macro expansion, external procedural macros, dependencies,
unsupported builds, logical correctness or side-channel resistance. The
compiler/Reference unsafe inventories can change with a future Rust etalon and
therefore remain a review trigger.

## Consumer and product impact

Consumers gain stronger build-time evidence for an existing safety policy.
They do not change source, API usage, packages, target support, wire values,
runtime data or compiler version. Existing generated spans and warning behavior
remain intact.

## Activation and rollback

The narrowing activates only when issue #189's PR passes the existing derive,
conformance, complete compatible Nix and hosted Linux gates and merges to
`develop`. Revert that PR and restore the broader `SDK-LIM-008` if a syntax or
compatibility regression appears; no data migration is required.

## Evidence

Evidence comprises exact Rust 1.98.1 lint source, the Rust Reference unsafe
attribute inventory, the rejected caller-span experiment, direct syntax-guard
tests, all existing finite-shape derive tests, exact-diff review, complete
compatible Nix checks and hosted required gates.
