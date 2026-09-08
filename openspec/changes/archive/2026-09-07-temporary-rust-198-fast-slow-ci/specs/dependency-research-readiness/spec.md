## RENAMED Requirements

- FROM: `### Requirement: MSRV selection is measurable and independently gated`
- TO: `### Requirement: Compiler-floor selection is phase-appropriate and measurable`

## MODIFIED Requirements

### Requirement: Compiler-floor selection is phase-appropriate and measurable

The SDK SHALL select its compiler floor from measured dependency value,
supported consumer constraints, target evidence and delivery phase rather than
an arithmetic average or release-distance formula. During the temporary
unpublished active-development phase authorized by discussion #172, the
workspace floor, primary compiler and compatibility etalon SHALL all be exact
Rust 1.98.1 and no lower-version compatibility SHALL be claimed. Dependency
research SHALL still record each candidate's declared and observed compiler
requirements so a later release decision has evidence.

A release candidate SHALL require a focused compatibility decision that uses
named consumer and target evidence, updates Cargo, Nix, machine policy,
migration guidance and CI together, and resolves all weekly slow-lane failures.
A boundary adapter MAY declare a different crate-local floor only through a
separate material decision and SHALL NOT change generic core automatically.

#### Scenario: Dependency requires a recent compiler during active development

- **WHEN** a cohesive dependency requires Rust no newer than the exact 1.98.1
  workspace floor and passes all other adoption gates
- **THEN** no artificial lower-MSRV lane blocks research or implementation

#### Scenario: Release candidate preparation begins

- **WHEN** the project proposes a release candidate or reaches 2026-12-08
- **THEN** a focused decision selects and enforces a consumer-driven compiler
  matrix before any artifact can be published

#### Scenario: Boundary adapter needs a distinct compiler constraint

- **WHEN** an accepted FFI or platform adapter cannot share the workspace floor
  for a measured reason
- **THEN** its focused ADR may define a crate-local constraint without silently
  changing the generic core promise
