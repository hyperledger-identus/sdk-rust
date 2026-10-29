# Constraints and limitations

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/578
Constraint blockers: none

## Existing entries affected

The factory preimplementation receipt, protected squash delivery path, and
immutable error-golden mechanism are affected. Public SDK constraints,
supported targets, dependencies, errors, wire formats and releases are not.

## Introduced or changed constraints

Every immutable golden binding must pin the complete preimplementation receipt
bytes. Active and ancestry-preserving evidence must retain the exact
base-to-contract-to-head and planning-only diff proof. An archived contract
integrated by squash may use canonical snapshot evidence only when its fixed
base is in current ancestry, receipt bytes are exact, both golden copies are
exact, all three files resolve byte-for-byte from `HEAD`, and any locally
reachable named contract object validates completely.

Feature-branch retention, GitHub availability and private merge receipts are
not correctness dependencies.

## Introduced or changed limitations

Canonical snapshot evidence proves the exact reviewed receipt and fixture that
survived integration; after the source commit becomes unreachable it cannot
reconstruct Git parentage discarded by squash. It therefore never claims that
the squash commit itself was planning-only. The fixed receipt hash and golden
hash are the durable canonical evidence.

## Consumer and product impact

None. This is behavior-neutral factory tooling. It restores repository health
for maintainers and agents without changing SDK behavior or consumer code.

## Activation and rollback

Activation requires issue #578 preflight, ADR 0196, mutation coverage, factory
and Nix gates, independent CLI review, and green protected CI. Rollback restores
the invalid ancestry-only assumption and would make protected squash
integration red; no SDK migration is involved.

## Evidence

Issue #578, ADR 0196, PR #577 metadata, exact Git identities, five immutable
receipt hashes, checker mutation tests, factory checks, independent review, and
protected CI.
