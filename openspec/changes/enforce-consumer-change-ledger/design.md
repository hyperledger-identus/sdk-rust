# Design

## Canonical registry and empty state

`docs/architecture/identus-platform-change-ledger.toml` remains the canonical
registry. Schema v2 uses a closed top level for schema/registry identity,
status, owner issue, allowed vocabularies, and `changes`. The initial
`changes = []` value is semantically meaningful: no consumer-visible change is
claimed by this governance implementation.

Each later `[[changes]]` record uses a stable capability-prefixed ID, lifecycle
state, issue and pull request, change and release-note classes, one or more
visibility dimensions, affected packages and consumers, source/target version
identity, exact old/new behavior, migration and compatibility windows,
deprecation and legacy-bug policy, observability, fallback, rollback, removal
gate, evidence selectors/artifacts, and additive lifecycle links.

The closed change classes are `additive`, `fix`, `behavioral`, `deprecated`,
`breaking`, and `security`. Visibility dimensions are `public-api`, `wire`,
`error`, `persistence`, `abi`, `target-support`, `runtime-requirement`,
`behavior`, and `security`. Release-note classes retain the existing
`added`/`changed`/`deprecated`/`removed`/`fixed`/`security` vocabulary. ADR 0164
owns deprecation and legacy-bug states.

## Cross-record graph

Every canonical record names nonempty `vector_ids` and
`quality_declaration_ids`. A language-adapter migration uses
`mapping_disposition = "required"` and nonempty `mapping_ids`; a Rust-only
record uses `mapping_disposition = "not-applicable"`, an empty mapping list,
and a reviewed rationale. The synthetic DID fixture supplies all three ID
kinds and selects the required disposition.

Resolution is local and read-only:

1. referenced records exist exactly once and are active;
2. their capability equals the change capability;
3. every referenced mapping shares at least one cited vector; and
4. every cited vector appears in at least one cited quality declaration.

The validator does not copy commands or dispositions and does not evaluate
whether a time-bounded quality receipt is fresh for promotion. That remains a
separate exact-candidate check.

## Compatibility rules

The validator uses a matrix rather than prose heuristics:

- additive/fix records cannot silently claim a breaking compatibility
  dimension; any visible behavioral delta is described explicitly;
- breaking records require nonempty migration action, replacement or an
  explicit no-replacement rationale, bounded compatibility window,
  first/last compatible versions, observability, fallback, and rollback;
- deprecated/default-off/removed transitions require the exact ADR 0164 phase
  evidence and removal gate rather than a date alone;
- security records require `reject-as-unsafe` whenever prior behavior weakens
  cryptography, trust, authorization, resource bounds, redaction, privacy, or
  persistence integrity; their rollback cannot restore unsafe behavior; and
- removed records remain present with their replacement and lifecycle links.

Exact version fields accept SemVer or the closed `unreleased` sentinel. A
record may be proposed with pull request `0`, but the merge-ready factory gate
requires the actual PR number and active lifecycle state.

## Compatibility-impact declaration

Qualifying active OpenSpec changes add `compatibility-impact.toml` with a
closed schema, owning issue, disposition, scope, rationale, and `ledger_ids`.
Allowed behavior-neutral scopes are `governance`, `documentation`,
`test-only`, `internal-refactor`, and `tooling`. A consumer-visible disposition
requires nonempty IDs resolving in the canonical ledger. A behavior-neutral
disposition requires an empty ID list and substantive rationale.

The declaration prevents accidental omission; it is not an automated semantic
oracle. Review must correct a false behavior-neutral claim. This #422 change
declares `behavior-neutral`/`governance` because it introduces evidence
infrastructure only.

Activation is forward-only. The implementation records its merge revision as
the policy activation revision. A change whose durable preimplementation
`contractHeadSha` predates that revision is grandfathered for OpenSpec-file
shape, while its eventual PR preflight must still provide the same explicit
impact, rationale, and ledger-ID decision. A later contract is never exempt.
This avoids rewriting an in-flight historical change while preventing a new
change from omitting the declaration.

## Validation and rendering

`scripts/check-platform-change-ledger.py` uses only the Python standard
library. It enforces bounded regular non-symlink inputs, closed fields and
vocabularies, stable IDs, lifecycle coherence, compatibility matrices,
cross-record resolution, safe selectors/paths, issue/PR identity, and
deterministic sort order. Stored commands or selectors are never executed.

It renders `docs/architecture/identus-platform-change-ledger.md` with sections
for release notes, migration actions, compatibility windows, rollback/removal,
and limitations. Empty input renders an explicit “no consumer-visible changes
recorded” statement in every derived view. Normal validation fails on render
drift; a separate `--write` mode updates the file.

Mutation tests independently break every critical field, class matrix,
lifecycle link, cross-record edge, impact disposition, bound, and rendered
output. A noncanonical synthetic DID fixture references:

- packet/vector IDs from `did.syntax.v1`;
- the four #505 DID value/error mapping IDs; and
- `did.syntax.quality.v1`.

The fixture is loaded only by focused tests and cannot be mistaken for the
canonical registry.

## Factory integration and stop boundary

The ledger checker joins the central factory structural check. The OpenSpec
contract validator requires `compatibility-impact.toml` for qualifying new
changes and cross-checks its ledger IDs. PR metadata carries the same decision
for every candidate, including a grandfathered in-flight change. Existing
archives remain historical and are not retrofitted.

This slice stops after repository metadata, validation, deterministic
rendering, templates, factory wiring, tests, and closeout evidence. It does not
modify Rust or consumer code, dispatch workflows, publish notes, activate a
deprecation, or authorize release/adoption.
