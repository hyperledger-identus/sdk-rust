# Constraints and limitations

Impact class: routine
Decision status: not-required
Decision reference: not-required
Constraint blockers: none

## Existing entries affected

- `SDK-ARCH-001`: the credentials-owned contract remains chain- and
  product-neutral.
- `SDK-ARCH-002`: no crate or dependency edge changes; catalogues remain
  private inside the existing credential domain layer.
- `SDK-ARCH-003`: cohesion, orthogonality, public ownership and release
  independence from ADR 0110 guide the decomposition.
- `SDK-COMPAT-002`, `SDK-COMPAT-004`, and `SDK-COMPAT-005`: Rust 1.98.1 and the
  temporary compiler/target evidence policy remain unchanged.
- `SDK-SEC-001`: authored unsafe Rust remains prohibited; no exception is
  proposed.
- `SDK-SEC-002`: static redacted public errors remain the only shared error
  projection and receive stronger characterization evidence.
- `SDK-DELIVERY-001`: issue #271, OpenSpec, ADR 0116, planning receipt and a
  later distinct review remain required.
- `SDK-LIM-001`: the unpublished `0.0.0` surface has no released stability
  promise, while this refactor voluntarily holds the exact current API and
  behavior as its compatibility baseline.
- `SDK-LIM-002`, `SDK-LIM-003`, `SDK-LIM-006`, and `SDK-LIM-007`: no FFI,
  runtime target, consumer adoption, or resource-bound claim changes.

## Introduced or changed constraints

No cross-cutting constraint is introduced or changed. Within this bounded
implementation, issue #271 and ADR 0116 require a private crate-owned
declarative contract, domain-cohesive catalogues, exhaustive routing, and exact
golden compatibility. These are slice acceptance criteria, not a mandate that
other crates adopt the same internal mechanism.

## Introduced or changed limitations

The pilot does not provide a shared SDK error framework, derive macro, external
schema, public error-introspection API, retryability model, structured error
metadata, wire representation, binding surface, localization system, or
cross-crate migration. The golden captures only the 47 current fieldless
credentials variants at the exact base. Future variants and other crates
require deliberate extensions or separate slices.

The change cannot prove downstream source compatibility because no consumer
adoption is in scope. It instead enforces the exact public Rust inventory and
runtime characterization inside the SDK, which is sufficient for this
unpublished repository-local refactor.

## Consumer and product impact

No consumer action is required and no credential, verification, status,
OID4VCI, trust, wallet, chain, storage, or product behavior changes. Current
callers continue constructing and matching the same enums/constants and receive
the same local and `IdentusError` results. Consumer repositories remain
uninspected and unchanged.

## Activation and rollback

The future implementation activates only an internal credentials module layout
when merged to protected `develop`. The exact golden, public API comparison,
and existing tests are preconditions. A focused source revert restores the
hand-written matches without data, wire, release, target or consumer migration.

Any implementation that requires a shared/public abstraction, dependency
change, protocol behavior, new error, message/code change, Serde/FFI, or
resource-limit change is outside the directed outcome and stops before
activation.

## Evidence

The planning golden is bound to
`develop@707a5a22c3fad18724d5c5cac953e7387f7e49d8`. The later implementation
must compare all 47 rows, the public API inventory and error-source behavior;
run focused credentials default/minimal/all-feature tests and Clippy/docs;
measure catalogue/mapping SLOC and duplication before/after; and pass the
repository factory plus applicable native/WASM/mobile/Nix gates. Unrun commands
remain implementation evidence and are not claimed by this planning record.
