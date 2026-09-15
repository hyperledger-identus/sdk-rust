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

Each Android slow-lane job SHALL select the Android SDK only from
`ANDROID_SDK_ROOT`, falling back to `ANDROID_HOME`, and SHALL invoke
`sdkmanager` by its executable path below that selected SDK. Missing SDK or
command-line tools SHALL fail before package installation. The workflow SHALL
retain exact Android package identifiers rather than accepting ambient default
versions.

The macOS package job SHALL install exact NDK `ndk;27.0.12077973` and platform
`platforms;android-35`, build/inspect the exact ARM64 AAR, and SHALL NOT attempt
to boot an Android VM. The dedicated Linux runtime job SHALL install the same
NDK/platform plus exact package
`system-images;android-35;default;x86_64`, require KVM, and execute only the
explicitly test-only x86_64 behavior artifact. Installation SHALL occur without
blanket SDK license acceptance.

The installed NDK package and each verifier mode's effective compiler root
SHALL agree. Ambient hosted-runner NDK aliases SHALL not override the exact
installed package. Package and runtime evidence SHALL be uploaded on success or
failure with exact SHA/run-attempt identity and seven-day retention.

#### Scenario: Hosted SDK tools are installed but absent from PATH

- **WHEN** a selected runner declares its Android SDK and does not expose a
  bare `sdkmanager` command
- **THEN** the slow lane invokes `cmdline-tools/latest/bin/sdkmanager` beneath
  that SDK and installs only the exact packages for its evidence role

#### Scenario: Declared SDK is incomplete

- **WHEN** neither SDK environment variable is present or the derived tool is
  not executable
- **THEN** the job fails before installing packages or starting native binding
  verification

#### Scenario: Google Play image has an unaccepted license

- **WHEN** the DID smoke test requires only an API-35 x86_64 Android runtime
- **THEN** the Linux lane selects the default AOSP image instead of accepting a
  Google Play or blanket outstanding SDK license

#### Scenario: ARM64 hosted runner cannot nest a VM

- **WHEN** the hosted macOS runner lacks nested virtualization
- **THEN** it still proves the exact distributable ARM64 package while the
  separate KVM-capable Linux job owns Android runtime behavior

#### Scenario: Runner default differs from the installed exact NDK

- **WHEN** a hosted image exports a newer default NDK root
- **THEN** native verification uses the exact side-by-side package installed by
  the workflow and fails if its metadata or artifact identity differs

#### Scenario: Runtime job fails after producing diagnostics

- **WHEN** package assembly, AVD boot or behavior execution fails
- **THEN** the immutable run receipt reports failure and the role-specific
  Android evidence artifact retains available logs and partial metadata
