## Purpose

The `sdk-rust` workspace is a hexagonal ring of 13 runtime crates — domain and protocol crates define primitives and ports; adapter-family crates and the composition root sit outside those semantics; conformance validates them and is never depended on by production — plus `identus-derive`, a build-time `proc-macro = true` crate admitted to the foundation layer (see "In-source layer rulebook") and excluded from the runtime ring. Dependency direction is inward and is enforced from the moment any real code lands. This capability's enduring rules are:

- **Foundation is dependency-free.** `identus-core` has no `identus-*` workspace dependencies and no dependency on product, protocol, adapter, binding, or conformance crates.
- **Dependency direction is inward.** Domain crates depend only on foundation and other domain-primitive crates. Credential/protocol/orchestration crates depend on their inner rings. Adapters and bindings may depend on stable domain/protocol/wallet crates, but domain crates must not depend back on adapters, bindings, or services.
- **Adapters, bindings, and conformance sit outside domain semantics.** Production crates (`core`, `crypto`, `did`, `trust`, `credentials`, `presentations`, `messaging`, `openid4vc`, `wallet`, `agent`) SHALL NOT depend on any `identus-adapters-<family>` crate, `identus-bindings`, or `identus-conformance`.
- **Conformance is never a production dependency.** `identus-conformance` may use dev-dependencies to validate contracts, but production crates must not depend on it.
- **Layer membership is the contract.** The in-source `LAYER_RULES` const defines which crates belong to which layer; the guard asserts the manifests conform. The layers are:

| Layer | Crates |
|---|---|
| foundation | `identus-core`, `identus-derive` |
| domain-primitives | `identus-crypto`, `identus-did`, `identus-trust` |
| credential-semantics | `identus-credentials`, `identus-presentations` |
| protocol-semantics | `identus-messaging`, `identus-openid4vc` |
| orchestration | `identus-wallet`, `identus-agent` |
| outer-boundary | `identus-adapters-entropy`, `identus-bindings` |
| verification | `identus-conformance` |

## Requirements

### Requirement: Crate stubs at minimal code depth

The workspace SHALL contain the following runtime crate stubs, each a placeholder at minimal code depth: the founding runtime stubs — `identus-crypto`, `identus-did`, `identus-trust`, `identus-credentials`, `identus-presentations`, `identus-messaging`, `identus-openid4vc`, `identus-wallet`, `identus-agent`, `identus-bindings`, `identus-conformance` — plus the adapter-family stub `identus-adapters-entropy` (the first of the `identus-adapters-<family>` crates; see "Adapter-family crates in the outer-boundary layer"). Additional `identus-adapters-<family>` stubs created under that convention are members of this stub set without further edit to this requirement. `identus-core` is the foundation (it owns the `Component` type and the redaction-safe error contract) and is not counted among the stubs. `identus-derive` is a `proc-macro = true` build-time crate (see "In-source layer rulebook") and is NOT counted among the runtime stubs. No fixed total stub count is normative; the stub population is the founding runtime stubs above plus each `identus-adapters-<family>` stub admitted by the adapter-family convention. Each runtime stub's `src/lib.rs` SHALL contain only a module doc-comment and a `pub const COMPONENT: identus_core::Component` with a stable `name` and `summary`.

#### Scenario: Each stub self-describes via COMPONENT

- **WHEN** `<crate>::COMPONENT.name` is inspected for each stub in the set (the founding runtime stubs plus each `identus-adapters-<family>` stub admitted by the adapter-family convention)
- **THEN** it SHALL equal the crate's `identus-<name>` package name

#### Scenario: Stubs compile with only COMPONENT

- **WHEN** `cargo build --workspace` is run
- **THEN** every stub SHALL compile with its `lib.rs` containing only a doc-comment and the `COMPONENT` const

### Requirement: Full intended inward dependency edges

Each stub's `Cargo.toml` SHALL declare its full intended inward dependency edges (matching the layer rules), even though the minimal `lib.rs` only uses `identus-core`. Unused crate dependencies SHALL remain silent under the workspace's `warnings = "deny"` policy.

#### Scenario: identus-trust declares its inward edges

- **WHEN** `crates/trust/Cargo.toml` is inspected
- **THEN** its `[dependencies]` SHALL include `identus-core`, `identus-crypto`, and `identus-did` (and no outward crates)

#### Scenario: Unused inward deps do not fail the build

- **WHEN** `cargo build --workspace` and `cargo clippy -- -D warnings` are run
- **THEN** the stubs SHALL pass despite declaring deps their minimal `lib.rs` does not yet call

### Requirement: Workspace dependency map

The root `Cargo.toml` SHALL include a `[workspace.dependencies]` block mapping every workspace crate — every member of `LAYER_RULES` — to its `path = "crates/<name>"`, so every crate can express `identus-X.workspace = true`. The set of mapped crates SHALL be exactly the `LAYER_RULES` membership (which includes `identus-core`, the runtime stubs, the adapter-family stubs, `identus-derive`, and `identus-conformance`); no fixed literal crate count is normative, and the count of mapped `identus-*` crates SHALL equal the `LAYER_RULES` member count.

#### Scenario: All LAYER_RULES crates are workspace dependencies

- **WHEN** the root `Cargo.toml` `[workspace.dependencies]` is inspected
- **THEN** every crate in `LAYER_RULES` SHALL be present with a correct `path` entry — including the proc-macro crate `identus-derive = { path = "crates/derive" }` and the adapter-family crate `identus-adapters-entropy = { path = "crates/adapters-entropy" }` — and the count of mapped `identus-*` crates SHALL equal the `LAYER_RULES` member count

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

