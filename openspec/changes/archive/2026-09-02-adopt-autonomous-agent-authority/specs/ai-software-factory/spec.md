## MODIFIED Requirements

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

## ADDED Requirements

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
