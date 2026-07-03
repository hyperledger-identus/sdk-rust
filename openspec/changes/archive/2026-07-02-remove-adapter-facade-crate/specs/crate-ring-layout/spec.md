## MODIFIED Requirements

### Requirement: Adapter-family crates in the outer-boundary layer

The `outer-boundary` layer SHALL consist of the `identus-adapters-<family>` leaf crates (each owning the concrete adapters for a single port family, e.g. `entropy`, `storage`, `transport`, `resolver`, `openid4vc`) plus `identus-bindings`, the composition root that wires selected adapter families into runnable artifacts. There is no catch-all `identus-adapters` crate; adapter concerns that do not belong to a single port family live in `identus-bindings` (wiring/composition), `identus-core` (shared cross-cutting types), orchestration crates, or `identus-conformance` (test fakes) as appropriate — not in a retained facade crate. Each `identus-adapters-<family>` crate SHALL (a) follow the `identus-adapters-<family>` naming convention; (b) expose `default = []` with each concrete adapter behind its own cargo feature; and (c) NOT depend on any other `identus-adapters-<family'>` crate (no cross-family coupling).

#### Scenario: an adapter-family crate is named for its port family

- **WHEN** a new outer-boundary adapter crate is created for the entropy port family
- **THEN** it SHALL be named `identus-adapters-entropy`

#### Scenario: adapter-family crates ship no adapter by default

- **WHEN** `cargo build -p <identus-adapters-family>` is run with default features
- **THEN** no concrete adapter and no adapter-specific external dependency SHALL be compiled

#### Scenario: adapter-family crates do not cross-couple

- **WHEN** the `[dependencies]` of any `identus-adapters-<family>` crate is inspected
- **THEN** it SHALL NOT contain a dependency on another `identus-adapters-<family'>` crate

#### Scenario: the outer-boundary layer is family leaves plus the composition root

- **WHEN** the `outer-boundary` layer membership in `LAYER_RULES` is inspected
- **THEN** it SHALL consist of zero or more `identus-adapters-<family>` leaf crates plus `identus-bindings` (the composition root), and SHALL NOT contain a catch-all `identus-adapters` crate

### Requirement: Crate stubs at minimal code depth

The workspace SHALL contain the following runtime crate stubs, each a placeholder at minimal code depth: the founding runtime stubs — `identus-crypto`, `identus-did`, `identus-trust`, `identus-credentials`, `identus-presentations`, `identus-messaging`, `identus-openid4vc`, `identus-wallet`, `identus-agent`, `identus-bindings`, `identus-conformance` — plus the adapter-family stub `identus-adapters-entropy` (the first of the `identus-adapters-<family>` crates; see "Adapter-family crates in the outer-boundary layer"). Additional `identus-adapters-<family>` stubs created under that convention are members of this stub set without further edit to this requirement. `identus-core` is the foundation (it owns the `Component` type and the redaction-safe error contract) and is not counted among the stubs. `identus-derive` is a `proc-macro = true` build-time crate (see "In-source layer rulebook") and is NOT counted among the runtime stubs. No fixed total stub count is normative; the stub population is the founding runtime stubs above plus each `identus-adapters-<family>` stub admitted by the adapter-family convention. Each runtime stub's `src/lib.rs` SHALL contain only a module doc-comment and a `pub const COMPONENT: identus_core::Component` with a stable `name` and `summary`.

#### Scenario: Each stub self-describes via COMPONENT

- **WHEN** `<crate>::COMPONENT.name` is inspected for each stub in the set (the founding runtime stubs plus each `identus-adapters-<family>` stub admitted by the adapter-family convention)
- **THEN** it SHALL equal the crate's `identus-<name>` package name

#### Scenario: Stubs compile with only COMPONENT

- **WHEN** `cargo build --workspace` is run
- **THEN** every stub SHALL compile with its `lib.rs` containing only a doc-comment and the `COMPONENT` const

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

- **WHEN** a domain crate (e.g. `identus-did`) declares a `[dependencies]` entry on an outer-boundary crate (e.g. `identus-adapters-entropy`) that is NOT flagged `proc_macro = true`
- **THEN** the guard test SHALL fail

#### Scenario: Guard fails when a production crate depends on conformance

- **WHEN** any production crate declares a `[dependencies]` entry on `identus-conformance`
- **THEN** the guard test SHALL fail