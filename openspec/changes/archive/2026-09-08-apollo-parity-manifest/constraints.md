# Constraint and limitation impact

Impact class: routine
Decision status: not-required
Decision reference: not-required
Constraint blockers: none

## Existing entries affected

No effective constraint entry changes. The manifest reports the existing Rust
1.98.1 and target policy without altering it. `SDK-BOUNDARY-001`,
`SDK-COMPAT-002`, `SDK-COMPAT-004`, `SDK-COMPAT-005`, `SDK-DEP-001`,
`SDK-SEC-001` and current limitations continue to be controlled by their
existing sources.

## Introduced or changed constraints

The factory gains a repository-local evidence-integrity rule: the pinned
Apollo capability set must have unique IDs, an allowed disposition, explicit
consumer impact and self-consistent immutable evidence. This is routine
review tooling and does not activate a product, target, compatibility or
publication promise.

## Introduced or changed limitations

No cross-cutting limitation is introduced. The manifest explicitly preserves
that target compilation is not language binding/distribution parity, that
accepted differences are not missing algorithm mechanics, and that link
availability plus remote content cannot be proven by the offline checker.

## Consumer and product impact

Reviewers and agents gain one executable source for Apollo parity claims.
Oxid, Midnight Identity, NeoPRISM, Lace ID Portal and Apollo remain unchanged.
No consumer is required to adopt sdk-rust and Apollo is not deprecated.

## Activation and rollback

Activation is the ordinary issue-linked merge of the manifest and checker into
`develop` after local review and green fast CI. Rollback removes the new files
and factory wiring; it does not change cryptographic behavior. A future Apollo
baseline refresh is a focused evidence update, not an automatic rolling pin.

## Evidence

Acceptance requires canonical manifest validation, mutation tests for all
fail-closed rules, deterministic Markdown rendering, factory integration,
exact baseline/CI binding and a distinct local review.
