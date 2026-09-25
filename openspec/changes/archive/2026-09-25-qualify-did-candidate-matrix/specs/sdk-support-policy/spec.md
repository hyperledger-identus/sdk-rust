# sdk-support-policy

## ADDED Requirements

### Requirement: Candidate matrix evidence remains on the slow line

The repository MUST run candidate-specific MSRV, secondary-host,
feature-profile, and portable-target qualification only in native
weekly/manual slow CI or exact local reproduction. It MUST NOT add another
required pull-request compiler, host, or target lane. Slow CI MUST retain
attempt-scoped lane artifacts and one exact-SHA aggregate receipt suitable for
later release approval.

#### Scenario: Ordinary pull request changes candidate matrix wiring

- **WHEN** a pull request targets protected `develop`
- **THEN** the existing Linux `fast` job remains the only required Rust/factory
  merge signal and structural tests validate the dormant slow wiring

#### Scenario: Final approval needs target evidence

- **WHEN** the candidate becomes otherwise ready for final review
- **THEN** a natural or explicitly authorized manual slow run at the unchanged
  SHA supplies the aggregate matrix receipt; PR CI success cannot substitute
