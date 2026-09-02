## ADDED Requirements

### Requirement: Qualifying work has an OpenSpec change contract
The repository SHALL require a kebab-case OpenSpec change before implementation
begins for behavioral, public API, architecture, protocol, security and
multi-step work. The change SHALL use the repository `spec-driven` schema and
identify its affected capabilities.

#### Scenario: Agent begins a qualifying implementation
- **WHEN** an agent is asked to implement qualifying work
- **THEN** the agent can identify a repository-local change containing a
  proposal, capability specs, design and tasks before editing implementation
  files

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
A verified change SHALL sync its accepted delta requirements into current
capability specs and SHALL be archived with its proposal, design and completed
tasks before the delivery is finalized.

#### Scenario: Bootstrap change is finalized
- **WHEN** every bootstrap task and verification gate passes
- **THEN** the two new capability specs exist under `openspec/specs/` and the
  completed change exists under `openspec/changes/archive/`
