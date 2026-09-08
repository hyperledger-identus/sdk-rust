# Constraint and limitation impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/189
Constraint blockers: none

## Existing entries affected

`SDK-SEC-001` remains the default unsafe-code prohibition and gains compiler
enforcement for direct syntax emitted by first-party procedural-macro
templates. `SDK-LIM-008` is narrowed to external macros, nested expansion
output and other generated syntax not governed by caller-spanned first-party
templates. `SDK-COMPAT-004` and `SDK-COMPAT-005` remain exact Rust 1.98.1.
`SDK-DELIVERY-001` is satisfied by issue #189 and this specification-first
change.

## Introduced or changed constraints

No new consumer prohibition or unsafe exception is introduced. Direct
first-party expansion templates must use caller-origin spans so the existing
workspace forbid remains effective in consuming SDK crates. Any future
template mechanism that cannot preserve this property requires a material
constraint decision; it cannot silently weaken the assurance.

## Introduced or changed limitations

The compiler claim remains evidence-bounded. It does not cover external
procedural macros, nested macro invocations, dependencies, arbitrary token
rewrites, unsupported builds, logical correctness or side-channel resistance.
The exact compiler behavior can change with a future Rust etalon and therefore
remains a review trigger.

## Consumer and product impact

Consumers gain stronger build-time evidence for an existing safety policy.
They do not change source, API usage, dependencies, features, target support,
wire values, runtime data or compiler version. Diagnostics from directly
generated syntax are attributed to the derived type.

## Activation and rollback

The narrowing activates only when issue #189's PR passes the existing derive,
conformance, complete compatible Nix and hosted Linux gates and merges to
`develop`. Revert that PR and restore the broader `SDK-LIM-008` if a span or
semantic regression appears; no data migration is required.

## Evidence

Evidence comprises exact Rust 1.98.1 compiler source, stable Span/quote APIs,
the empirical span matrix, the committed compile-fail fixture, all existing
finite-shape derive tests, exact-diff review, complete compatible Nix checks
and hosted required gates.
