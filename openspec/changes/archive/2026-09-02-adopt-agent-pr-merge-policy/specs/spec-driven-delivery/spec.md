## ADDED Requirements

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
