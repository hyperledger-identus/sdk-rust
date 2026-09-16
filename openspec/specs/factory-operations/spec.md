# factory-operations Specification

## Purpose
TBD - created by archiving change operationalize-ai-software-factory. Update Purpose after archive.
## Requirements
### Requirement: Dev-loop proves OpenSpec readiness before implementation

A production-ready dev-loop SHALL bind one repository issue, one issue branch
and one active OpenSpec change to an exact `develop` base before changing
implementation paths. It SHALL pass strict change validation, research
readiness and constraint readiness and SHALL persist a pre-implementation
receipt that final readiness revalidates.

#### Scenario: Ready contract begins implementation

- **WHEN** the clean issue branch differs from its exact base only through the
  issue's planning/specification paths and every readiness gate passes
- **THEN** preflight writes an issue, branch, base, contract-head and change
  receipt and implementation may begin

#### Scenario: Implementation precedes contract readiness

- **WHEN** preflight observes an implementation path changed before a valid
  receipt exists or a research/constraint blocker remains
- **THEN** it fails without claiming the dev-loop is implementation-ready

### Requirement: One pinned bootstrap controls the worker runtime

The repository SHALL provide one Nix-backed bootstrap entrypoint that can
check the factory, audit the effective Pi runtime, configure repository-owned
Git hooks, apply the bounded user sub-agent policy only with an explicit
execute action, and launch Pi after its configuration audit passes. It SHALL
not read or replace provider authentication.

The bootstrap health check SHALL fail unless structural factory checks, the
effective pinned runtime audit and the operational test suite all pass in the
same pinned environment.

#### Scenario: Operator launches Pi

- **WHEN** the tracked runtime, package declarations, policies and local user
  sub-agent bounds are aligned
- **THEN** `./bootstrap.sh --pi` launches the Nix-supplied Pi executable

#### Scenario: Runtime audit finds drift

- **WHEN** a required contract, version, budget, hook or capacity invariant is
  missing or inconsistent
- **THEN** audit exits non-zero with a remediation and Pi is not launched

#### Scenario: Operator runs the bootstrap health check

- **WHEN** the operator invokes `./bootstrap.sh --check`
- **THEN** structural factory checks, effective pinned runtime audit and
  operational tests run in order, and any failed stage makes the command fail

### Requirement: Factory policy is machine-readable and repository-specific

Tracked contracts SHALL define delivery profiles, issue/branch/commit grammar,
bounded review and sub-agent budgets, worktree capacity, target routing,
metrics privacy and SDK-specific merge authority. Vault or donor guidance
SHALL NOT silently replace an accepted SDK version or governance decision.

#### Scenario: Oxid guidance conflicts with SDK policy

- **WHEN** the reference factory uses milestone trains or human-only
  `develop` merges
- **THEN** the SDK contracts retain direct issue PRs to `develop` and the
  delegated exact-head green merge authority from ADRs 0003/0004

### Requirement: Contribution provenance has local and hosted enforcement

The factory SHALL validate issue branch grammar, Conventional Commit scope,
DCO and OpenPGP provenance, breaking-change footers and staged secret safety
through repository-owned local hooks. Mandatory invariants SHALL also run in
hosted CI because local hooks are bypassable.

#### Scenario: Invalid outgoing commit is pushed

- **WHEN** an authored outgoing commit lacks the required signature, DCO or
  valid subject, or its branch lacks an issue identity
- **THEN** pre-push fails before network mutation and names the violated rule

### Requirement: Managed worktrees have single ownership and safe lifecycle

The factory SHALL use one canonical managed path per issue and SHALL audit
capacity before creation. Cleanup SHALL require an exact path and expected
head and SHALL refuse primary, current, dirty, locked, broad, symlinked or
ambiguously delivered targets.

#### Scenario: Exact merged worktree is closed

- **WHEN** the named PR's remote head, merged state, base and expected local
  head match a clean unlocked canonical managed worktree
- **THEN** an explicit execute action may remove that one worktree

#### Scenario: Cleanup target is ambiguous

