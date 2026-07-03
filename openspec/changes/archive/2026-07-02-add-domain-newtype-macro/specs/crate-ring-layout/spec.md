## MODIFIED Requirements

### Requirement: In-source layer rulebook

`identus-conformance` SHALL encode the layer rules as an in-source `pub(crate) const LAYER_RULES` (typed Rust data), porting the seed's `layer_rules` and `allowed_target_layers_by_source_layer`: the 7 layers (foundation, domain-primitives, credential-semantics, protocol-semantics, orchestration, outer-boundary, verification), each layer's `identus-*` crate membership, and each source layer's allowed inward target layers. The rulebook SHALL be the guard's source of truth for layer membership and direction. Each member entry SHALL carry a `proc_macro: bool` flag indicating whether the crate is a proc-macro crate (`[lib] proc-macro = true`); `proc_macro = true` crates are exempt from the inward-direction policy (see "Rust dep-graph guard enforces layer rules"). The `identus-derive` crate SHALL be a foundation-layer member flagged `proc_macro = true`. No `.json` fixture file SHALL be introduced for this purpose.

#### Scenario: Rulebook defines all 7 layers and all workspace crates

- **WHEN** `LAYER_RULES` is inspected
- **THEN** it SHALL list the foundation, domain-primitives, credential-semantics, protocol-semantics, orchestration, outer-boundary, and verification layers, and every `identus-*` workspace crate SHALL appear in exactly one layer's membership, and `identus-derive` SHALL appear in the foundation layer flagged `proc_macro = true`

#### Scenario: Rulebook carries a proc_macro flag per member

- **WHEN** a member entry in `LAYER_RULES` is inspected
- **THEN** it SHALL expose a `proc_macro: bool` field; `identus-derive`'s entry SHALL set it `true` and every other existing member SHALL set it `false`

#### Scenario: Rulebook encodes the seed's inward-direction policy

- **WHEN** `LAYER_RULES`'s allowed-inward lists are inspected
- **THEN** each source layer's allowed target layers SHALL match the seed's `allowed_target_layers_by_source_layer` (foundation → none; domain-primitives → foundation + domain-primitives; credential-semantics → those plus credential-semantics; protocol-semantics → those plus protocol-semantics; orchestration → those plus orchestration; outer-boundary → foundation through orchestration; verification → foundation)

### Requirement: Rust dep-graph guard enforces layer rules

`identus-conformance` SHALL contain a `#[test]` that reads `crates/*/Cargo.toml` and the root `Cargo.toml` (via the `toml` crate), identifies workspace-internal dependencies by membership in the root `[workspace.dependencies]`, and asserts every workspace-internal `[dependencies]` edge obeys the layer rules encoded in the in-source `LAYER_RULES` const. The guard SHALL treat any dependency edge whose target crate is flagged `proc_macro = true` in `LAYER_RULES` as exempt from the inward-direction policy: such an edge SHALL be permitted regardless of the source crate's layer. The guard's workspace-crate-count assertions (the total member count across `LAYER_RULES` and the count of `crates/*/Cargo.toml` manifests) SHALL be derived from `LAYER_RULES` membership (no hard-coded literal), and SHALL account for `identus-derive` as a workspace crate (a foundation `proc_macro = true` member) alongside every other `LAYER_RULES` member. The guard SHALL read no `.json` file and SHALL invoke no subprocess. The guard SHALL run through the existing crane `rust-test` nix check with no Node, no `serde_json`, and no nix config change.

#### Scenario: Guard permits a dependency on a proc-macro crate from any layer

- **WHEN** `identus-core` (foundation) declares `[dependencies] identus-derive.workspace = true` and `identus-derive` is flagged `proc_macro = true` in `LAYER_RULES`
- **THEN** the guard test SHALL pass (the edge is exempt from `foundation → none`)

#### Scenario: Guard still rejects a non-proc-macro outward dependency from foundation

- **WHEN** `identus-core` (foundation) declares `[dependencies] identus-did.workspace = true` and `identus-did` is NOT flagged `proc_macro = true`
- **THEN** the guard test SHALL fail (the edge violates `foundation → none`)

#### Scenario: Guard passes on a conforming ring with the proc-macro member

