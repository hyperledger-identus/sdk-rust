## MODIFIED Requirements

### Requirement: Pull-request integration policy is executable

The repository SHALL provide a portable pull-request policy checker and a
GitHub Actions job named `pull-request-policy`. For pull requests, the checker
SHALL require `develop` as the base branch, a non-draft state, a repository
issue reference and completed local review evidence. The job SHALL verify that
the referenced issue exists in the repository and use read-only permissions.
Its workflow, checker, policy configuration, and executable repository content
SHALL come from the event's exact protected base revision. Pull-request head
content and metadata SHALL be treated only as untrusted data; the job SHALL NOT
check out or execute the head tree.

#### Scenario: Ready issue-linked pull request is checked

- **WHEN** a non-draft pull request targets `develop`, references its issue and
  records completed local review
- **THEN** the `pull-request-policy` status completes successfully

#### Scenario: Pull request changes its own checker

- **WHEN** a pull request modifies the workflow, checker, or policy
  configuration that will govern later contributions
- **THEN** that pull request is still judged only by those artifacts from its
  exact protected base revision

#### Scenario: Base or head identity is inconsistent

- **WHEN** the checked-out base or fetched head object differs from the exact
  SHA in the event
- **THEN** the job fails closed before contribution policy evaluation

#### Scenario: Pull request omits its issue

- **WHEN** the pull-request body has no repository issue reference
- **THEN** the `pull-request-policy` status fails with an issue-reference error

#### Scenario: Draft pull request is checked

- **WHEN** the pull-request event reports that the pull request is a draft
- **THEN** the `pull-request-policy` status fails until the pull request is ready
