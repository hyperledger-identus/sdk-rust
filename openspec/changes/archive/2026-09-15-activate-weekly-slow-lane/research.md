# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-15
Source retrieval date: 2026-09-15
Research blockers: none

## Problem and existing implementation

GitHub reports `main` as the public repository's default branch and `develop`
as a separately protected integration branch. Workflow-level token defaults
are read-only. The `slow` workflow and three sanitizer workflows exist only on
`develop`, declare weekly/manual triggers, and are therefore not native hosted
schedules today. Manual historical `slow` runs prove the workflow can execute,
but do not activate its schedule.

The current implementation is therefore a complete but undiscoverable native
workflow surface plus manual CLI history, not an active hosted cadence.

Ruleset `23274352` explicitly protects `refs/heads/develop`, requires signed
pull requests, exact required checks and resolved review threads, and has no
bypass actors. Ruleset `16710155`, although named `main`, currently selects
`~DEFAULT_BRANCH`; it must be rebound to `refs/heads/main` before the default
changes or the reserved branch would lose its protection. Organization policy
limits Actions artifacts and logs to seven days.

## Normative sources

- Issue #276, ADRs 0001/0003/0004/0081, repository-settings policy and the
  2026-09-14 live settings receipt.
- GitHub documentation retrieved 2026-09-15:
  `https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows`
  states that schedules run only on the latest default-branch commit and may
  be delayed or dropped under load; it also documents the schedule actor and
  workflow notification behavior.
- `https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-branches-in-your-repository/changing-the-default-branch`
  documents that the default becomes the normal pull-request base and that the
  change requires repository administration authority.
- `https://docs.github.com/en/actions/concepts/security/github_token` and
  `https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax`
  document the per-job ephemeral repository token and explicit least-privilege
  permission model.

## Candidate decisions

| Candidate | Decision | Identity and credential | Coupling and failure behavior | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| External scheduler dispatches exact `develop` | `not-adopt` | Requires a durable external identity/token; current local OAuth is broader than dispatch-only | Adds secret rotation, host liveness and two-system recovery | Organization supplies an approved GitHub App scheduler or default-branch policy forbids `develop` |
| Make protected `develop` the default | `adopt` | Native GitHub schedule; each job gets ephemeral `contents: read`, all other scopes absent | No dispatcher or new secret; schedule and source revision share one branch | Release strategy promotes `main`, organization policy changes, or GitHub schedule reliability is insufficient |
| Minimal scheduler shim on `main` | `not-adopt` | Needs `actions: write` to dispatch another workflow | Populates reserved `main`, creates recursion/branch-drift surface and a second workflow | `main` is explicitly activated by a later release ADR |
| Keep manual/local only | `not-adopt` | No new credential | Does not meet weekly or missed-run acceptance | Only as rollback/degraded recovery |

## Revision, concurrency and evidence

GitHub native schedule binds `GITHUB_SHA` to the latest default-branch commit.
The workflow records that requested SHA and verifies `git rev-parse HEAD`
after checkout. Artifact names and the metadata manifest include the SHA and
run attempt. One workflow concurrency group allows at most one running and one
pending slow run without cancelling an older running revision. Each substantive
job has a timeout. The final metadata job runs under `always()`, records each
dependency conclusion and publishes the run URL even when a substantive job
fails.

GitHub's live run record supplies server start/end/conclusion and artifact
links. A repository checker validates the static contract offline; a separate
read-only supervisor command checks default-branch identity and the newest
scheduled run without entering required PR CI. A weekly Codex heartbeat invokes
that command after the scheduled window and reports missing, stale, running,
cancelled or failed evidence. This monitor is not the scheduler and cannot
mutate repository state.

## Failure, notification, retention and recovery

GitHub notifies the schedule actor according to their Actions notification
settings. The external read-only heartbeat is the independent missed-run
signal because GitHub documents that high-load schedules may be delayed or
dropped and public-repository schedules can be disabled after inactivity.
Artifacts and logs retain for the organization maximum of seven days. The
heartbeat runs before expiry and points to the immutable run URL; a later
governance issue may request longer organization retention.

Recovery is manual dispatch of the same default-branch workflow at the current
protected `develop` head. It cannot relabel a manual run as scheduled evidence.
Rollback returns the default to protected `main`, restores pending qualifiers,
and leaves all local/manual commands and the required `fast` lane intact.

## Security, privacy and maintenance evidence

The selected scheduler adds no secret and no write-capable workflow token.
Every job keeps explicit `contents: read`; no package, environment, OIDC,
deployment or release authority is present. The live liveness audit requests
only repository default-branch and run metadata, bounds responses and age, and
does not print token material or workflow logs. The schedule actor and the
external supervisor are maintenance dependencies recorded in the runbook.

## Compatibility and dependency evidence

No Rust, Cargo, Nix, npm or consumer dependency changes. The change affects
GitHub configuration, workflow metadata and first-party Python/shell factory
validation only. `main` remains prohibited for delivery/release under
`SDK-REPO-002`; changing the repository default does not activate it.

Exact version and feature evidence is the GitHub Actions workflow syntax and
REST behavior retrieved on 2026-09-15 plus the full action commit pins already
tracked in each workflow. License and provenance remain the repository's
Apache-2.0 first-party sources; no third-party code or fixture is copied. MSRV
and Rust 1.98.1 are unchanged. The direct dependency cone and resolved Cargo
dependency cone are unchanged because no crate or package is added.

Unsafe Rust and native-code evidence are unchanged; the scheduler introduces
neither. Supply-chain evidence remains full-SHA action pinning, read-only
workflow permissions, Cargo deny/audit in the existing slow jobs and no new
secret. Public API and wire compatibility are unchanged. The facade boundary
is repository workflow/factory tooling only, with all runtime crates below it.
No SSI protocol or draft currency decision is made by this infrastructure
change.

## Rejected or deferred candidates

An external PAT/App scheduler, `main` dispatcher workflow, release activation,
long-lived evidence storage and per-PR slow gates are rejected or deferred.
Seven-day retention is a recorded organization constraint, not a chosen SDK
quality target.

## Open questions and blockers

No implementation blocker remains. The first natural schedule cannot be
observed in this immediate slice, so issue #276 remains open after activation
until the dated run receipt is merged.

## Evidence commands

Planned evidence includes strict OpenSpec/research/constraint readiness,
support-policy mutation tests, workflow YAML/action lint, factory contracts,
ruleset/default-branch before-and-after receipts, exact-head manual dispatch,
bounded run observation and the first natural scheduled-run heartbeat.
Exact commands will be recorded in `verification.md`. Unrun checks, including
the not-yet-due natural schedule, will be named as limitations rather than
implied successes.