- **WHEN** any ownership, path, head, cleanliness, lock or merge proof is
  missing
- **THEN** cleanup fails without deleting files or refs

### Requirement: CI routing is immutable and keeps the fast and slow model

The factory SHALL derive one target plan from an exact base/head diff and
delivery profile. The active-development PR requirement SHALL remain the
single Linux `fast` gate; active hosted weekly/manual slow evidence or its exact
local reproduction SHALL be recommended for toolchain, Nix, security, FFI and
release-sensitive paths. Unknown diff state SHALL fail closed to the complete
public evidence recommendation.

The target plan SHALL distinguish integration readiness from production-
promotion readiness. Fast integration SHALL retain policy, OpenSpec,
formatting, normal workspace build, strict Clippy, tests, and bounded
first-party analysis in the pinned Linux environment. Its initial execution
SLO SHALL be p50 at most six minutes and p95 at most eight minutes; a rolling
p95 above ten minutes SHALL trigger focused optimization without silently
removing evidence. Slow promotion SHALL retain complete exact-revision
platform, target, binding, security, conformance, coverage, performance,
fuzz/sanitizer, packaging, and receipt evidence as applicable. A slow failure
SHALL block production promotion, publication, and release preparation but
SHALL NOT add another required ordinary PR build or retroactively invalidate
an unrelated green integration.

#### Scenario: Factory-only pull request is planned

- **WHEN** an exact diff changes factory scripts, contracts or CI
- **THEN** the plan requires `fast`, records the factory area and recommends
  the complete slow backstop without creating three per-PR Rust builds

#### Scenario: Ordinary active-development slice is planned

- **WHEN** a bounded exact diff targets protected `develop`
- **THEN** integration readiness requires the single `fast` status while
  production-promotion readiness remains a separate exact-SHA decision

#### Scenario: Fast latency drifts beyond its ceiling

- **WHEN** a rolling comparable sample reports p95 execution above ten minutes
- **THEN** the factory creates or selects a focused optimization issue and does
  not remove correctness evidence without an accepted replacement

#### Scenario: Candidate is promoted toward production

- **WHEN** publication, release preparation, or a production-support claim is
  requested
- **THEN** the exact unchanged candidate SHA requires a green complete slow
  receipt and no unresolved release-blocking finding

### Requirement: Delivery metrics are private, bounded and exact-head

One authoritative metrics record SHALL be keyed by repository, issue,
optional PR and exact head in a private Git-common-dir store. Public output
SHALL contain allowlisted aggregates and exactly one bounded hidden payload,
and publication SHALL update only the authenticated publisher's matching
comment. Unknown counters SHALL be `null`, never estimated as zero.

The factory SHALL continue to validate and render closed version 1 records.
New version 2 records SHALL additionally distinguish total and phase duration,
CI queue and execution duration, failed/canceled/retry and post-CI-push counts,
sessions, turns, tools, non-overlapping input/output/cache-read/cache-write
tokens, and separate worktree/target/cache peaks. Every unavailable v2 value
SHALL have an enum-bounded reason.

Every completed production-ready work item SHALL retain or confirm its exact
private record before attempting one public allowlisted receipt. Publication
SHALL default to the record's exact PR and SHALL fall back to its issue when no
PR exists. An explicit issue target MAY support issue-centric or historical
reporting. Historical PR-backed publication SHALL verify repository, issue,
the PR's authoritative closing reference to that issue, and exact hosted head
without requiring the caller's checkout to remain on that head. The bounded
public payload SHALL be rendered and size-checked before immutable local
retention. A record whose outcome remains `in-progress` SHALL NOT enter that
immutable store or publication flow. One valid draft retained by the previous
writer MAY transition atomically to matching terminal evidence; terminal
evidence SHALL remain immutable, and concurrent transition attempts SHALL
permit at most one winner. One bounded retry MAY handle a transient comment
failure; persistent failure SHALL remain visible telemetry debt and SHALL NOT
invalidate otherwise independent product evidence.

#### Scenario: Valid exact-head closeout is rendered

