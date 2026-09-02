## MODIFIED Requirements

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

### Requirement: Verified changes preserve their specification history

A verified change SHALL sync its reviewed delta requirements into current
capability specs and SHALL be archived with its proposal, design and completed
tasks before the delivery is finalized.

#### Scenario: Verified change is finalized

- **WHEN** every change task and verification gate passes
- **THEN** its reviewed capability requirements exist under `openspec/specs/`
  and the completed change exists under `openspec/changes/archive/`
