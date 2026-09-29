# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

Run `36526211467`, job `109269723339`, passed `scripts/check-pr-policy.sh` and
then received `HTTP 499` from `gh issue view` in `Verify corresponding issue
exists`. Issue #470 exists at the expected repository URL, and the PR body
starts with `Closes #470`. The fast, hygiene, and DCO checks succeeded.

The current protected-base workflow invokes `gh issue view` once inline. That
command uses GitHub GraphQL and any command failure immediately fails the job;
there is no classification, retry boundary, or deterministic unit seam.

## Normative sources

Issue #474, the canonical `factory-operations` specification, protected-base
execution rules in `.github/workflows/pull-request-policy.yml`, GitHub's REST
issue representation, and the existing shell/factory contracts govern the
correction.

## Candidate decisions

| Candidate | Decision | Reason |
| --- | --- | --- |
| Ignore the required check | `not-adopt` | Removes issue-linkage assurance. |
| Manually rerun failed workflows | `not-adopt` | Treats symptoms and adds nondeterministic operator work. |
| Retry every failure | `not-adopt` | Delays permanent failures and obscures policy defects. |
| Bounded retry for classified transient failures | `adopt` | Removes provider flakiness while preserving fail-closed identity checks. |

## Compatibility and dependency evidence

The hosted runner already provides Bash, `gh`, and `jq`. No manifest, lockfile,
toolchain, target, public API, protocol, or SDK consumer changes are required.
REST issue resources expose both `html_url` and the optional `pull_request`
member needed for exact classification.

## Security, privacy and maintenance evidence

The token remains workflow-provided and is never printed. Repository and issue
identities are validated before interpolation. Provider output is held in
mode-protected temporary files and parsed as data. Retry does not convert
unknown or failed evidence into success, and the workflow continues to execute
only base-branch policy code.

## Rejected or deferred candidates

Ignoring the check, manual workflow reruns, retrying every error, adding a new
action dependency, or weakening exact URL/resource classification are rejected.
Broader general-purpose API retry infrastructure is deferred until more than
one repository-owned lookup demonstrates the same need.

## Open questions and blockers

None. The retry class is deliberately narrow and can be expanded only with new
observed evidence and a separate policy decision.

## Evidence commands

Strict OpenSpec/factory readiness, deterministic shell tests, `shellcheck`,
`actionlint`, file hygiene, and the normal exact-head hosted checks govern this
change.
