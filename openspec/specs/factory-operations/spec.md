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

#### Scenario: Operator launches Pi

- **WHEN** the tracked runtime, package declarations, policies and local user
  sub-agent bounds are aligned
- **THEN** `./bootstrap.sh --pi` launches the Nix-supplied Pi executable

#### Scenario: Runtime audit finds drift

- **WHEN** a required contract, version, budget, hook or capacity invariant is
  missing or inconsistent
- **THEN** audit exits non-zero with a remediation and Pi is not launched

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
