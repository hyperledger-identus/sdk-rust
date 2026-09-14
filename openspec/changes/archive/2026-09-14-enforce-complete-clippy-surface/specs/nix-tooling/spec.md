# nix-tooling Specification

## ADDED Requirements

### Requirement: Complete Clippy surface is slow evidence

The checks module SHALL provide a generated
`rust-clippy-all-targets-all-features` check on primary Rust 1.98.1 which runs
Clippy with workspace, all-target, all-feature, and warnings-denied selection.
Both host-tested systems SHALL execute that named check through the
weekly/manual slow workflow. The required fast workflow SHALL retain its
existing default-surface `rust-clippy` selector and SHALL NOT select the
complete check.

#### Scenario: A non-default target or feature emits a warning

- **WHEN** any first-party workspace target under the all-feature selection
  emits a Clippy warning
- **THEN** `rust-clippy-all-targets-all-features` fails the slow evidence

#### Scenario: A pull request enters required fast CI

- **WHEN** the temporary active-development fast workflow evaluates a pull
  request to `develop`
- **THEN** it runs the existing default-surface `rust-clippy` check without
  selecting `rust-clippy-all-targets-all-features`

#### Scenario: A maintainer reproduces the complete check locally

- **WHEN** the documented Cargo command or the named host-specific Nix check is
  run with the pinned environment
- **THEN** it evaluates the same workspace/all-target/all-feature
  warnings-denied surface used by slow CI

### Requirement: First-party Clippy exceptions are narrow and expiring

A retained first-party Clippy exception SHALL be scoped to the smallest item,
use a reasoned lint expectation rather than an open-ended production
allowance, and record its owning component, rationale, and objective removal
condition. Crate-level and workspace-wide Clippy allowances SHALL NOT be
introduced.

#### Scenario: A retained lint exception stops matching

- **WHEN** refactoring removes the lint from an item carrying a reasoned
  expectation
- **THEN** the warnings-denied complete check reports the unfulfilled
  expectation instead of silently retaining dead suppression

#### Scenario: A production exception is audited

- **WHEN** a reviewer inspects the lint-exception registry
- **THEN** the exact source item, lint, owner, rationale, and removal condition
  are available without inferring them from history
