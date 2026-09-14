# ADR 0116: use crate-local declarative public-error contracts

- **Status:** Accepted for the credentials pilot
- **Date:** 2026-09-14
- **Decision authority:** issue
  [#271](https://github.com/hyperledger-identus/sdk-rust/issues/271) under the
  standing architecture-maintenance mandate
- **Applies to:** `identus-credentials` pilot only
- **Related decisions:** ADR 0110, the canonical core-error conventions and
  credential specifications
- **Assessed revision:** sdk-rust
  `707a5a22c3fad18724d5c5cac953e7387f7e49d8`

## Context

Public SDK errors have two deliberate surfaces: a crate-owned local enum and a
redaction-safe `IdentusError` projection. In `identus-credentials`, 44
`CredentialError` variants are repeated across public code constants, a large
projection match, a separate display match, and partial domain test tables.
Three `CredentialVerificationError` variants repeat the same shape at smaller
scale. The behavior is correct, but a reviewer must reconcile multiple lists
to see whether one domain is complete and unchanged.

The problem is catalogue representation, not protocol complexity. Moving all
SDK errors into one crate or derive would replace local duplication with
cross-crate coupling and a new release axis. Generating the golden from the
new implementation would also make compatibility evidence tautological.

Issue #271 requires a smallest representative pilot, exact behavioral
preservation, and an independently captured characterization contract before
any broader migration.

## Decision

1. `identus-credentials` owns a private `ErrorContract` containing exactly the
   compile-time metadata needed by its current surfaces: stable code, kind,
   capability, local display text, and public message.
2. Private catalogue modules group contract records by the cohesive domain
   invariants already present in the crate: envelope/artifact,
   metadata/schema, status, verification report, and verification execution.
   Reviewers can inspect one group without reading unrelated protocol errors.
3. Each public error enum has one private exhaustive, wildcard-free router from
   every variant to exactly one catalogue record. The compiler therefore
   rejects a newly added variant until it receives a contract.
4. The existing `Display` and `to_identus_error` implementations consume the
   selected record. `CredentialVerificationError::to_identus_error` remains
   `const`; `CredentialError::to_identus_error` remains non-const.
5. Public enum declarations, derives, `#[non_exhaustive]`, variants,
   documentation, error-code module/constants, re-exports, method signatures,
   trait implementations and constant visibility remain explicit and
   unchanged. The catalogue does not generate the public enum or constants.
6. A planning-only golden fixture captured from the assessed revision pins all
   47 current variants. It records enum/variant, constant name and visibility,
   stable code, kind, capability, local display, public message, full public
   display and error-source state. Tests compare both surfaces exactly and
   reject missing, duplicate or malformed rows.
7. The fixture is characterization evidence, not a production schema or code
   generator. Production code neither includes nor parses it, and a future
   behavior change may replace a row only through its own compatibility
   decision.
8. The pilot adds no public abstraction, crate, dependency, feature, unsafe or
   native code, serialization, FFI, retryability, structured metadata, error,
   protocol behavior or consumer migration.
9. Other crates do not automatically adopt this representation. Each later
   migration is an independently revertible issue/PR using its own golden and
   may choose a different crate-local shape if its errors carry data or
   sources. A shared runtime error crate remains rejected.

## Golden and compatibility boundary

The golden deliberately distinguishes local `Display` from the public message.
For example, `CredentialError::InvalidFormat` currently displays
`credential format is invalid` locally but projects `invalid credential
format` publicly. Refactoring those into one string would violate this
decision even though both are static and safe.

The 44 `CredentialError` constants remain public at their existing
`identus_credentials::error::error_code::*` paths. The three verification
execution code constants remain private. Both enums continue to implement
`Clone`, `Copy`, `Debug`, `Eq`, `Error`, and `Display`, have no error source,
and expose no Serde or binding contract. Existing public `IdentusError` values
remain byte-for-byte identical when rendered.

## Consequences

- Each credentials error variant has one reviewable contract record instead of
  separately maintained projection and display decisions.
- Domain grouping makes ownership and change cadence visible without creating
  public modules or new dependencies.
- Compiler exhaustiveness and a pre-refactor golden catch different classes of
  omission and drift.
- Some duplication remains intentionally: public enums/constants stay explicit
  and the golden repeats behavior as independent regression evidence.
- The pilot establishes evidence for deciding later crate-local migrations but
  does not standardize one workspace-wide macro or error taxonomy.

## Alternatives rejected or deferred

### Central shared error crate or public trait

Rejected. Domain crates must own their error taxonomies, and a new shared
runtime edge would concentrate unrelated change and release cycles.

### Generate the public enum and constants from one macro invocation

Rejected for the pilot. It reduces more text but makes public documentation,
attributes and diagnostics depend on macro expansion. Keeping public items
explicit minimizes compatibility risk while the private catalogue is proven.

### Cross-workspace derive or procedural macro

Deferred. One pilot cannot justify new macro syntax, diagnostics and expansion
policy. A later proposal needs at least two completed crate-local examples and
must still preserve crate ownership.

### `build.rs`, external schema, or post-refactor golden generation

Rejected. These add build/code-generation complexity or allow the new source
to bless its own drift. The immutable planning fixture is intentionally
captured first.

### Keep the current matches

Rejected as the target state. They pass behavior tests but retain the measured
review and duplication problem recorded by issue #271.

## Verification and rollback

Verification requires the 47-row golden test, redaction canaries, exhaustive
compile routing, exact public API comparison, current credential tests, safe
source behavior, default/minimal/all-feature checks, Clippy/docs, dependency
inventory, applicable targets, factory/Nix gates, and before/after mapping SLOC
and duplication evidence. A distinct architecture/API/security review must
find no drift.

Rollback is a focused source revert to the explicit matches. No persisted data,
wire schema, release, downstream adoption or migration is created by either
direction.
