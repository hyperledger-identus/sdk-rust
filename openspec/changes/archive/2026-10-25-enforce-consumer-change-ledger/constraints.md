# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/422
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001` and `SDK-LIM-005` keep the ledger generic and prevent consumer
or product policy from entering SDK-Rust. `SDK-ARCH-002` prevents a tooling
record from creating a production dependency edge. `SDK-DELIVERY-001` supplies
the issue-linked, spec-driven factory boundary. `SDK-LIM-006` keeps downstream
adoption separate. `SDK-LIM-009` keeps promotion freshness and slow evidence
outside each active-development PR. None is weakened.

## Introduced or changed constraints

Every qualifying OpenSpec change will declare exactly one compatibility impact:
`consumer-visible` or `behavior-neutral`. Consumer-visible work must reference
one or more validated schema-v2 ledger records in the same candidate.
Behavior-neutral work must reference no ledger record and must state a closed
scope plus substantive rationale. The factory rejects an omitted, unknown, or
internally contradictory disposition; it does not infer compatibility.

Every canonical change record has a stable ID and explicit capability, class,
visibility dimensions, affected packages/consumers, old/new behavior,
compatibility and version window, migration, release-note class, deprecation
and legacy-bug policy, observability, fallback, rollback, removal gate,
lifecycle, issue/PR, and exact vector/quality evidence. Language-adapter
migrations require exact mapping evidence; a Rust-only record requires a
reviewed not-applicable mapping disposition. Cross-record references resolve
locally and capability-coherently.

Breaking, deprecated, removed, persistence-changing, wire-changing,
runtime-requirement, and security records have additional fail-closed evidence
requirements. Historical records remain available after supersession or
removal. Renderer output is deterministic derived evidence, not an independent
authority.

## Introduced or changed limitations

This contract cannot prove that a human or agent classified a semantic change
honestly; local review and later consumer evidence remain required. It does not
evaluate current quality freshness, execute stored commands, inspect a remote
consumer, select a version, publish release notes, activate deprecation, or
authorize a release. The canonical ledger remains empty until a real
consumer-visible change exists.

The first synthetic DID record is test data only. It proves resolution against
the #420 vector catalog, #505 adapter mappings, and #501 quality declaration;
it is never rendered as shipped behavior or counted as a migration.

## Consumer and product impact

No SDK or product behavior changes. No language DTO or error shape constrains
Rust. Future consumer migrations gain explicit, reviewable migration and
rollback evidence, but #492 retains ownership of any SDK-TS code or default
switch.

## Activation and rollback

Activation requires the signed/DCO planning commit and exact preflight receipt,
schema-v2 empty registry, strict validator and mutation suite, deterministic
renderer, compatibility-impact gate and template, synthetic DID fixture,
factory integration, focused evidence, distinct review, and green protected
CI. Before merge, rollback removes the repository-local change. After stable
IDs merge, corrections use a schema/registry version increment and explicit
lifecycle links rather than rewriting history.

## Evidence

Issue #422, parent #504, ADRs 0164 and 0173, the A1 contract blueprint, and the
delivered #420/#505/#501 registries provide the decision authority and exact
cross-record inputs.
