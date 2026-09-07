# Research readiness contract

Research is a pre-implementation gate for every OpenSpec change. Its purpose is
to make an agent prove that it understands the existing implementation,
normative version, reuse choices and integration risks before it writes the
solution. It is not a request for human format approval.

Create `research.md` beside `proposal.md`. Use this shape:

```markdown
# Research readiness

Research class: protocol
Research status: draft
Decision date: 2026-09-07
Source retrieval date: 2026-09-07
Research blockers: blocked - normative final text has not been pinned

## Problem and existing implementation

...

## Normative sources

...

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| ... | ... | `adopt` | ... | ... |

## Compatibility and dependency evidence

...

## Security, privacy and maintenance evidence

...

## Rejected or deferred candidates

...

## Open questions and blockers

...

## Evidence commands

...
```

Allowed research classes are `routine`, `foundational`, `protocol`,
`cryptography-security` and `storage-ffi`. Allowed candidate decisions are
`adopt`, `conditional-adopt`, `spike`, `oracle`, `retain-local`, `not-adopt`
and `not-applicable`.

For a routine change, each section may contain a concise `not-applicable` with
the evidence that makes it inapplicable. Foundational, protocol,
cryptography/security and storage/FFI changes require a full assessment that
includes:

- pinned normative sources and retrieval date;
- current implementation and consumer evidence;
- build-versus-adopt candidates, including rejected options;
- exact version/revision, license and provenance;
- MSRV and supported target/feature behavior;
- direct and resolved dependency cone, reachable unsafe/native code and
  supply-chain evidence;
- public/wire compatibility, facade boundary and rollback;
- maintenance/release/security posture and protocol/draft currency;
- objective reconsideration triggers; and
- exact commands actually run, with unrun checks stated truthfully.

The offline checker recognizes those categories through explicit terms in the
record. Use unambiguous wording such as “current implementation”, “consumer”,
“version”, “feature”, “revision”, “license”, “provenance”, “MSRV”, “target”,
“direct and resolved dependency cone”, “unsafe”, “native”, “supply-chain”,
“public and wire compatibility”, “facade”, “rollback”, “maintenance”,
“release”, “security”, “protocol/draft currency”, “reconsideration trigger”,
“command” and “unrun”. This proves minimum shape only; links, revisions and
commands remain the semantic evidence.

Mark `Research status: ready` and `Research blockers: none` only after a fresh
semantic review clears every load-bearing uncertainty. Then run:

```bash
scripts/factory research-ready <change>
```

This gate proves presence and a minimum machine-checkable evidence shape. It
does not prove the sources correct or approve a dependency. The planner/review
agent remains accountable for semantic quality. If implementation invalidates
an assumption, return the research status to `draft`, update the OpenSpec
artifacts and rerun the gate before continuing.
