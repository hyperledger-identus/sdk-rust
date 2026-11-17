## MODIFIED Requirements

### Requirement: Compiler-floor selection is phase-appropriate and measurable

The SDK SHALL select its compiler floor from measured dependency value,
supported consumer constraints, target evidence and delivery phase rather than
an arithmetic average or release-distance formula. The published
`identus-derive` / `identus-core` / `identus-crypto` `0.1.0-rc.1` artifacts
retain their historical Rust 1.89.0 floor. The next unpublished `0.2.0-rc.1`
source line SHALL use exact stable Rust 1.99.1 for its package floor, primary
development, quality, documentation, candidate preparation, and etalon
evidence. Dependency research SHALL record each candidate's declared and
observed compiler requirements against the active 1.99.1 contract.

The independently named MSRV evidence SHALL execute in weekly/manual slow and
exact release-candidate evidence rather than required per-PR fast CI. A future
MSRV increase SHALL identify concrete dependency, correctness, security,
target, or supported-consumer value; update Cargo, Nix, machine policy,
migration guidance, compatibility ledger, and evidence atomically; and SHALL
NOT occur within the `0.2.x` release line. A boundary adapter MAY declare a
different crate-local floor only through a separate material decision and
SHALL NOT change generic core automatically.

#### Scenario: Dependency fits the active source floor

- **WHEN** a cohesive dependency declares and proves Rust requirements no newer
  than 1.99.1 and passes every other adoption gate
- **THEN** the active source MSRV does not block its independently justified use

#### Scenario: Dependency requires a newer compiler

- **WHEN** a candidate requires Rust newer than 1.99.1
- **THEN** it remains deferred until a focused compatibility decision proves
  concrete payoff, migration, all target/profile evidence, and a later minor
  release line

#### Scenario: Release candidate preparation runs

- **WHEN** the exact unpublished `0.2.0-rc.1` candidate is assembled
- **THEN** its manifests and preparation receipt declare exact Rust 1.99.1
  while published `0.1.0-rc.1` evidence remains historical and unchanged

#### Scenario: Boundary adapter needs a distinct compiler constraint

- **WHEN** an accepted FFI or platform adapter cannot share the workspace floor
  for a measured reason
- **THEN** its focused ADR may define a crate-local constraint without silently
  changing the generic core promise
