## Purpose

`identus-conformance` is the workspace's self-policing test crate. It encodes the workspace's structural invariants (the layer/dependency-direction rules and the port/adapter naming rules) as a static rulebook plus a set of `#[cfg(test)]` guards that parse manifests and source and assert conformance. This capability governs how the conformance crate itself is structured: the separation of invariant data (the rulebook) from enforcement logic (the guards), the one-module-per-guard layout under `guard/`, the shared helpers, and the dev-only placement of source-scanning dependencies such as `syn`.
## Requirements
### Requirement: Rulebook data is separated from guard logic

`identus-conformance` SHALL separate its invariant *data* (the rulebook — the static `const` structures encoding workspace invariants, e.g. `Layer`, `Member`, `LayerRule`, and `LAYER_RULES`) from its enforcement *logic* (the guards — `#[cfg(test)]` modules that read manifests/source and assert conformance). The rulebook data SHALL live in `crates/conformance/src/rulebook.rs` (a single file until a second rulebook justifies a `rulebook/` directory), SHALL be `pub(crate)`, and SHALL be runtime-available (NOT gated by `#[cfg(test)]`), so it is the referenceable source of truth for guards. The guard logic SHALL live under `crates/conformance/src/guard/` and SHALL be `#[cfg(test)]`. `crates/conformance/src/lib.rs` SHALL be a thin root containing only the crate doc-comment, the `pub const COMPONENT`, `mod` declarations, and re-exports of rulebook items needed by guards; it SHALL NOT contain invariant data or guard logic.

#### Scenario: the rulebook is a separate module

- **WHEN** `crates/conformance/src/` is inspected
- **THEN** it SHALL contain `rulebook.rs` holding `Layer`, `Member`, `LayerRule`, and `LAYER_RULES`, and `lib.rs` SHALL NOT define those items inline

#### Scenario: the rulebook is pub(crate) and runtime-available

- **WHEN** `crates/conformance/src/rulebook.rs` is inspected
- **THEN** `LAYER_RULES` SHALL be declared `pub(crate) const` and SHALL NOT be gated by `#[cfg(test)]`

#### Scenario: lib.rs is thin

- **WHEN** `crates/conformance/src/lib.rs` is inspected
- **THEN** it SHALL contain only the crate doc-comment, `pub const COMPONENT`, `mod rulebook;`/`mod guard;` declarations, and re-exports of rulebook items; it SHALL NOT contain invariant data definitions or guard test functions

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

### Requirement: syn is a dev-dependency of the conformance crate

`identus-conformance` SHALL declare `syn` under `[dev-dependencies]` in `crates/conformance/Cargo.toml`, referenced via `syn.workspace = true` (per `workspace-dependency-conventions`). `syn` SHALL NOT appear under `[dependencies]`; the production crate's dependency graph SHALL be unchanged by source-scanning guards, which run only under `#[cfg(test)]`.

#### Scenario: syn is dev-only

- **WHEN** `crates/conformance/Cargo.toml` is inspected
- **THEN** `syn` SHALL appear under `[dev-dependencies]` as `syn.workspace = true` and SHALL NOT appear under `[dependencies]`

#### Scenario: the production build has no syn

- **WHEN** `cargo build -p identus-conformance` is run (without `--tests`)
- **THEN** the build SHALL NOT pull `syn` into the production dependency graph of `identus-conformance`
