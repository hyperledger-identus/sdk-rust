## MODIFIED Requirements

### Requirement: One pinned bootstrap controls the worker runtime

The repository SHALL provide one Nix-backed bootstrap entrypoint that can
check the factory, audit the effective Pi runtime, configure repository-owned
Git hooks, apply the bounded user sub-agent policy only with an explicit
execute action, and launch Pi after its configuration audit passes. It SHALL
not read or replace provider authentication.

The bootstrap health check SHALL fail unless structural factory checks, the
effective pinned runtime audit and the operational test suite all pass in the
same pinned environment.

#### Scenario: Operator launches Pi

- **WHEN** the tracked runtime, package declarations, policies and local user
  sub-agent bounds are aligned
- **THEN** `./bootstrap.sh --pi` launches the Nix-supplied Pi executable

#### Scenario: Runtime audit finds drift

- **WHEN** a required contract, version, budget, hook or capacity invariant is
  missing or inconsistent
- **THEN** audit exits non-zero with a remediation and Pi is not launched

#### Scenario: Operator runs the bootstrap health check

- **WHEN** the operator invokes `./bootstrap.sh --check`
- **THEN** structural factory checks, effective pinned runtime audit and
  operational tests run in order, and any failed stage makes the command fail
