# dependency-research-readiness

## MODIFIED Requirements

### Requirement: Compiler-floor selection is phase-appropriate and measurable

The SDK SHALL select its compiler floor from measured dependency value,
supported consumer constraints, target evidence and delivery phase rather than
an arithmetic average or release-distance formula. For the
`identus-derive` / `identus-core` / `identus-crypto` `0.1.0-rc.1` train, the
published package floor SHALL be exact Rust 1.89.0 while primary development,
quality, documentation, and candidate preparation SHALL use exact stable Rust
1.98.1. Dependency research SHALL record each candidate's declared and
observed compiler requirements against both versions.

The independent MSRV lane SHALL execute in weekly/manual slow and exact
release-candidate evidence rather than required per-PR fast CI. A future MSRV
increase SHALL identify concrete dependency, correctness, security, target, or
supported-consumer value; update Cargo, Nix, machine policy, migration
guidance, and evidence atomically; and SHALL NOT occur within the `0.1.x`
release line. A boundary adapter MAY declare a different crate-local floor
only through a separate material decision and SHALL NOT change generic core
automatically.

#### Scenario: Dependency fits the published release floor

- **WHEN** a cohesive dependency declares and proves Rust requirements no newer
  than 1.89.0 and passes every other adoption gate
- **THEN** the release MSRV does not block its independently justified use

#### Scenario: Dependency requires a newer compiler

- **WHEN** a candidate requires Rust newer than 1.89.0
- **THEN** it remains deferred until a focused compatibility decision proves
  concrete payoff, migration, all target/profile evidence, and a later minor
  release line

#### Scenario: Release candidate preparation runs

- **WHEN** the exact `0.1.0-rc.1` candidate is assembled
- **THEN** its manifests declare Rust 1.89.0 and its receipt binds both the
  independent MSRV evidence and Rust 1.98.1 preparation evidence

#### Scenario: Boundary adapter needs a distinct compiler constraint

- **WHEN** an accepted FFI or platform adapter cannot share the workspace floor
  for a measured reason
- **THEN** its focused ADR may define a crate-local constraint without silently
  changing the generic core promise

