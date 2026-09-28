# ADR 0164: govern test authority, compatibility and deprecation

- **Status:** Accepted
- **Date:** 2026-09-29
- **Decision authority:** issue
  [#415](https://github.com/hyperledger-identus/sdk-rust/issues/415)
- **Related:** ADR 0162, ADR 0163
- **Constraint impact:** material; adds test provenance, change-ledger,
  deprecation, and legacy-bug compatibility requirements

## Context

Legacy SDK tests are valuable but not equally authoritative. Some reproduce
standards vectors, some define released Identus behavior, some prevent a known
consumer regression, and others merely lock one implementation. Treating every
test as a contract would import historical bugs and contradictions into Rust;
ignoring them would create accidental breaks.

Migration and release notes also fail when compatibility decisions live only
in pull-request prose. Deprecations become permanent when there is no explicit
phase or exit evidence. In rare cases, downstream software may depend on a bug,
but preserving that bug in the generic core would corrupt future behavior.

## Decision

### Test authority

Classify each relevant fixture or suite:

| Authority | Meaning | Cross-SDK compatibility effect |
|---|---|---|
| `normative` | official standards suite or canonical published vector | binding unless an explicit Identus profile narrows it |
| `identus-contract` | reviewed Identus public/wire/error fixture shared across implementations | binding for the named profile and versions |
| `consumer-regression` | reproduced released behavior needed by an identified consumer | binding within its documented compatibility window |
| `implementation-regression` | useful local invariant without public/normative evidence | quality evidence; not automatically portable |
| `exploratory` | property, fuzz, benchmark, differential, or experiment evidence | discovery evidence only |

Conflicts are resolved in that order, subject to a documented Identus profile
decision. Promotion requires provenance, immutable inputs, expected output or
error, version/profile scope, and an owner. Coverage percentage alone never
establishes authority.

### Change ledger

Record every consumer-visible change before merge as `additive`, `fix`,
`behavioral`, `deprecated`, `breaking`, or `security`. A record includes the
capability, affected packages/languages, old and new behavior, replacement,
migration action, compatibility window, first/last version, release-note class,
tests, issue/PR, and rollback. Non-breaking changes are included so migration
guides and release notes are generated from the same durable evidence.

### Deprecation phases

Use `proposed`, `announced`, `available-replacement`, `default-off`, and
`removed`. Advancement is evidence-gated, not date-only:

- proposed: disposition accepted and consumers identified;
- announced: documentation, warnings, and target versions published;
- available-replacement: supported replacement and migration guide exist;
- default-off: consumer rehearsal and rollback pass; opt-in remains if needed;
- removed: support window closed and tracked consumers migrated or explicitly
  accepted the break.

### Legacy bugs

The default is `fix-and-document`. Use `preserve-profile` only when the behavior
is an intentional supported Identus profile. Use `simulate-temporarily` only
when an identified released consumer cannot migrate atomically and the bug can
be reproduced safely through a bounded outer compatibility facade. The mode
must be explicit or version-scoped, testable, observable where feasible,
owned, warned, and assigned a removal release.

Use `reject-as-unsafe` when historical behavior weakens cryptography, trust,
input/resource bounds, secret redaction, privacy, authorization, or persistence
integrity. Such behavior fails closed and is documented as an intentional
breaking security fix; it is never simulated.

## Consequences

- Tests become traceable evidence instead of an undifferentiated parity score.
- Release notes and migration guides can be derived from one reviewed ledger.
- Deprecations have explicit exit criteria and cannot linger invisibly.
- Necessary legacy behavior is quarantined at a facade rather than infecting
  generic invariants.
- Inventory effort increases, but compatibility risk is found before consumers
  are switched.

## Alternatives rejected

- Treat all existing tests as authoritative: preserves accidents and conflicts.
- Trust coverage percentage: measures execution, not contract provenance.
- Maintain release notes manually after merge: loses changes and rationale.
- Preserve bugs indefinitely for compatibility: creates a second permanent
  semantic core.
- Remove deprecated behavior on a calendar alone: ignores replacement and
  consumer readiness.

## Verification and rollback

Migration PRs must link registry rows, authoritative vectors, change-ledger
entries, deprecation state where applicable, generated/reviewed migration text,
and rollback evidence. A compatibility simulation is tested both enabled and
disabled and cannot be the generic-core default. Reverting a migration restores
the facade route and ledger status; immutable evidence is retained.
