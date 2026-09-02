## MODIFIED Requirements

### Requirement: Humans retain decision and release authority

The AI Software Factory SHALL keep product intent, scope acceptance, repository
administration, security disclosure, publishing, promotion to `main` and release
authority with accountable human maintainers. For a human-approved scope, a
human or agent MAY publish a locally reviewed feature branch, open an
issue-linked pull request targeting `develop` and merge it after every required
CI check succeeds and no blocking review remains. An agent-generated result
SHALL NOT approve its own scope or waive a security, compatibility, provenance
or conformance finding.

#### Scenario: Agent completes locally reviewed work

- **WHEN** an agent completes implementation and a distinct local review pass
  for work with an accepted scope
- **THEN** the agent MAY publish the feature branch and open a ready pull request
  targeting `develop` that references the corresponding issue

#### Scenario: Pull request satisfies the integration gate

- **WHEN** an issue-linked pull request is non-draft, mergeable, has no
  unresolved blocking review and every required CI check is successful
- **THEN** a human or agent MAY merge it into `develop` without a separate
  per-merge authorization

#### Scenario: Pull request has an incomplete gate

- **WHEN** any required CI check is missing, pending, cancelled or failing, or a
  blocking review remains unresolved
- **THEN** the pull request SHALL NOT be merged

#### Scenario: Automation encounters protected authority

- **WHEN** progress requires scope expansion, repository administration,
  publication, disclosure, secret use, release or promotion to `main`
- **THEN** the automation stops and reports the required human decision

## ADDED Requirements

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

- **WHEN** the pull-request body has no accepted issue reference
- **THEN** the `pull-request-policy` status fails with an issue-reference error

#### Scenario: Draft pull request is checked

- **WHEN** the pull-request event reports that the pull request is a draft
- **THEN** the `pull-request-policy` status fails until the pull request is ready
