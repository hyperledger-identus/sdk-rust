# weekly-slow-evidence Specification

## Purpose
TBD - created by archiving change activate-weekly-slow-lane. Update Purpose after archive.
## Requirements
### Requirement: Native weekly evidence runs from protected develop

During the temporary active-development phase, protected `develop` SHALL be the
GitHub default branch and the only native source of scheduled SDK evidence.
Reserved `main` SHALL remain explicitly protected, minimal, and prohibited for
ordinary delivery, publication and release. The slow and sanitizer workflows
SHALL remain outside required pull-request checks and SHALL use only ephemeral
read-only repository credentials.

#### Scenario: GitHub evaluates the weekly schedule

- **WHEN** the native `schedule` event is accepted
- **THEN** GitHub runs the workflow version and latest commit from protected
  `develop` without a dispatcher, external token or `main` workflow shim

#### Scenario: Default branch changes

- **WHEN** repository administration changes the default from `main` to
  `develop`
- **THEN** the `main` ruleset already targets `refs/heads/main` explicitly and
  both branches retain their distinct protected roles

### Requirement: Slow run evidence is immutable and bounded

The slow workflow SHALL bind requested revision, actual checkout SHA, event,
run ID, run attempt, UTC start/end timestamps, dependency job conclusions,
retention and immutable run URL in an always-run metadata artifact. Artifact
names SHALL include revision and run attempt. Workflow concurrency SHALL bound
overlapping runs without cancelling an older running revision, and every
substantive job SHALL have an explicit timeout.

#### Scenario: Slow work succeeds or fails

- **WHEN** every substantive job reaches a terminal conclusion
- **THEN** the metadata job records the exact conclusions and revision while
  GitHub's run page remains the authoritative overall conclusion and artifact
  index

#### Scenario: New schedule arrives during an older run

- **WHEN** another slow event is accepted while the prior revision still runs
- **THEN** concurrency retains the older running evidence, bounds pending work,
  and cannot relabel one revision's result as another revision

### Requirement: Retention and liveness limitations are actionable

Repository policy SHALL record the organization-enforced seven-day artifact and
log retention ceiling. A network-explicit, read-only supervisor audit SHALL
verify that `develop` is default and classify the newest scheduled slow run as
fresh-success, running, failed, cancelled, stale or missing without printing
credentials. A weekly supervisor heartbeat SHALL invoke this audit after the
scheduled window and notify on every state except fresh success.

#### Scenario: Scheduled run is missing or unhealthy

- **WHEN** no scheduled run exists inside the declared freshness window or its
  terminal conclusion is not success
- **THEN** the audit fails closed with run identity/state only and the
  supervisor reports an actionable recovery link before evidence expires

#### Scenario: GitHub scheduling is delayed or disabled

- **WHEN** the independent heartbeat detects missing evidence
- **THEN** recovery MAY manually dispatch the same default-branch workflow but
  SHALL NOT represent that manual canary as natural scheduled evidence

### Requirement: Activation is proven in two phases

The repository change SHALL merge before protected settings are mutated. A
dated live receipt SHALL record ruleset/default state and a successful manual
exact-head canary. Issue #276 SHALL remain open until a naturally scheduled run
is observed, bound to the then-current default-branch SHA, and recorded in a
follow-up receipt.

#### Scenario: Immediate activation canary passes

- **WHEN** the merged workflow is manually dispatched after the default changes
- **THEN** it proves the native workflow path and bounded evidence contract but
  does not close the weekly-cadence acceptance item

#### Scenario: First natural schedule passes

- **WHEN** a later `schedule` run completes successfully and its exact evidence
  survives review
- **THEN** the dated receipt may close #276 without creating a release or
  support claim
