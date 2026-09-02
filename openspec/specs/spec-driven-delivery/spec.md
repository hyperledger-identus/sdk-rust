# spec-driven-delivery Specification

## Purpose
TBD - created by archiving change bootstrap-ai-software-factory. Update Purpose after archive.
## Requirements
### Requirement: Qualifying work has an OpenSpec change contract

The repository SHALL require a kebab-case OpenSpec change before implementation
begins for behavioral, public API, architecture, protocol, security and
multi-step work. The change SHALL use the repository `spec-driven` schema,
identify its affected capabilities and receive a semantic review with no
unresolved blocker. A human or agent MAY author and activate the contract; a
separate human acceptance state SHALL NOT be required for routine work within
the standing mandate.

#### Scenario: Agent begins a qualifying implementation

- **WHEN** an agent begins qualifying work within the standing mandate
- **THEN** the agent can identify a repository-local change containing a
  proposal, capability specs, design, tasks and completed semantic review before
  editing implementation files

#### Scenario: Agent authors the contract

- **WHEN** the agent-authored contract is structurally valid, semantically
  reviewed and free of unresolved blockers
- **THEN** the contract is ready for implementation without a human approval or
  format-acceptance step

#### Scenario: Administrative edit is exempt

- **WHEN** a contributor makes a typo, formatting or other explicitly exempt
  administrative edit
- **THEN** the pull request records that OpenSpec is not applicable and does not
  fabricate an empty change

### Requirement: Change artifacts are structurally valid
Every active change SHALL pass strict, non-interactive OpenSpec validation. An
active change committed for review SHALL contain `.openspec.yaml`,
`proposal.md`, `design.md`, `tasks.md` and at least one capability spec.

#### Scenario: Complete active change is checked
- **WHEN** `scripts/factory check` runs against a complete and structurally
  valid active change
- **THEN** the command exits successfully

#### Scenario: Active change is missing an artifact
- **WHEN** the structural checker finds an active change without a required
  artifact or task checkbox
- **THEN** the command exits non-zero and names the missing contract element

#### Scenario: OpenSpec validation fails
- **WHEN** any current spec or active change fails strict OpenSpec validation
- **THEN** the factory check exits non-zero without marking the change ready

### Requirement: Draft and ready states are distinct
The factory SHALL permit incomplete task checkboxes during draft validation and
SHALL reject readiness while any required artifact or task is incomplete.

#### Scenario: Draft change has pending tasks
- **WHEN** `scripts/factory check` runs and an otherwise valid change has
  unchecked tasks
- **THEN** structural validation succeeds so the draft can receive review

#### Scenario: Pending task blocks readiness
- **WHEN** `scripts/factory ready <change>` runs for a change with an unchecked
  task
- **THEN** the command exits non-zero and identifies the incomplete task state

#### Scenario: Completed change is ready
- **WHEN** strict validation passes and every required artifact and task for the
  named change is complete
- **THEN** `scripts/factory ready <change>` exits successfully

### Requirement: Verified changes preserve their specification history

A verified change SHALL sync its reviewed delta requirements into current
capability specs and SHALL be archived with its proposal, design and completed
tasks before the delivery is finalized.

#### Scenario: Verified change is finalized

- **WHEN** every change task and verification gate passes
- **THEN** its reviewed capability requirements exist under `openspec/specs/`
  and the completed change exists under `openspec/changes/archive/`

### Requirement: Every pull request has an issue and local review evidence

Every pull request SHALL reference a corresponding repository issue that exists
before the pull request is opened. If no issue exists, a human or agent SHALL
create one first. Implementation and a distinct local review pass SHALL be
complete before a ready pull request is published. The pull request SHALL
record the issue and local review result in the repository template.

#### Scenario: Existing issue covers the work

- **WHEN** a contributor completes and locally reviews the scoped change
- **THEN** the ready pull request targets `develop` and references the existing
  issue

#### Scenario: No issue covers the work

- **WHEN** completed, locally reviewed work has no corresponding issue
- **THEN** the human or agent creates an issue before opening the pull request

#### Scenario: Administrative edit is delivered

- **WHEN** an OpenSpec-exempt typo, formatting or administrative edit is ready
- **THEN** it still references a lightweight delivery issue and records why
  OpenSpec was not required
