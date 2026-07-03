## RENAMED Requirements

- FROM: `### Requirement: Twelve crate stubs at minimal code depth`
- TO: `### Requirement: Crate stubs at minimal code depth`

## MODIFIED Requirements

### Requirement: Crate stubs at minimal code depth

The workspace SHALL contain the following runtime crate stubs, each a placeholder at minimal code depth: the founding runtime stubs — `identus-crypto`, `identus-did`, `identus-trust`, `identus-credentials`, `identus-presentations`, `identus-messaging`, `identus-openid4vc`, `identus-wallet`, `identus-agent`, `identus-adapters`, `identus-bindings`, `identus-conformance` — plus the adapter-family stub `identus-adapters-entropy` (the first of the `identus-adapters-<family>` crates; see "Adapter-family crates in the outer-boundary layer"). Additional `identus-adapters-<family>` stubs created under that convention are members of this stub set without further edit to this requirement. `identus-core` is the foundation (it owns the `Component` type and the redaction-safe error contract) and is not counted among the stubs. No fixed total stub count is normative; the stub population is the founding runtime stubs above plus each `identus-adapters-<family>` stub admitted by the adapter-family convention. Each stub's `src/lib.rs` SHALL contain only a module doc-comment and a `pub const COMPONENT: identus_core::Component` with a stable `name` and `summary`.

#### Scenario: Each stub self-describes via COMPONENT

- **WHEN** `<crate>::COMPONENT.name` is inspected for each stub in the set (the founding runtime stubs plus each `identus-adapters-<family>` stub admitted by the adapter-family convention)
- **THEN** it SHALL equal the crate's `identus-<name>` package name

#### Scenario: Stubs compile with only COMPONENT

- **WHEN** `cargo build --workspace` is run
- **THEN** every stub SHALL compile with its `lib.rs` containing only a doc-comment and the `COMPONENT` const

### Requirement: Workspace dependency map

The root `Cargo.toml` SHALL include a `[workspace.dependencies]` block mapping every workspace crate — every member of `LAYER_RULES` — to its `path = "crates/<name>"`, so every crate can express `identus-X.workspace = true`. The set of mapped crates SHALL be exactly the `LAYER_RULES` membership; no fixed literal crate count is normative, and the count of mapped `identus-*` crates SHALL equal the `LAYER_RULES` member count.

#### Scenario: All LAYER_RULES crates are workspace dependencies

- **WHEN** the root `Cargo.toml` `[workspace.dependencies]` is inspected
- **THEN** every crate in `LAYER_RULES` SHALL be present with a correct `path` entry, and the count of mapped `identus-*` crates SHALL equal the `LAYER_RULES` member count

## ADDED Requirements

### Requirement: Adapter-family crates in the outer-boundary layer

The `outer-boundary` layer SHALL consist of the retained `identus-adapters` crate (cross-cutting/facade home for adapter concerns that do not belong to a single port family) plus zero or more `identus-adapters-<family>` crates, each owning the concrete adapters for a single port family (e.g. `entropy`, `storage`, `transport`, `resolver`, `openid4vc`). Each `identus-adapters-<family>` crate SHALL (a) follow the `identus-adapters-<family>` naming convention; (b) expose `default = []` with each concrete adapter behind its own cargo feature; and (c) NOT depend on any other `identus-adapters-<family'>` crate (no cross-family coupling).

#### Scenario: an adapter-family crate is named for its port family

- **WHEN** a new outer-boundary adapter crate is created for the entropy port family
- **THEN** it SHALL be named `identus-adapters-entropy`

#### Scenario: adapter-family crates ship no adapter by default

- **WHEN** `cargo build -p <identus-adapters-family>` is run with default features
- **THEN** no concrete adapter and no adapter-specific external dependency SHALL be compiled

#### Scenario: adapter-family crates do not cross-couple

- **WHEN** the `[dependencies]` of any `identus-adapters-<family>` crate is inspected
- **THEN** it SHALL NOT contain a dependency on another `identus-adapters-<family'>` crate

### Requirement: identus-adapters-entropy crate

The workspace SHALL contain an `identus-adapters-entropy` crate as the first adapter-family crate, a member of the `outer-boundary` layer. It SHALL be registered in root `[workspace.dependencies]` (with `path = "crates/adapters-entropy"`) and in `LAYER_RULES` `outer-boundary` membership. Its `src/lib.rs` SHALL contain only a module doc-comment and a `pub const COMPONENT: identus_core::Component` (with `name = "identus-adapters-entropy"`) at the same minimal code depth as the baseline stubs. Its `Cargo.toml` SHALL depend on `identus-crypto` (inward, to implement `identus_crypto::SecureRandom`) and `identus-core`, declare `ring` as an optional external dependency behind a `ring` cargo feature, and set `default = []`. The `ring`-backed `SecureRandom` adapter *content* is filled by `add-crypto-capability` (the consumer); this change creates the crate skeleton only.

#### Scenario: identus-adapters-entropy is a workspace and layer member

- **WHEN** root `Cargo.toml` `[workspace.dependencies]` and `LAYER_RULES` are inspected
- **THEN** `identus-adapters-entropy` SHALL appear in both, in the `outer-boundary` layer

#### Scenario: identus-adapters-entropy declares ring optional behind a feature

- **WHEN** `crates/adapters-entropy/Cargo.toml` is inspected
- **THEN** `ring` SHALL be `{ workspace = true, optional = true }`, gated by a `ring` feature, with `default = []`

#### Scenario: identus-adapters-entropy depends only inward

- **WHEN** `crates/adapters-entropy/Cargo.toml` workspace-internal `[dependencies]` is inspected
- **THEN** it SHALL contain only `identus-crypto` and `identus-core` (plus the optional external `ring`)

### Requirement: Adapter-family crates are composition-root-only dependencies

No production crate (`identus-core`, `identus-crypto`, `identus-did`, `identus-trust`, `identus-credentials`, `identus-presentations`, `identus-messaging`, `identus-openid4vc`, `identus-wallet`, `identus-agent`) SHALL depend on any `identus-adapters-<family>` crate. Adapter-family crates SHALL be consumed only by binaries, examples, and `identus-bindings` (the composition root). This restates the inward-direction policy for adapter-family crates specifically.

#### Scenario: a domain crate depending on an adapter-family crate is rejected

- **WHEN** `identus-crypto` (domain-primitives) declares `[dependencies] identus-adapters-entropy.workspace = true`
- **THEN** the conformance guard SHALL fail

### Requirement: Conformance guard derives workspace crate count and outer-set from LAYER_RULES

The `identus-conformance` guard SHALL derive the expected workspace crate count and the set of outer/verification crates from `LAYER_RULES` membership rather than from hard-coded literals or a hand-maintained `OUTER_CRATES` list, so that additional `identus-adapters-<family>` members are admitted by adding them to `LAYER_RULES` and root `[workspace.dependencies]` without editing the guard's assertions. The guard SHALL treat every `identus-adapters-<family>` member as an `outer-boundary` crate subject to the inward-direction policy.

#### Scenario: adding an adapter-family crate keeps the guard green

- **WHEN** `identus-adapters-entropy` is added to `LAYER_RULES` `outer-boundary` membership and root `[workspace.dependencies]`, and its manifest conforms
- **THEN** `cargo test -p identus-conformance` SHALL pass without editing any hard-coded crate-count literal

#### Scenario: the guard recognizes an adapter-family crate as outer

- **WHEN** the guard evaluates whether a production crate depends on `identus-adapters-entropy`
- **THEN** it SHALL treat `identus-adapters-entropy` as an outer-boundary crate (via `LAYER_RULES` membership) and reject the edge