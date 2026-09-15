# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/276
Constraint blockers: none

The project sponsor explicitly directed execution of the selected backlog in
the current session; issue #276 is the durable material-decision record.

## Existing entries affected

- `SDK-REPO-001`: `develop` remains the active integration branch and becomes
  the GitHub default so native scheduled evidence uses the same protected head.
- `SDK-REPO-002`: `main` remains minimal and prohibited for normal delivery,
  publication and release; its ruleset changes from default-relative to an
  explicit branch reference before the default moves.
- `SDK-LIM-004`: hosted slow/fuzz cadence moves from deferred to effective,
  while release-candidate eligibility and the temporary Rust 1.98.1 review
  boundary remain unchanged.
- `SDK-AGENT-001` and `SDK-AGENT-002`: the supervisor gains a read-only weekly
  liveness check; worker and protected-action authority do not expand.

## Introduced or changed constraints

The default branch SHALL be protected `develop` while this bootstrap policy is
effective. Reserved `main` SHALL remain explicitly protected rather than
protected indirectly through a default-branch selector. Scheduled jobs SHALL
use only `contents: read`, bounded concurrency and timeouts. The organization
retention ceiling of seven days is effective external policy.

## Introduced or changed limitations

GitHub schedules are best-effort and may be delayed, dropped or disabled after
public-repository inactivity. Seven-day evidence can expire before a second
weekly run. The supervisor heartbeat detects but cannot prevent missed runs.
The immediate manual canary proves the workflow path, not natural cadence; the
issue remains open until natural scheduled evidence exists.

## Consumer and product impact

The default pull-request base becomes `develop`, matching existing SDK delivery
policy. Rust APIs, wire behavior, compiler/target claims and consumer
repositories remain unchanged. `main` gains no product or release meaning.

## Activation and rollback

After the implementation PR merges, repository administration first rewrites
ruleset `16710155` to `refs/heads/main`, verifies both rulesets, then changes
the default branch to `develop`. A manual canary runs the merged workflow.
Rollback reverses the default-branch setting and documentation without changing
the required `fast` gate or SDK code.

## Evidence

The PR carries offline workflow/policy tests and exact before-state evidence.
The live transition records ruleset/default values before and after, and GitHub
run receipts bind canary/scheduled runs to exact `develop` SHAs.
