## MODIFIED Requirements

### Requirement: One module per guard, under a guard directory

Each invariant family SHALL be enforced by exactly one guard module under `crates/conformance/src/guard/`, named for the invariant family (e.g. `dep_graph.rs` for the layer/dependency-direction guard, `naming.rs` for the port/adapter naming guard). The `guard/` directory SHALL be declared `#[cfg(test)] mod guard;` from `lib.rs`, and its submodules SHALL be declared from `guard/mod.rs`. Adding a new guard SHALL NOT require editing an existing guard module.

#### Scenario: the dep-graph guard is its own module

- **WHEN** `crates/conformance/src/guard/` is inspected
- **THEN** it SHALL contain `dep_graph.rs` holding the layer-membership and dependency-direction guard (moved verbatim from the prior single-file `lib.rs`)

#### Scenario: the naming guard is its own module

- **WHEN** `crates/conformance/src/guard/` is inspected
- **THEN** it SHALL contain `naming.rs` holding the port/adapter naming guard, as a sibling of `dep_graph.rs`

#### Scenario: guards are test-only

- **WHEN** `crates/conformance/src/lib.rs` is inspected for the guard declaration
- **THEN** it SHALL declare `#[cfg(test)] mod guard;` so that no guard logic compiles into a production build

#### Scenario: the unsafe-policy guard is its own module

- **WHEN** `crates/conformance/src/guard/` is inspected
- **THEN** it SHALL contain `unsafe_policy.rs` holding root/member lint and
  behavioral compile-fail enforcement as a sibling of existing guards

### Requirement: Shared helpers live in the guard module root

File-walking (enumerating `crates/*/src/**/*.rs` and `crates/**/Cargo.toml` files), root-manifest parsing, and `syn` source parsing SHALL be provided as shared helpers in `crates/conformance/src/guard/mod.rs` and SHALL be reused by every guard. A guard SHALL NOT duplicate file-walking, manifest-parsing, or `syn::parse_file` logic that already exists in the shared helpers.

#### Scenario: shared file-walking helper exists

- **WHEN** `crates/conformance/src/guard/mod.rs` is inspected
- **THEN** it SHALL provide a helper that enumerates workspace source/manifest file paths, used by both `dep_graph.rs` and `naming.rs`

#### Scenario: shared syn parsing helper exists

- **WHEN** `crates/conformance/src/guard/mod.rs` is inspected
- **THEN** it SHALL provide a helper that parses a `.rs` file into a `syn::File`, reused by `naming.rs` (and available to future source-scanning guards)

#### Scenario: guards reuse rather than duplicate

- **WHEN** any guard module under `crates/conformance/src/guard/` is inspected
- **THEN** it SHALL obtain file paths and parsed syntax trees via the shared helpers in `guard/mod.rs`, not via its own walker or `syn::parse_file` call

#### Scenario: unsafe-policy guard enumerates manifests

- **WHEN** the unsafe-policy guard checks root and member lint configuration
- **THEN** it uses shared workspace-root, manifest-reading and crate-manifest
  enumeration helpers rather than maintaining a second member list or walker
