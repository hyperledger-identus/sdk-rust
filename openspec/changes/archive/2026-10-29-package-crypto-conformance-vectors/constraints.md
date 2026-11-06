# Constraints and limitations

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/586
Constraint blockers: none

## Existing entries affected

Package name, version, APIs, algorithms, provenance, digest, publication
authority, Rust support policy, and protected release process remain unchanged.
The candidate and source-distribution specifications govern this repair.

## Introduced or changed constraints

- A test input required by a publishable crate's declared targets is owned
  below that crate root and included in Cargo's package listing.
- The crypto packet has one mutable authority. Catalog, quality, Nix, mutation,
  and Rust evidence reference that same regular file.
- Candidate success requires testing the extracted archive without a
  repository-relative fallback.
- Vector bytes, digest, provenance, semantics, and expected results stay fixed.

## Introduced or changed limitations

This does not claim registry publication, alter crypto behavior, add a runtime
resource API, change versions, or authorize release. The red slow run remains
immutable evidence; a fresh run is allowed only after a protected fix merge.

## Consumer and product impact

Consumers receive the same code, API, behavior, and version. A future archive
gains the public fixture needed to test itself. Maintainers gain honest isolated
evidence; release authority and downstream adoption remain unchanged.

## Activation and rollback

Activation requires issue-bound preflight, package-list and extracted-test
regressions, focused and full evidence, independent CLI review, green protected
CI, and guarded merge. Rollback requires another self-contained ownership model;
otherwise it reintroduces the known release blocker.

## Evidence

The exact red protected run, unchanged SHA-256, catalog, package listing,
extracted test, complete candidate receipt, factory and Nix gates, independent
review, protected CI, guarded merge SHA, and subsequent fresh slow run form the
evidence chain.