- **WHEN** a schema-valid private record matches the current repository,
  issue, PR and head
- **THEN** the renderer emits a bounded summary without prompts, transcripts,
  credentials, raw command output or provider billing data

#### Scenario: Metrics payload is untrusted

- **WHEN** a payload is malformed, oversized, duplicated, stale, forged or
  contains a forbidden field
- **THEN** collection/publication fails closed while product evidence remains
  independently reportable

#### Scenario: Historical version 1 evidence is read

- **WHEN** an existing closed version 1 record is validated or rendered after
  version 2 activates
- **THEN** the factory preserves its original semantics and versioned marker
  without inventing version 2 fields

#### Scenario: Version 2 counter is unavailable

- **WHEN** a phase, CI, runtime or resource value cannot be measured exactly
- **THEN** its value is `null` and the record contains one closed reason for
  that field rather than zero or an estimate

#### Scenario: Completed PR metrics are published

- **WHEN** a schema-valid record names an authoritative PR whose hosted head
  equals the record head and whose closing references include the recorded
  issue in the authoritative repository
- **THEN** the factory atomically retains or confirms the private record before
  creating or updating the publisher's unique versioned comment on that PR

#### Scenario: Issue receipt is selected

- **WHEN** a valid record has no PR or the supervisor explicitly selects its
  existing authoritative issue
- **THEN** the same local-before-remote and unique-marker rules publish the
  bounded receipt to that issue

#### Scenario: Historical or local evidence conflicts

- **WHEN** the hosted PR head differs, the PR does not close the recorded issue,
  the issue/PR is unavailable, a private issue/head record differs, the public
  payload exceeds its bound, or multiple owned target comments share a marker
- **THEN** publication fails before an ambiguous or forged public receipt is
  created

#### Scenario: Draft metric is retained

- **WHEN** an otherwise schema-valid record still has the `in-progress` outcome
- **THEN** validation and rendering remain available but immutable retention
  and publication fail before creating terminal evidence

#### Scenario: Legacy draft precedes terminal evidence

- **WHEN** the former writer retained a valid `in-progress` record whose stable
  schema, repository, issue, head, profile and start identity match a closed
  record
- **THEN** the factory atomically replaces that draft once with the terminal
  evidence, rejects a competing transition without overwrite, and applies
  ordinary immutability thereafter

#### Scenario: Public mutation remains unavailable

- **WHEN** comment creation or update still fails after one bounded retry
- **THEN** the private record remains authoritative and the supervisor reports
  visible telemetry debt without treating independent product evidence as red

### Requirement: Harness tuning follows measured canary evidence

The factory SHALL run one bounded issue-backed SDK slice through the pinned Pi
shell after the factory change merges to `develop`. Runtime, retry, tool, token,
CI and disk evidence SHALL inform a separate focused tuning issue; unavailable
measurements SHALL remain explicit and the canary SHALL NOT activate `main`.

#### Scenario: Canary reveals friction

- **WHEN** the first Pi-driven slice records a reproducible runtime, package,
  review, CI or resource problem
- **THEN** the supervisor files or selects one bounded harness issue rather
  than performing an unreviewed general upgrade

### Requirement: Pi project packages use an external content-addressed cache

The bootstrap SHALL keep Pi project-package state out of every registered Git
working tree. It SHALL select a repository-private cache from the exact pinned
runtime, project package declarations and resolved package lock, verify
completeness before use and preserve Pi's expected project path without reading
or moving authentication, session, prompt, transcript, provider or model state.

#### Scenario: Two worktrees use the same exact harness

- **WHEN** two trusted managed worktrees launch the same pinned Pi, Node, npm
  and exact project package set
- **THEN** each `.pi/npm` resolves to the same complete external cache identity
  without retaining a duplicate package installation in either worktree

#### Scenario: Harness inputs change

- **WHEN** any pinned runtime or exact project package declaration differs
- **THEN** bootstrap selects a distinct cache identity and does not mutate the
  cache created for the previous inputs

