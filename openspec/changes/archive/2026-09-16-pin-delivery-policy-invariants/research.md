# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-17
Source retrieval date: 2026-09-17
Research blockers: none

## Problem and existing implementation

ADR 0127, the canonical factory specification, and `.factory-policy.json`
already select the three ordered promotion blockers and the 12-file/1,000-line
decomposition guidance. `scripts/ci/target-plan.mjs` instead checks only that
`release-preparation` is present and that both thresholds are positive. This
permits policy mutation that changes accepted behavior while validation still
succeeds.

## Normative sources

Project direction in issue #305 and the accepted values in ADR 0127 control
this follow-up. The canonical `factory-operations` specification,
`.factory-policy.json`, the target-plan validator, and its focused tests are
the implementation evidence.

## Candidate decisions

Review threads on PR #304 identified both gaps. Direct inspection of the
policy, target-plan validator, and focused Node tests confirms that no external
crate, service, or protocol research is needed.

| Candidate | Decision | Reason |
| --- | --- | --- |
| Exact ordered equality | `adopt` | Matches the accepted machine policy and fails on omission, insertion, or reorder. |
| Membership-only validation | `reject` | Cannot prove the complete promotion-blocking contract. |
| Positive-only thresholds | `reject` | Allows silent weakening or strengthening of accepted guidance. |
| Schema/dependency addition | `not-adopt` | Two closed local comparisons and mutation tests are sufficient. |

## Compatibility and dependency evidence

There is no public or wire compatibility impact and no Cargo, Nix, action,
feature, target, MSRV, facade, license, provenance, or resolved dependency-cone
change. Existing target-plan output remains byte-for-byte compatible for the
canonical policy.

## Security, privacy and maintenance evidence

The change is repository tooling only. It adds no dependency, unsafe code,
runtime network access, credential handling, public API, wire format, target,
compiler, or supply-chain input. Exact equality is deliberately maintenance
sensitive: any future policy change must update the accepted specification and
tests in the same issue-first slice.

## Rejected or deferred candidates

Membership-only blocker validation and positive-only thresholds are rejected
because they permit semantic drift. A JSON Schema or new validation dependency
is unnecessary for this closed local object and is not adopted. No candidate
is deferred.

## Open questions and blockers

No blocker remains. The exact values are already accepted; this slice only
makes their validation complete.

## Evidence sources

- Issue #305 and the two linked review threads from PR #304.
- ADR 0127, `.factory-policy.json`, canonical `factory-operations`,
  `scripts/ci/target-plan.mjs`, and `scripts/tests/factory-operations.mjs` at
  `develop@4c740fbbd6736b36534e0d9dfe73f0f8d45351ff`.
- Unrun before implementation: focused mutation tests and the repository
  health check. They are required after the planning receipt.

## Evidence commands

Commands run: repository search over policy, validator, tests, ADR 0127 and the
canonical specification; issue and PR review inspection; research and
constraint readiness. Unrun before implementation: focused Node tests and
`./bootstrap.sh --check`.
