## RENAMED Requirements

- FROM: `### Requirement: Layer conformance — depends only on identus-core`
- TO: `### Requirement: Layer conformance — depends only on identus-core and the proc-macro attribute provider`

## MODIFIED Requirements

### Requirement: Layer conformance — depends only on identus-core and the proc-macro attribute provider

The crate SHALL declare `identus-core` as its only runtime workspace-internal dependency, plus the proc-macro attribute provider `identus-derive` (referenced via `identus-derive.workspace = true` under `[dependencies]`) per `naming-conventions` ("Port-owning crates depend on the proc-macro attribute provider"); it SHALL NOT depend on any other workspace-internal crate. It SHALL NOT depend on `identus-adapters`, `identus-bindings`, `identus-conformance`, or any `identus-adapters-<family>` crate (e.g. `identus-adapters-entropy`). It SHALL NOT depend on `ring` (the `ring` entropy adapter is in `identus-adapters-entropy`). All external crates SHALL be declared at workspace level per `workspace-dependency-conventions`. The `crate-ring-layout` conformance guard SHALL pass.

#### Scenario: crypto declares only identus-core and identus-derive inward

- **WHEN** `crates/crypto/Cargo.toml` `[dependencies]` is inspected for workspace-internal crates
- **THEN** it SHALL contain only `identus-core` and `identus-derive` (the proc-macro attribute provider, per `naming-conventions`), and no other workspace-internal crate

#### Scenario: crypto does not depend on an adapter-family crate

- **WHEN** `crates/crypto/Cargo.toml` is inspected
- **THEN** it SHALL NOT list `identus-adapters-entropy` (or any `identus-adapters-*`); the conformance guard SHALL reject such an edge

#### Scenario: Layer guard passes for crypto

- **WHEN** `cargo test -p identus-conformance` is run
- **THEN** the dep-graph guard SHALL pass for `identus-crypto` (no outward edges)