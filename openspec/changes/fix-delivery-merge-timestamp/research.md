# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-17
Source retrieval date: 2026-09-17
Research blockers: none

## Problem and existing implementation

`scripts/factory-tools/delivery.mjs` parses the hosted `mergedAt` field through
`Date.parse` and then requires `Date#toISOString()` to equal the source. This
rejects GitHub CLI's valid second-precision UTC response because JavaScript
normalizes it by inserting `.000`.

## Normative sources

Issue #322, the live PR #321 response, RFC 3339's Internet timestamp profile,
the GitHub CLI response, and the archived issue-#320 factory contract govern
the repair. The SDK uses UTC-only receipt evidence, so numeric offsets remain
outside the accepted local grammar even when semantically equivalent.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Require only JavaScript's millisecond serialization | `not-adopt` | It rejects GitHub's live valid shape. | GitHub guarantees millisecond output. |
| Accept anything `Date.parse` accepts | `not-adopt` | Engine parsing admits shapes and offsets outside the receipt contract. | Never for security-relevant evidence. |
| Closed UTC RFC 3339 grammar plus semantic round trip | `adopt` | Accepts seconds/fractions while rejecting invalid dates and offsets. | GitHub changes the API field contract. |
| Skip post-merge timestamp validation | `not-adopt` | Weakens receipt identity and stale-state detection. | Never. |

## Compatibility and dependency evidence

The change affects only local factory evidence parsing. It adds no dependency
and changes no SDK package, target, feature, wire format, or consumer surface.
Rollback restores the stricter but non-interoperable validator.

## Security, privacy and maintenance evidence

The parser remains closed, bounded, UTC-only, and non-reflective. Tests reject
invalid dates, offsets, missing zone, excessive fraction width, and trailing
content. The repair neither prints nor retains PR body content.

## Rejected or deferred candidates

Permissive engine-only parsing, timestamp omission, tool upgrades, and broader
delivery refactoring are rejected. Recovery ergonomics for an externally
performed merge may be considered separately after this blocking repair.

## Open questions and blockers

None. The exact live timestamp and verified merge commit are available, and
the receipt recovery remains non-mutating to GitHub.

## Evidence sources

- PR #321 exact head `1ae7a53b55984d6f50184458113bd3facc376620`.
- Verified squash commit `700d581940a8f16f167c8981fe21004e5c033564`.
- Hosted `mergedAt` value `2026-09-17T09:48:40Z`.
- Issue #322 and the archived `2026-09-17-harden-factory-delivery-closeout`
  contract.

## Evidence commands

Before implementation: `gh pr view 321`, GitHub commit REST inspection, and
the guarded recovery command. After implementation: focused Node tests,
factory checks, exact receipt recovery, and hosted exact-head gates.
