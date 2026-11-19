# Constraints and limitations

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/608
Constraint blockers: none

## Existing entries affected

- `SDK-ARCH-001`: Rust owns semantics; target adapters depend inward.
- `SDK-COMPAT-004`: mappings and consumer-facing compatibility remain explicit.
- `SDK-LIM-002`: the package remains private and is not a stable commitment.
- `SDK-LIM-009`: binding target failures remain isolated from Rust core.

## Introduced or changed constraints

The core provider remains three methods. Peer operations require the explicit
peer capability. Browser and Node select injected or peer-WASM behavior before
initialization. Once selected, failure is terminal and never falls through to
another provider. React reuses its existing context and accepts either core or
peer-capable providers.

## Introduced or changed limitations

The correction does not activate publication, external SDK-TS adoption,
React Native, native bindings, support budgets, stable API, release, or mdoc.
It does not claim that importing an arbitrary consumer bundler entry has zero
module-loader cost; it proves the generated WASM module and initializer are not
entered for the injected selection route.

## Consumer and product impact

Existing core-only providers remain structurally assignable and peer factories
retain strong peer types. No Rust API, identifier, document, error, wire value,
generated export, external repository, product runtime, or supported target
changes.

## Activation and rollback

Activation requires planning preflight before implementation, all local target
evidence, Nix/factory gates, distinct review, signed+DCO commits, green CI, and
review. Before publication, rollback removes selection and the peer type split
while retaining the Rust engine and canonical evidence. Rollback never means
automatic runtime fallback.

## Evidence

Issue #608 supplies routine decision authority. Canonical capability, mapping,
vector, quality, target-isolation, and limitation records remain authoritative.
The planning receipt binds this contract to the exact base.