#### Scenario: Worktree package path is unsafe or operator-owned

- **WHEN** `.pi/npm` is a regular directory, an unexpected symlink, or the
  derived store fails canonical path and non-symlink checks
- **THEN** bootstrap exits before Pi starts and does not replace, move, delete
  or follow that data

#### Scenario: Raw Pi bypasses bootstrap

- **WHEN** Pi creates `.pi/npm` directly in a repository working tree
- **THEN** the root ignore fallback prevents generated packages from dirtying
  Git while the next bootstrap reports explicit non-destructive recovery

#### Scenario: Concurrent first initialization

- **WHEN** two worktrees prepare one previously absent cache identity
- **THEN** isolated staging and atomic promotion expose only a verified complete
  canonical store, and incomplete staging is never selected by a Pi launch

### Requirement: Supervisor-to-Pi runs are exact, bounded and observable

The factory SHALL prepare a closed invocation envelope that binds repository,
issue, profile, exact `origin/develop` base, branch/head, active OpenSpec
receipt, worker role, task, allowed paths, tool surface, deadline and
post-artifact grace. It SHALL launch the worker only through `./bootstrap.sh
--pi`, emit an external content-free heartbeat, bound the process group, and
keep merge, release, publication, CI and cleanup authority with the supervisor.

#### Scenario: Exact ready work item launches

- **WHEN** the envelope matches the current issue worktree and durable
  preflight receipt
- **THEN** the supervisor launches one Pi worker with the declared tools and
  private run/session locations and independently updates liveness

#### Scenario: Invocation identity is stale

- **WHEN** repository, issue, base, branch, head, change or receipt differs
  from the current worktree
- **THEN** launch fails before Pi starts and does not claim work completion

#### Scenario: Worker exceeds its deadline

- **WHEN** the Pi process remains active past the declared hard deadline
- **THEN** the supervisor terminates its process group after bounded grace and
  records a timed-out terminal heartbeat separately from handoff validity

### Requirement: Worker handoff is machine-readable and effect-checked

The worker SHALL write one closed, byte-bounded terminal handoff with exact
identity and status, changed paths, acceptance evidence, check outcomes and
durations, finding dispositions, process ownership and a closed safe next
action. The supervisor SHALL validate that record and current Git effects
before accepting it.

#### Scenario: Valid worker checkpoint is accepted

- **WHEN** the handoff identity matches the envelope/current head, changed
  paths are allowlisted, checks and findings are structurally valid and no
  worker-owned process remains
- **THEN** the supervisor reports an accepted checkpoint without granting the
  worker CI, merge, release, publication or cleanup authority

#### Scenario: Handoff is malformed or out of scope

- **WHEN** the handoff is absent, oversized, symlinked, stale, malformed,
  declares an unapproved path/task or proposes an unsafe next action
- **THEN** the supervisor records handoff failure distinctly from product-code
  outcome and refuses closeout acceptance

### Requirement: Persisted Pi usage is harvested without session content

The factory SHALL harvest exact session, turn, tool-call and non-overlapping
token counters from supported private persisted Pi session data. It SHALL NOT
retain or emit prompts, message content, identifiers, commands, raw output,
provider/model data, cost, credentials or billing data. Unsupported or unsafe
input SHALL produce explicit unavailable reasons, never guessed counters.

#### Scenario: Supported Pi v3 session is harvested

- **WHEN** one bounded regular Pi version 3 session and its content-free event
  stream contain terminal assistant usage and tool lifecycle events
- **THEN** the collector counts each terminal response/tool once and emits
  only aggregate sessions, turns, tools and input/output/cache token buckets

#### Scenario: Session source is unsafe or unsupported

- **WHEN** the session is malformed, oversized, symlinked, outside the private
  run, uses an unknown major or has incomplete usage
- **THEN** the collector rejects or marks the affected aggregate unavailable
  with a closed reason and does not copy raw content to metrics or logs

### Requirement: Supervisor backlog freshness is explicit and non-mutating

