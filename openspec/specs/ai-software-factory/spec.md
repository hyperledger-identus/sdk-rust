# ai-software-factory Specification

## Purpose
TBD - created by archiving change bootstrap-ai-software-factory. Update Purpose after archive.
## Requirements
### Requirement: Humans retain decision and release authority

The AI Software Factory SHALL keep product strategy, governance, repository
administration, security disclosure, secret use, publishing, promotion to
`main` and release authority with accountable human maintainers. The standing
project mandate SHALL authorize a human or agent to select and prioritize
routine work; create its issue, OpenSpec contract and ADR; make reversible
product and technical decisions; implement and distinctly review it; publish a
focused branch; repair branch-owned CI; and merge an eligible pull request into
`develop` without separate formal, format, push or merge approval. No agent
SHALL waive an unresolved security, compatibility, provenance, conformance,
legal or data-loss finding.

#### Scenario: Agent starts routine work

- **WHEN** an agent identifies a bounded, reversible slice within the recorded
  roadmap and repository boundaries
- **THEN** it MAY create the issue and contract and proceed through semantic
  review without waiting for human scope or artifact-format acceptance

#### Scenario: Agent completes locally reviewed work

- **WHEN** an agent completes implementation and a distinct local review pass
  for issue-linked work within the standing mandate
- **THEN** the agent MAY publish the feature branch and open a ready pull request
  targeting `develop`

#### Scenario: Pull request satisfies the integration gate

- **WHEN** an issue-linked pull request is non-draft, mergeable, has no
  unresolved blocking review and every required CI check is successful
- **THEN** a human or agent MAY merge it into `develop` without a separate
  per-merge authorization

#### Scenario: Pull request has an incomplete gate

- **WHEN** any required CI check is missing, pending, cancelled or failing, or a
  blocking review remains unresolved
- **THEN** the pull request SHALL NOT be merged until the condition is resolved

#### Scenario: Automation encounters protected authority

- **WHEN** progress requires a material strategy or public-commitment change,
  governance or licensing action, secret use, private disclosure, irreversible
  external action, repository administration, publication, release, promotion
  to `main` or acceptance of unresolved material risk
- **THEN** the automation stops and reports the required human decision

### Requirement: One portable factory entrypoint controls the workflow

The repository SHALL provide `scripts/factory` with `doctor`, `status`,
`validate`, `check`, `ready` and `receipt` commands. The same implementation
SHALL be callable through the pinned Nix environment and documented `just`
aliases without requiring a global OpenSpec installation.

The entrypoint SHALL additionally provide `archive <change>`, which runs
readiness and the scoped modified-requirement preservation preflight before
invoking the pinned non-interactive OpenSpec archive, then validates the
post-archive factory state.

#### Scenario: Contributor is inside the Nix devshell

- **WHEN** the contributor invokes `scripts/factory check` with `openspec` on
  `PATH`
- **THEN** the script uses the pinned binary and validates the repository

#### Scenario: Contributor has Nix but not global OpenSpec

- **WHEN** the contributor invokes `scripts/factory check` outside the devshell
- **THEN** the script re-enters the pinned Nix environment and performs the same
  validation

#### Scenario: Factory help is requested

- **WHEN** the contributor invokes `scripts/factory help`
- **THEN** the command lists every supported operation and its required inputs

#### Scenario: Guarded archive is requested

- **WHEN** a contributor invokes `scripts/factory archive <change>`
- **THEN** readiness and preservation checks pass before OpenSpec mutates the
  canonical specs or moves the change, and the post-archive state is validated

#### Scenario: Lossy archive is requested

- **WHEN** a modified requirement would silently discard canonical content
- **THEN** `scripts/factory archive <change>` exits non-zero before OpenSpec
  changes either the canonical spec or active change directory

### Requirement: Factory work is isolated from integration and consumer trees
Implementation agents SHALL use focused branches and dedicated worktrees based
on current `develop`. They SHALL treat `main` and downstream consumer
repositories as read-only unless an explicit, separate authorization names the
target and mutation.

#### Scenario: Doctor runs on main
- **WHEN** `scripts/factory doctor` detects the `main` branch
- **THEN** it exits non-zero and directs implementation work to a branch based
  on `develop`

#### Scenario: Agent works on an authorized feature branch
- **WHEN** the current branch descends from `origin/develop` and the OpenSpec
  relationship is healthy
- **THEN** `scripts/factory doctor` reports the branch as a valid work surface

### Requirement: Readiness produces a truthful evidence receipt
`scripts/factory receipt <change>` SHALL run the readiness gate before printing
the change name, branch, head revision and `develop` merge base. The receipt
SHALL distinguish factory-contract evidence from unrun product-specific gates.

#### Scenario: Ready change requests a receipt
- **WHEN** all change artifacts and tasks are complete and strict validation
  passes