- **WHEN** `cargo test -p identus-conformance` is run and the manifests conform to `LAYER_RULES` (including `identus-derive` as a `proc_macro = true` foundation member)
- **THEN** the guard test SHALL pass

#### Scenario: Guard fails on an outward dependency to a non-proc-macro crate

- **WHEN** a domain crate (e.g. `identus-did`) declares a `[dependencies]` entry on an outer-boundary crate (e.g. `identus-adapters`) that is NOT flagged `proc_macro = true`
- **THEN** the guard test SHALL fail

#### Scenario: Guard fails when a production crate depends on conformance

- **WHEN** any production crate declares a `[dependencies]` entry on `identus-conformance`
- **THEN** the guard test SHALL fail

### Requirement: Crate stubs at minimal code depth

The workspace SHALL contain the following runtime crate stubs, each a placeholder at minimal code depth: the founding runtime stubs — `identus-crypto`, `identus-did`, `identus-trust`, `identus-credentials`, `identus-presentations`, `identus-messaging`, `identus-openid4vc`, `identus-wallet`, `identus-agent`, `identus-adapters`, `identus-bindings`, `identus-conformance` — plus the adapter-family stub `identus-adapters-entropy` (the first of the `identus-adapters-<family>` crates; see "Adapter-family crates in the outer-boundary layer"). Additional `identus-adapters-<family>` stubs created under that convention are members of this stub set without further edit to this requirement. `identus-core` is the foundation (it owns the `Component` type and the redaction-safe error contract) and is not counted among the stubs. `identus-derive` is a `proc-macro = true` build-time crate (see "In-source layer rulebook") and is NOT counted among the runtime stubs. No fixed total stub count is normative; the stub population is the founding runtime stubs above plus each `identus-adapters-<family>` stub admitted by the adapter-family convention. Each runtime stub's `src/lib.rs` SHALL contain only a module doc-comment and a `pub const COMPONENT: identus_core::Component` with a stable `name` and `summary`.

#### Scenario: Each stub self-describes via COMPONENT

- **WHEN** `<crate>::COMPONENT.name` is inspected for each stub in the set (the founding runtime stubs plus each `identus-adapters-<family>` stub admitted by the adapter-family convention)
- **THEN** it SHALL equal the crate's `identus-<name>` package name

#### Scenario: Stubs compile with only COMPONENT

- **WHEN** `cargo build --workspace` is run
- **THEN** every runtime stub SHALL compile with its `lib.rs` containing only a doc-comment and the `COMPONENT` const

### Requirement: Workspace dependency map

The root `Cargo.toml` SHALL include a `[workspace.dependencies]` block mapping every workspace crate — every member of `LAYER_RULES` — to its `path = "crates/<name>"`, so every crate can express `identus-X.workspace = true`. The set of mapped crates SHALL be exactly the `LAYER_RULES` membership (which includes `identus-core`, the runtime stubs, the adapter-family stubs, `identus-derive`, and `identus-conformance`); no fixed literal crate count is normative, and the count of mapped `identus-*` crates SHALL equal the `LAYER_RULES` member count.

#### Scenario: All LAYER_RULES crates are workspace dependencies

- **WHEN** the root `Cargo.toml` `[workspace.dependencies]` is inspected
- **THEN** every crate in `LAYER_RULES` SHALL be present with a correct `path` entry — including the proc-macro crate `identus-derive = { path = "crates/derive" }` and the adapter-family crate `identus-adapters-entropy = { path = "crates/adapters-entropy" }` — and the count of mapped `identus-*` crates SHALL equal the `LAYER_RULES` member count

### Requirement: Foundation has no workspace dependencies

`identus-core` SHALL declare no `identus-*` runtime dependencies. The guard SHALL assert this, treating any `[dependencies]` entry on a workspace crate flagged `proc_macro = true` (e.g. `identus-derive`) as exempt and therefore not a violation of this requirement.

#### Scenario: Core is runtime-dependency-free

- **WHEN** the guard inspects `crates/core/Cargo.toml`
- **THEN** it SHALL assert there are no `identus-*` dependencies except those whose target is flagged `proc_macro = true` in `LAYER_RULES`

#### Scenario: Core may depend on identus-derive

- **WHEN** `crates/core/Cargo.toml` declares `identus-derive.workspace = true` and `identus-derive` is `proc_macro = true`
- **THEN** the guard SHALL pass this requirement