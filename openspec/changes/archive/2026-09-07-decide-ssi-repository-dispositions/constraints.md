# Constraint impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/discussions/174
Constraint blockers: none

## Existing entries affected

- `SDK-ARCH-001`: chain/product dependency direction remains unchanged.
- `SDK-COMPAT-001`: supported code remains stable-only; each future adoption
  must prove compatibility with the effective toolchain policy at that time.
- `SDK-SEC-001`: unsafe Rust remains prohibited without a focused exception;
  reachable upstream unsafe code requires explicit review in an adapter issue.

## Introduced or changed constraints

No cross-cutting index value changes. The portfolio ADRs refine the
repository-specific entries under ADR 0061 without changing its general
owned-facade rule. `oracle` and `not-adopt` repositories cannot become
production dependencies without a superseding ADR; `conditional-adopt` still
requires a separate issue and evidence gate.

## Introduced or changed limitations

Repository evidence is a dated snapshot. No candidate receives an SDK target,
security, conformance, publication, or maintenance guarantee from this change.

## Consumer and product impact

Consumers see no API, artifact, build, or dependency change. Future agents gain
clear source-selection constraints and avoid repeated framework evaluation.

## Activation and rollback

The documentation decision becomes effective when its issue-linked PR merges
to `develop`. A later evidence-backed ADR can supersede one repository
decision. Reverting the PR removes the documentation only; runtime behavior is
unchanged.

## Evidence

Discussion #174 is the exact sponsor direction. ADR 0066 explicitly supersedes
ADR 0061's Spruce-specific oracle-only disposition while retaining its general
framework boundary; no dependency is added by that policy refinement. The
change pins every assessed repository revision and links its primary evidence.
Factory constraint and exact-diff checks verify that no Cargo, support-policy,
or consumer change is bundled with these decisions.
