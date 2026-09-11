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
single Linux `fast` gate; weekly/manual slow evidence SHALL be recommended for
toolchain, Nix, security, FFI and release-sensitive paths. Unknown diff state
SHALL fail closed to the complete public evidence recommendation.

#### Scenario: Factory-only pull request is planned

- **WHEN** an exact diff changes factory scripts, contracts or CI
- **THEN** the plan requires `fast`, records the factory area and recommends
  the complete slow backstop without creating three per-PR Rust builds

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
