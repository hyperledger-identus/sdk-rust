# ai-software-factory Specification

## Purpose
TBD - created by archiving change bootstrap-ai-software-factory. Update Purpose after archive.
## Requirements
### Requirement: Humans retain decision and release authority
The AI Software Factory SHALL keep product intent, scope acceptance, governance,
security disclosure, merge, repository administration and release authority
with accountable human maintainers. An agent-generated result SHALL NOT count
as approval of its own work.

#### Scenario: Agent completes a change
- **WHEN** an agent produces implementation and verification evidence
- **THEN** the result remains a review candidate until the required human and
  independent reviews accept it

#### Scenario: Automation encounters an authority gate
- **WHEN** progress requires merge, publication, disclosure, secret use or a
  repository-policy change not explicitly authorized by a human
- **THEN** the automation stops and reports the required decision

### Requirement: One portable factory entrypoint controls the workflow
The repository SHALL provide `scripts/factory` with `doctor`, `status`,
`validate`, `check`, `ready` and `receipt` commands. The same implementation
SHALL be callable through the pinned Nix environment and documented `just`
aliases without requiring a global OpenSpec installation.

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
