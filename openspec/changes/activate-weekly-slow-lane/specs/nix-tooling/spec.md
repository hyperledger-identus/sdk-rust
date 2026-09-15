## MODIFIED Requirements

### Requirement: Complete Clippy surface is slow evidence

The checks module SHALL provide a generated
`rust-clippy-all-targets-all-features` check on primary Rust 1.98.1 which runs
Clippy with workspace, all-target, all-feature, and warnings-denied selection.
Both host-tested systems SHALL execute that named check through the active
native weekly/manual slow workflow or its exact local reproduction. The
required fast workflow SHALL retain its existing default-surface `rust-clippy`
selector and SHALL NOT select the complete check.

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