- **WHEN** a domain crate (e.g. `identus-did`) declares a `[dependencies]` entry on an outer-boundary crate (e.g. `identus-adapters-entropy`) that is NOT flagged `proc_macro = true`
- **THEN** the guard test SHALL fail

#### Scenario: Guard fails when a production crate depends on conformance

- **WHEN** any production crate declares a `[dependencies]` entry on `identus-conformance`
- **THEN** the guard test SHALL fail

### Requirement: Foundation has no workspace dependencies

`identus-core` SHALL declare no `identus-*` runtime dependencies. The guard SHALL assert this, treating any `[dependencies]` entry on a workspace crate flagged `proc_macro = true` (e.g. `identus-derive`) as exempt and therefore not a violation of this requirement.

#### Scenario: Core is runtime-dependency-free

- **WHEN** the guard inspects `crates/core/Cargo.toml`
- **THEN** it SHALL assert there are no `identus-*` dependencies except those whose target is flagged `proc_macro = true` in `LAYER_RULES`

#### Scenario: Core may depend on identus-derive

- **WHEN** `crates/core/Cargo.toml` declares `identus-derive.workspace = true` and `identus-derive` is `proc_macro = true`
- **THEN** the guard SHALL pass this requirement

### Requirement: No cross-crate dev-dependencies in the conformance crate yet

`identus-conformance` SHALL depend on `identus-core` (production, for `COMPONENT`) and on `toml` (dev, for the guard's manifest parsing). It SHALL NOT declare `serde_json` or any `identus-*` domain crate as a dev-dependency, since the rulebook is an in-source `const` (no JSON parsing) and the domain crates are stubs with no public contracts to drift against.

#### Scenario: Conformance dev-deps are tooling only

- **WHEN** `crates/conformance/Cargo.toml` is inspected
- **THEN** its `[dev-dependencies]` SHALL include `toml` only and SHALL NOT include `serde_json` or any `identus-*` domain crate

### Requirement: No docs/architecture files introduced

This change SHALL NOT create any `docs/architecture/` files. The ring rules live in this spec's `## Purpose` and requirements; per-change decisions in `design.md`.

#### Scenario: No docs/architecture directory is created

- **WHEN** the change's file additions are inspected
- **THEN** no path under `docs/architecture/` SHALL be present

### Requirement: Dep-graph guard rejects inline external dependency versions

The `identus-conformance` dep-graph guard (the `#[test]` module in `crates/conformance/src/lib.rs`) SHALL, in the same manifest-reading pass that enforces the `LAYER_RULES` inward-direction policy, also enumerate the top-level `[dependencies]`, `[dev-dependencies]`, and `[build-dependencies]` tables of each `crates/*/Cargo.toml`, together with their target-specific `[target.'cfg(...)'.dependencies]`, `[target.'cfg(...)'.dev-dependencies]`, and `[target.'cfg(...)'.build-dependencies]` variants, and SHALL assert that no external (non-`workspace.internal`, non-`path`) entry in any of those sections pins `version = "..."` inline. Every external entry SHALL be expressed as `<dep>.workspace = true` resolving to a root `[workspace.dependencies]` entry. `optional = true`, `build`, and target-specific external entries SHALL be treated identically to plain `[dependencies]` entries for this check. The guard SHALL read no `.json` file, SHALL invoke no subprocess, and SHALL introduce no Node, no `serde_json`, and no nix config change; it SHALL continue to run through the existing crane `rust-test` nix check.

#### Scenario: Guard passes when external deps use the workspace form

- **WHEN** `cargo test -p identus-conformance` is run and every external dependency in every crate manifest is expressed as `<dep>.workspace = true` resolving to a root `[workspace.dependencies]` entry
- **THEN** the guard test SHALL pass

#### Scenario: Guard fails on an inline external version in dependencies

- **WHEN** a crate manifest's `[dependencies]` declares an external crate with an inline `version = "..."` (e.g. `toml = "0.8"`)
- **THEN** the guard test SHALL fail

#### Scenario: Guard fails on an inline external version in dev-dependencies

- **WHEN** a crate manifest's `[dev-dependencies]` declares an external crate with an inline `version = "..."` (e.g. `toml = "0.8"`)
- **THEN** the guard test SHALL fail

#### Scenario: Guard fails on an inline external version in build-dependencies

- **WHEN** a crate manifest's `[build-dependencies]` declares an external crate with an inline `version = "..."`
- **THEN** the guard test SHALL fail

#### Scenario: Guard fails on an inline external version in a target-specific dependencies section

- **WHEN** a crate manifest's `[target.'cfg(...)'.dependencies]` declares an external crate with an inline `version = "..."`
- **THEN** the guard test SHALL fail

#### Scenario: Guard fails on an inline external version in a target-specific dev-dependencies section

- **WHEN** a crate manifest's `[target.'cfg(...)'.dev-dependencies]` declares an external crate with an inline `version = "..."`
- **THEN** the guard test SHALL fail

#### Scenario: Guard fails on an inline external version in a target-specific build-dependencies section

- **WHEN** a crate manifest's `[target.'cfg(...)'.build-dependencies]` declares an external crate with an inline `version = "..."`
- **THEN** the guard test SHALL fail

#### Scenario: Guard accepts an optional workspace-declared external entry

- **WHEN** a crate manifest declares `ed25519-dalek = { workspace = true, optional = true }` and the root `[workspace.dependencies]` contains the `ed25519-dalek` entry with its `version`
- **THEN** the guard test SHALL pass

#### Scenario: Guard fails on an inline optional external entry

- **WHEN** a crate manifest declares `ed25519-dalek = { version = "2.0", optional = true }`
- **THEN** the guard test SHALL fail

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