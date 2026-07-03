## Purpose

The `sdk-rust` workspace is a 13-crate hexagonal ring: domain and protocol crates define primitives and ports; adapters and bindings sit outside those semantics; conformance validates them and is never depended on by production. Dependency direction is inward and is enforced from the moment any real code lands. This capability's enduring rules are:

- **Foundation is dependency-free.** `identus-core` has no `identus-*` workspace dependencies and no dependency on product, protocol, adapter, binding, or conformance crates.
- **Dependency direction is inward.** Domain crates depend only on foundation and other domain-primitive crates. Credential/protocol/orchestration crates depend on their inner rings. Adapters and bindings may depend on stable domain/protocol/wallet crates, but domain crates must not depend back on adapters, bindings, or services.
- **Adapters, bindings, and conformance sit outside domain semantics.** Production crates (`core`, `crypto`, `did`, `trust`, `credentials`, `presentations`, `messaging`, `openid4vc`, `wallet`, `agent`) SHALL NOT depend on `identus-adapters`, `identus-bindings`, or `identus-conformance`.
- **Conformance is never a production dependency.** `identus-conformance` may use dev-dependencies to validate contracts, but production crates must not depend on it.
- **Layer membership is the contract.** The in-source `LAYER_RULES` const defines which crates belong to which layer; the guard asserts the manifests conform. The layers are:

| Layer | Crates |
|---|---|
| foundation | `identus-core` |
| domain-primitives | `identus-crypto`, `identus-did`, `identus-trust` |
| credential-semantics | `identus-credentials`, `identus-presentations` |
| protocol-semantics | `identus-messaging`, `identus-openid4vc` |
| orchestration | `identus-wallet`, `identus-agent` |
| outer-boundary | `identus-adapters`, `identus-bindings` |
| verification | `identus-conformance` |

## ADDED Requirements

### Requirement: Twelve crate stubs at minimal code depth

The workspace SHALL contain 12 additional crate stubs (besides `identus-core`): `identus-crypto`, `identus-did`, `identus-trust`, `identus-credentials`, `identus-presentations`, `identus-messaging`, `identus-openid4vc`, `identus-wallet`, `identus-agent`, `identus-adapters`, `identus-bindings`, `identus-conformance`. Each stub's `src/lib.rs` SHALL contain only a module doc-comment and a `pub const COMPONENT: identus_core::Component` with a stable `name` and `summary`.

#### Scenario: Each stub self-describes via COMPONENT

- **WHEN** `<crate>::COMPONENT.name` is inspected for each of the 12 stubs
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

The root `Cargo.toml` SHALL include a `[workspace.dependencies]` block with all 13 crates mapped to their `path = "crates/<name>"`, so every stub can express `identus-X.workspace = true`.

#### Scenario: All 13 crates are workspace dependencies

- **WHEN** the root `Cargo.toml` `[workspace.dependencies]` is inspected
- **THEN** all 13 `identus-*` crates SHALL be present with correct `path` entries

### Requirement: In-source layer rulebook

`identus-conformance` SHALL encode the layer rules as an in-source `pub(crate) const LAYER_RULES` (typed Rust data), porting the seed's `layer_rules` and `allowed_target_layers_by_source_layer`: the 7 layers (foundation, domain-primitives, credential-semantics, protocol-semantics, orchestration, outer-boundary, verification), each layer's `identus-*` crate membership, and each source layer's allowed inward target layers. The rulebook SHALL be the guard's source of truth for layer membership and direction. No `.json` fixture file SHALL be introduced for this purpose.

#### Scenario: Rulebook defines all 7 layers and all 13 crates

- **WHEN** `LAYER_RULES` is inspected
- **THEN** it SHALL list the foundation, domain-primitives, credential-semantics, protocol-semantics, orchestration, outer-boundary, and verification layers, and every `identus-*` crate SHALL appear in exactly one layer's membership

#### Scenario: Rulebook encodes the seed's inward-direction policy

- **WHEN** `LAYER_RULES`'s allowed-inward lists are inspected
- **THEN** each source layer's allowed target layers SHALL match the seed's `allowed_target_layers_by_source_layer` (foundation → none; domain-primitives → foundation + domain-primitives; credential-semantics → those plus credential-semantics; protocol-semantics → those plus protocol-semantics; orchestration → those plus orchestration; outer-boundary → foundation through orchestration; verification → foundation)

### Requirement: Rust dep-graph guard enforces layer rules

`identus-conformance` SHALL contain a `#[test]` that reads `crates/*/Cargo.toml` and the root `Cargo.toml` (via the `toml` crate), identifies workspace-internal dependencies by membership in the root `[workspace.dependencies]`, and asserts every workspace-internal `[dependencies]` edge obeys the layer rules encoded in the in-source `LAYER_RULES` const. The guard SHALL read no `.json` file and SHALL invoke no subprocess. The guard SHALL run through the existing crane `rust-test` nix check with no Node, no `serde_json`, and no nix config change.

#### Scenario: Guard passes on a conforming ring

- **WHEN** `cargo test -p identus-conformance` is run and the manifests conform to `LAYER_RULES`
- **THEN** the guard test SHALL pass

#### Scenario: Guard fails on an outward dependency

- **WHEN** a domain crate (e.g. `identus-did`) declares a `[dependencies]` entry on an outer-boundary crate (e.g. `identus-adapters`)
- **THEN** the guard test SHALL fail

#### Scenario: Guard fails when a production crate depends on conformance

- **WHEN** any production crate declares a `[dependencies]` entry on `identus-conformance`
- **THEN** the guard test SHALL fail

### Requirement: Foundation has no workspace dependencies

`identus-core` SHALL declare no `identus-*` dependencies. The guard SHALL assert this.

#### Scenario: Core is dependency-free

- **WHEN** the guard inspects `crates/core/Cargo.toml`
- **THEN** it SHALL assert there are no `identus-*` dependencies

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