The factory SHALL expose a dedicated live backlog audit for supervisor work
selection. The command SHALL use the repository identity from tracked policy,
request only bounded issue-state metadata, deduplicate issue lookups and make
GitHub or malformed-response failures visible. It SHALL NOT modify issues,
roadmap state, repository settings or worker configuration.

A strict snapshot input MAY reproduce and test the decision without network
access. Snapshot use SHALL be reported distinctly from a live result and SHALL
validate exact schema, repository identity, unique issue numbers and closed
state values.

#### Scenario: Supervisor prepares a Pi task

- **WHEN** the supervisor is about to select a canonical `in_progress` work
  item
- **THEN** it runs the live backlog audit before preparing the issue-bound Pi
  invocation envelope

#### Scenario: Fast CI validates repository structure

- **WHEN** required pull-request CI runs without GitHub coordination access
- **THEN** it continues to run the offline backlog checker and does not claim
  live freshness

#### Scenario: Audit receives a fixture snapshot

- **WHEN** a test or operator supplies a valid exact-repository issue-state
  snapshot
- **THEN** the same delivery-state rules run without invoking GitHub and the
  output identifies snapshot mode

### Requirement: Delivery iteration is locally batched and review-bounded

The factory SHALL treat one automatic discovery review and one remediation
round as the normal review budget. It SHALL perform focused local checks and a
distinct local review before the first candidate push, batch related repairs,
and preserve final exact-head evidence. A round budget SHALL NOT waive a P0/P1,
security regression, introduced defect, or failed acceptance criterion. New
independent non-blocking findings after the remediation cutoff SHALL become
linked follow-up issues instead of silently expanding the current slice.

One slice SHALL own one coherent behavior. Crossing 12 changed files or 1,000
non-generated changed lines SHALL require a decomposition note, but SHALL NOT
automatically waive or reject a cohesive migration, fixture, or security
change.

#### Scenario: First candidate is ready for hosted evidence

- **WHEN** focused local checks and local review pass for the bounded slice
- **THEN** the supervisor pushes one candidate head and requests one automatic
  discovery review after the first green fast result

#### Scenario: Review finds an introduced defect

- **WHEN** any round identifies a security regression, P0/P1, failed acceptance
  criterion, or defect introduced by the slice
- **THEN** the finding remains blocking until fixed or the slice is withdrawn

#### Scenario: Later review discovers independent hardening

- **WHEN** the remediation round is complete and a new P2/P3 finding is
  independent of the slice's acceptance and introduced behavior
- **THEN** the supervisor links a follow-up issue with evidence and does not
  expand an otherwise eligible pull request

#### Scenario: Slice exceeds decomposition guidance

- **WHEN** the exact diff crosses the documented file or non-generated line
  threshold
- **THEN** the plan records why the slice remains cohesive or names the split;
  the number alone neither approves nor rejects integration

### Requirement: Accepted delivery-policy values are validated exactly

The factory SHALL derive one target plan from an exact base/head diff and
delivery profile. The active-development PR requirement SHALL remain the
single Linux `fast` gate. Slow promotion SHALL block exactly, in order,
`production-promotion`, `publication`, and `release-preparation`; an omitted,
reordered, or additional blocker SHALL fail policy validation.

Slice decomposition guidance SHALL remain exactly 12 changed files and 1,000
changed text lines with `decomposition-note` as the advisory action. A lower or
higher threshold SHALL fail policy validation. Crossing the canonical values
SHALL request a decomposition note and SHALL NOT alone approve or reject work.

#### Scenario: Slow blocker policy drifts

- **WHEN** a blocker is omitted, reordered, or added
- **THEN** policy validation fails closed before a target plan is emitted

#### Scenario: Decomposition guidance drifts

- **WHEN** either threshold is lower or higher than its accepted value, or the
  advisory action changes
- **THEN** policy validation fails closed without turning the threshold into a
  correctness decision

#### Scenario: Canonical policy is planned

- **WHEN** the exact accepted blocker set and decomposition guidance are loaded
- **THEN** the existing fast/slow plan shape and single required `fast` status
  remain unchanged
