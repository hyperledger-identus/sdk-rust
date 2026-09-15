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

Scheduled fuzz failure artifacts SHALL likewise include revision and run
attempt and SHALL use the repository's effective seven-day retention ceiling.

A manually requested rerun of a scheduled run SHALL remain recovery evidence,
not natural schedule evidence, even when GitHub retains the original `schedule`
event. The live audit SHALL require attempt one for natural cadence acceptance.

#### Scenario: Slow work succeeds or fails

- **WHEN** every substantive job reaches a terminal conclusion
- **THEN** the metadata job records the exact conclusions and revision while
  GitHub's run page remains the authoritative overall conclusion and artifact
  index

#### Scenario: New schedule arrives during an older run

- **WHEN** another slow event is accepted while the prior revision still runs
- **THEN** concurrency retains the older running evidence, bounds pending work,
  and cannot relabel one revision's result as another revision

#### Scenario: A failed scheduled run is rerun manually

- **WHEN** a later attempt succeeds while retaining the original schedule event
- **THEN** its attempt-qualified artifacts remain reviewable but the live audit
  SHALL reject it as natural schedule evidence

### Requirement: Retention and liveness limitations are actionable

Repository policy SHALL record the organization-enforced seven-day artifact and
log retention ceiling. A network-explicit, read-only supervisor audit SHALL
verify that `develop` is default, require the audited run's recorded head branch
to be `develop`, and classify the newest scheduled slow run as fresh-success,
running, failed, cancelled, stale or missing without printing credentials. A
weekly supervisor heartbeat SHALL invoke this audit after the scheduled window
and notify on every state except fresh success.

#### Scenario: Scheduled run is missing or unhealthy

- **WHEN** no scheduled run exists inside the declared freshness window or its
  terminal conclusion is not success
- **THEN** the audit fails closed with run identity/state only and the
  supervisor reports an actionable recovery link before evidence expires

#### Scenario: Historical run came from another default branch

- **WHEN** GitHub reports a scheduled run whose recorded head branch is not
  `develop`
- **THEN** the audit rejects it even when `develop` is the repository's current
  default branch

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

### Requirement: Android command-line tool discovery is explicit

The macOS slow lane SHALL select the Android SDK only from
`ANDROID_SDK_ROOT`, falling back to `ANDROID_HOME`, and SHALL invoke
`sdkmanager` by its executable path below that selected SDK. Missing SDK or
command-line tools SHALL fail before package installation. The workflow SHALL
retain exact Android package identifiers rather than accepting ambient default
versions.

The emulator dependency SHALL be exact package path
`system-images;android-35;default;arm64-v8a`, and installation SHALL occur
without blanket SDK license acceptance. Workflow installation and native
verifier package/directory identity SHALL agree or fail closed.

The installed NDK package `ndk;27.0.12077973` and the native verifier's
effective compiler root SHALL agree. Ambient hosted-runner NDK aliases SHALL
not override the exact installed package.

#### Scenario: Hosted SDK tools are installed but absent from PATH

- **WHEN** the macOS runner declares its Android SDK and does not expose a bare
  `sdkmanager` command
- **THEN** the slow lane invokes `cmdline-tools/latest/bin/sdkmanager` beneath
  that SDK and installs the exact reviewed package set

#### Scenario: Declared SDK is incomplete

- **WHEN** neither SDK environment variable is present or the derived tool is
  not executable
- **THEN** the step fails before installing packages or starting native
  binding verification

#### Scenario: Google Play image has an unaccepted license

- **WHEN** the DID smoke test requires only an API-35 ARM64 Android runtime
- **THEN** the lane selects the default AOSP image instead of accepting a Google
  Play or blanket outstanding SDK license

#### Scenario: Runner default differs from the installed exact NDK

- **WHEN** the hosted image exports a newer default NDK root
- **THEN** native verification uses the exact side-by-side package installed by
  the workflow and fails if its metadata or artifact identity differs
