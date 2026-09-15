# ADR 0120: run weekly evidence from protected `develop`

- **Status:** Accepted by project-sponsor direction
- **Date:** 2026-09-15
- **Issue:** [#276](https://github.com/hyperledger-identus/sdk-rust/issues/276)
- **Supersedes operationally:** the inactive-scheduler status notes attached to ADR 0081
- **Review no later than:** 2026-12-08 and before any release candidate

## Context

GitHub evaluates scheduled workflows only from the repository's default branch.
The SDK's implementation and reviewed workflows live on protected `develop`,
while `main` is intentionally minimal. Keeping `main` as the default therefore
made the accepted weekly cadence descriptive rather than executable. External
orchestration would require another credential, scheduler and audit boundary.

Changing the default branch is a repository-administration action. The existing
`main` ruleset used default-branch indirection, so it must first be rebound to
the explicit `refs/heads/main` target. Otherwise changing the default could
weaken `main` protection and accidentally overlap the `develop` ruleset.

## Decision

1. Rebind the reserved-branch ruleset to explicit `refs/heads/main`, preserving
   its enforcement, bypass actors and rules exactly.
2. After this ADR and workflow merge through protected `develop`, select
   `develop` as the GitHub default branch. Keep its existing explicit ruleset,
   required pull request, DCO, signed-commit, required-check and resolved-thread
   protections.
3. Use GitHub's native weekly schedule and `workflow_dispatch`; do not add a
   personal token, GitHub App, default-branch shim or external scheduler.
4. Grant the workflow only `contents: read`. Pin third-party actions to full
   revisions. Give every job a timeout.
5. Serialize each workflow/ref pair as one running plus one pending run and do
   not cancel an in-flight evidence run.
6. Bind every receipt to `GITHUB_SHA` and the actual checkout SHA. Record event,
   run and attempt identity, UTC start/end, per-job result, run URL and the
   seven-day organization retention ceiling. Qualify every evidence artifact by
   SHA and attempt so reruns cannot collide.
7. Add `scripts/factory slow-live` as a read-only audit of the latest scheduled
   run. The audit requires both the current default branch and the run's
   recorded head branch to be `develop`. Missing, foreign-branch,
   non-successful or older-than-160-hour evidence fails closed and identifies
   the run without echoing command stderr. The threshold leaves an eight-hour
   alert margin before the seven-day retention ceiling.
8. Merge the activation PR with `Refs #276`, then apply and verify the repository
   settings and run one manual canary. Keep #276 open until the first successful
   natural scheduled run is attached as evidence.
9. Treat only attempt one as natural schedule evidence. A successful manual
   rerun retains operational value but cannot satisfy the first-schedule gate.

## Consequences

- The repository's UI and new pull requests default to `develop`, matching the
  actual integration branch. Contributors must still use issue-linked feature
  branches and protected pull requests.
- `main` remains minimal and protected by its explicit target; this decision
  does not populate, release from or promote to it.
- A manual canary proves dispatch and workflow correctness but cannot be called
  natural schedule evidence.
- Scheduled runs can be delayed or omitted by the platform. The freshness audit
  turns that condition into visible operational debt; it does not self-dispatch
  or mutate repository state.
- Seven days is sufficient for weekly freshness and is the current organization
  maximum, but it is not a durable release archive. Release evidence requires a
  separate accepted retention and attestation decision.

## Rollback

Pause the slow and fuzz workflow schedules, restore `main` as the default branch
only after verifying its explicit ruleset, set the machine policy to inactive,
and record the incident in a superseding ADR. Do not delete historical run or
receipt references. A rollback does not relax `develop` protection or authorize
release.