- **THEN** the command prints immutable change and Git identifiers and states
  that the factory contract passed

#### Scenario: Incomplete change requests a receipt
- **WHEN** readiness fails for the named change
- **THEN** the command exits non-zero and does not print a passing receipt

### Requirement: CI enforces a stable factory contract
The Nix flake SHALL expose a `factory-contract` check, and GitHub Actions SHALL
run a job with that stable name for pull requests and pushes to `develop`. The
check SHALL validate the structural test suite and strict OpenSpec state.

#### Scenario: Valid factory change reaches CI
- **WHEN** a pull request contains valid factory and OpenSpec artifacts
- **THEN** the `factory-contract` status completes successfully

#### Scenario: Invalid active change reaches CI
- **WHEN** a pull request omits a required artifact or contains an invalid spec
- **THEN** the `factory-contract` status fails with the structural or OpenSpec
  validation error

### Requirement: Client adapters do not fork the factory contract
Repository-owned client prompts, skills and chains SHALL delegate to the
portable factory and OpenSpec lifecycle. Personal model, token, MCP and
workspace configuration SHALL NOT be committed.

#### Scenario: Pi apply chain is invoked
- **WHEN** a user supplies an OpenSpec change name to the Pi apply chain
- **THEN** the chain delegates that change name without a hard-coded historical
  change identifier

#### Scenario: Tracked local state is checked
- **WHEN** the factory checker finds a tracked personal environment, model,
  token, MCP or editor-state path
- **THEN** it exits non-zero and names the forbidden path

### Requirement: Pull-request integration policy is executable

The repository SHALL provide a portable pull-request policy checker and a
GitHub Actions job named `pull-request-policy`. For pull requests, the checker
SHALL require `develop` as the base branch, a non-draft state, a repository
issue reference and completed local review evidence. The job SHALL verify that
the referenced issue exists in the repository, use read-only permissions and
SHALL treat pull-request body text as data rather than executable shell input.

#### Scenario: Ready issue-linked pull request is checked

- **WHEN** a non-draft pull request targets `develop`, references its issue and
  records completed local review
- **THEN** the `pull-request-policy` status completes successfully

#### Scenario: Pull request omits its issue

- **WHEN** the pull-request body has no repository issue reference
- **THEN** the `pull-request-policy` status fails with an issue-reference error

#### Scenario: Draft pull request is checked

- **WHEN** the pull-request event reports that the pull request is a draft
- **THEN** the `pull-request-policy` status fails until the pull request is ready

### Requirement: Routine delivery uses evidence gates instead of approval gates

The factory SHALL treat issues, OpenSpec contracts, ADRs, evidence receipts,
local reviews and CI results as durable coordination and evidence gates. It
SHALL NOT require a human approval token or named human scope owner for routine,
reversible work within the standing mandate.

#### Scenario: Agent makes a routine implementation decision

- **WHEN** naming, formatting, decomposition, implementation, testing,
  documentation, refactoring, CI repair or reversible tooling work is needed
- **THEN** the agent records material reasoning where useful and continues
  without requesting approval

#### Scenario: Contract has an unresolved material product fork

- **WHEN** two plausible choices produce materially different user outcomes and
  the roadmap, normative sources and safe investigation do not resolve them
- **THEN** the agent records the alternatives and requests product direction

### Requirement: Archive success proves the requested state transition

The factory archive facade SHALL report success only after the requested active
change is absent, exactly one new dated archive entry ending in the requested
change name exists relative to the pre-mutation snapshot, that entry is a
regular non-symlink directory, every mandatory change artifact is preserved,
and the resulting OpenSpec store passes the factory validation gate. It SHALL
derive completion from repository state, SHALL NOT predict the completed
archive from the host-local calendar date, and SHALL NOT parse human-readable
OpenSpec output as a machine contract.

#### Scenario: OpenSpec exits zero without archiving

- **WHEN** the pinned OpenSpec archive command returns zero but leaves the
  requested change active or creates no new matching archive entry
- **THEN** `scripts/factory archive` exits non-zero and does not print its safe
  archive success marker

#### Scenario: Archive date differs from the host date

- **WHEN** OpenSpec removes the active change and creates one complete matching
  archive under a date different from the host-local date
- **THEN** `scripts/factory archive` validates that newly created directory and
  reports that the named change was archived safely

#### Scenario: Archive result is ambiguous or indirect

- **WHEN** OpenSpec creates multiple new matching entries, creates a matching
  symlink, or only a pre-existing matching entry is present
- **THEN** `scripts/factory archive` exits non-zero without accepting any entry
  as the completed archive

#### Scenario: Requested archive transition completes

- **WHEN** OpenSpec removes the active change, creates exactly one new regular
  matching archive directory, preserves every mandatory artifact and the
  resulting store validates
- **THEN** `scripts/factory archive` reports that the named change was archived
  safely

