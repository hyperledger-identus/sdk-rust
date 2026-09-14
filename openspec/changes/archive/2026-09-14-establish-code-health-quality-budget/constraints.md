# Constraint impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/270
Constraint blockers: none

## Existing entries affected

- `SDK-ARCH-002`: decomposition must preserve inward dependency direction.
- `SDK-ARCH-003`: the reusable-module gates now also govern refactoring of
  capabilities already owned by sdk-rust.
- `SDK-DELIVERY-001`: a reproducible exact-head report becomes architecture
  evidence for qualifying refactors.
- `SDK-LIM-005`: product and protocol ownership cannot be erased by sharing a
  superficially similar helper.

## Introduced or changed constraints

`SDK-ARCH-004` will make code-health signals review prompts with explicit
dispositions and a touched-semantic-scope ratchet. It will prohibit claiming
improvement through metric-only rearrangement, but it will not impose an
arbitrary line-count or complexity build failure.

This is material because it constrains future module ownership and extraction.
Issue #270 is the exact sponsor-directed outcome. The implementation remains
reversible and changes no protected setting or release promise.

## Introduced or changed limitations

- Static metrics cannot prove cohesion, correctness, security, or good design.
- Conditional compilation is classified conservatively: unknown feature or
  target predicates remain production unless `test = false` makes the whole
  expression false.
- Test-only module reachability supports ordinary Rust module layout and fails
  closed on `#[path]` overrides; it does not emulate rustc's complete module
  loader.
- Macro-expanded code is not attributed as authored production. Generated code
  is excluded only through an exact reviewed path-and-marker allowlist.
- Fast source binding requires the policy-pinned Git object; source-only Nix
  archives enforce schema/digest while weekly full-history CI regenerates the
  analyzer evidence.
- Baselines describe one immutable revision and are not a quality score or a
  requirement that every later PR update a global snapshot.

## Consumer and product impact

This repository-only evidence contract changes no SDK consumer API or runtime
behavior. Contributors gain one repeatable architecture review input. No
downstream repository, release, publication, protected setting, #7 feature, or
#168 resource-bound behavior is modified.

## Activation and rollback

The guardrail becomes effective when the issue-linked PR merges to `develop`.
Reverting the PR restores both private call sites and removes the tool/report
contract. No persisted or wire data requires migration.

## Evidence

Issue #270 is the exact durable sponsor direction. ADR 0110 and constraints
`SDK-ARCH-002`, `SDK-ARCH-003`, `SDK-DELIVERY-001`, and `SDK-LIM-005` are the
existing architecture boundary. The assessed implementation revision, tool
version, population caveat, hotspot classifications, limitations, and exact
rollback appear in `research.md` and `design.md`.
