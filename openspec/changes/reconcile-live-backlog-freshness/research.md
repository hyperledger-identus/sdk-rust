# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-15
Source retrieval date: 2026-09-15
Research blockers: none

## Problem and existing implementation

`scripts/check-ssi-upstream-backlog.py` validates the 30-row schema, ordering,
enumerations, predecessor direction, issue syntax and repository vocabulary
entirely offline. It correctly prevents program issue #20 from owning active
component delivery, but it cannot prove that a referenced issue exists or is
open. `scripts/factory doctor` therefore passes while two `in_progress` rows
reference closed children.

Live inspection through GitHub found:

- `IDR-004 -> #95`, closed after the first JWS Compact slice;
- `IDR-023 -> #250`, closed after the deferred request constructor;
- `IDR-008 -> #85` and `IDR-010 -> #91`, both closed, but their status is
  `specified`: these are durable delivered-contract evidence rather than an
  active implementation claim;
- queued and conditional rows intentionally share open program issue #20;
- delivered rows may correctly reference either a closed delivery issue or an
  open umbrella whose administrative/release remainder is outside the row.

## Normative sources

The controlling sources are issue #285, issue #20, ADRs 0003/0004, the
canonical `ssi-upstream-program` and `factory-operations` specifications, the
CSV ledger, `.factory-policy.json`, and the factory operations handbook. This
is repository-local orchestration; no third-party implementation is adopted.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Query GitHub in required fast CI | `not-adopt` | Makes deterministic validation depend on network, credentials and GitHub availability | The project adopts an explicitly networked required policy gate |
| Store live issue state in the CSV | `not-adopt` | Duplicates external state and becomes stale immediately | GitHub ceases to be the durable coordination system |
| Require every non-delivered row to reference an open issue | `not-adopt` | `specified` rows may intentionally point to the closed issue that established the reusable contract | Delivery-state semantics are changed by a separate roadmap decision |
| Add an explicit live supervisor audit for existence plus open `in_progress` ownership | `adopt` | Preserves offline CI and blocks selection of a closed active child | A purpose-built GitHub connector replaces CLI access |
| Let the audit update or close issues | `not-adopt` | Observation and mutation need separate authority and failure handling | A separately specified backlog reconciler is accepted |

## Compatibility and dependency evidence

The live command uses the existing `gh` executable and repository identity from
`.factory-policy.json`; no Rust or npm dependency is added. A strict JSON
snapshot input supports hermetic tests and incident reproduction. The default
path performs bounded read-only calls for the unique issue numbers in the
30-row ledger. It never reads tokens or prints credentials.

The offline checker remains unchanged as the fast-lane source of structural
truth. The live command is a supervisor/operator gate and is not added to
`scripts/factory check`, `bootstrap.sh --check`, or required PR CI.

## Security, privacy and maintenance evidence

The repository name comes from tracked policy and issue numbers come from the
validated CSV grammar. Calls use argv boundaries, not shell evaluation. A
missing `gh`, authentication/visibility error, malformed response, duplicate
snapshot issue or absent referenced issue fails closed with issue numbers and
states only. Issue bodies, comments, actor data and credentials are neither
requested nor retained.

## Nix deprecation evidence

The two devshell modules use the deprecated `stdenv.isDarwin` alias only as a
platform predicate. `stdenv.hostPlatform.isDarwin` is its direct supported
replacement; this does not change inputs, package selection or supported
targets.

## Rejected or deferred candidates

Automatic GitHub mutation, required-network CI, priority inference, product
roadmap changes and completion inference remain rejected or deferred. Issue
#286 owns the cryptography completion audit; issue #7 continues to own OID4VCI
until its next bounded child is created.

## Open questions and blockers

No blocker remains. GitHub unavailability is intentionally a non-zero live
audit result and does not make offline CI fail.

## Evidence commands

Planned evidence includes strict OpenSpec validation, backlog unit tests,
factory contract tests, snapshot and live audit runs, shell/file hygiene,
Nix evaluation and the normal hosted `fast` gate.
