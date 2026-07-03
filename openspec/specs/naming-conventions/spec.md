## Purpose

The Identus Rust SDK follows a ports-and-adapters (hexagonal) architecture: domain/protocol crates define *port* traits (infrastructure interfaces), and outer-boundary `identus-adapters-<family>` crates provide concrete *adapter* structs that implement those ports. This capability codifies the naming conventions that keep the port/adapter vocabulary uniform across the workspace: port traits are bare capability nouns with no suffix, port-ness is declared by the `#[identus::port]` proc-macro attribute (the single source of truth, with no hand-maintained port registry), port-owning crates depend on the proc-macro attribute provider, and adapter structs use the `<Backend><Capability>Adapter` form. The conventions are enforced by a compile-time attribute check (for the port-name suffix) and a test-time naming guard (for the adapter suffix).

## Requirements

### Requirement: Port traits are bare capability nouns with no suffix

The Identus Rust SDK SHALL name port traits (the infrastructure interfaces implemented by outer-boundary adapters) as bare capability nouns with **no suffix** — e.g. `SecureRandom`, `Storage`, `Transport`, `Resolver`, `Clock`. A port trait name SHALL NOT end in `Port` (the `Port` suffix is explicitly forbidden, to prevent drift toward the Java-style `*Port`/`*Adapter` mismatch). Port traits are `pub trait` items defined in domain or protocol crates (inner rings), never in `identus-adapters-*` crates (that would be a layer violation already rejected by `crate-ring-layout`).

#### Scenario: a port trait has no suffix

- **WHEN** a port trait is defined in a domain or protocol crate
- **THEN** its name SHALL be a bare capability noun (e.g. `SecureRandom`) and SHALL NOT end in `Port`

#### Scenario: a Port-suffixed port trait is rejected at compile time

- **WHEN** a `pub trait` annotated with `#[identus::port]` is named with a trailing `Port` (e.g. `StoragePort`)
- **THEN** the `#[identus::port]` attribute macro SHALL emit a compile error, failing the build before the conformance test run

### Requirement: The `#[identus::port]` attribute declares port-ness

`identus-derive` SHALL expose an inert attribute `#[identus::port]` (a `#[proc_macro_attribute]`) that port traits are annotated with. The attribute SHALL carry no runtime semantics and SHALL NOT propagate to implementers of the trait (unlike a supertrait marker, it is attached to the trait *item*, not a `Self:` bound). The attribute SHALL parse its input as `syn::ItemTrait` and emit `compile_error!` if the trait name ends in `Port` (per the no-suffix requirement); otherwise it SHALL emit the trait unchanged. A port trait SHALL be annotated `#[identus::port]` (e.g. `pub trait SecureRandom` with `#[identus::port]` above it). The attribute is the self-documenting declaration that the trait is a port and the single source of truth from which the naming guard derives the port set; there SHALL be no hand-maintained registry of port names. The attribute SHALL be usable by any crate that depends on `identus-derive`. No `Port` marker trait SHALL be added to `identus-core` (the prior marker-supertrait approach is rejected because a supertrait bound is an implicit `Self:` bound that would force every adapter to implement the marker).

#### Scenario: the attribute is inert and lives in identus-derive

- **WHEN** `crates/derive/src/` is inspected
- **THEN** it SHALL expose `#[identus::port]` as a `#[proc_macro_attribute]` that, on a non-`*Port`-named trait, emits the trait unchanged, and on a `*Port`-named trait, emits a compile error

#### Scenario: a port declares port-ness via the attribute

- **WHEN** `crates/crypto/src/securerandom.rs` is inspected
- **THEN** the trait SHALL be declared as `#[identus::port] pub trait SecureRandom` (the attribute present above the trait)

#### Scenario: no Port marker trait is added to identus-core

- **WHEN** `crates/core/src/` is inspected
- **THEN** it SHALL NOT define a `Port` marker trait; port-ness is declared by the attribute in `identus-derive`, not by a trait in `identus-core`

### Requirement: Port-owning crates depend on the proc-macro attribute provider

A crate that defines a port trait annotated with `#[identus::port]` (a "port-owning crate" — `identus-crypto` for `SecureRandom`, and future `identus-storage`/`identus-transport`/`identus-resolver` as those port families land) SHALL depend on `identus-derive` (the proc-macro crate that provides the attribute), referenced via `identus-derive.workspace = true` under `[dependencies]`. The edge is layer-legal because `identus-derive` is a `proc_macro = true` foundation member and proc-macro crates are exempt from the inward-direction policy per `crate-ring-layout`; it is build-time only (proc-macro tooling), not a runtime dependency. This requirement is the canonical allowance that each port-owning crate's own layer-conformance requirement cites, so a port-owning crate's spec need not restate the proc-macro exemption.

#### Scenario: a port-owning crate lists identus-derive

- **WHEN** a port-owning crate's `Cargo.toml` workspace-internal `[dependencies]` is inspected
- **THEN** it SHALL list `identus-derive` (referenced via `identus-derive.workspace = true`)

#### Scenario: the proc-macro edge is layer-legal

- **WHEN** the `crate-ring-layout` dep-graph guard is run against a port-owning crate's manifest
- **THEN** the edge to `identus-derive` SHALL pass, because `identus-derive` is flagged `proc_macro = true` in `LAYER_RULES` and proc-macro targets are exempt from the inward-direction policy

### Requirement: Adapter structs use the `<Backend><Capability>Adapter` form

Adapter structs — concrete `pub struct` items in `identus-adapters-<family>` crates that `impl` a port trait — SHALL be named `<Backend><Capability>Adapter`, where `<Backend>` names the implementation backend (e.g. `Ring`, `Getrandom`, `Deterministic`) and `<Capability>` names the port's capability (e.g. `SystemRandom`). Every adapter struct name SHALL end in `Adapter`. The form preserves the capability word at injection sites and avoids introducing a second vocabulary (the `adapters-<family>` crate name) into the struct name.

#### Scenario: an adapter is named with the Adapter suffix

- **WHEN** an adapter struct implementing `SecureRandom` is defined in `identus-adapters-entropy`
- **THEN** its name SHALL match `<Backend><Capability>Adapter` (e.g. `RingSystemRandomAdapter`, `DeterministicRandomAdapter`) and SHALL end in `Adapter`

#### Scenario: a non-Adapter adapter struct is rejected

- **WHEN** a `pub struct` in an `identus-adapters-*` crate implements a port trait and its name does not end in `Adapter`
- **THEN** the naming guard SHALL fail, reporting the offending struct and the port it implements

### Requirement: Existing adapters are retrofitted to the convention

The two existing public adapters in `identus-adapters-entropy` SHALL be renamed to the `<Backend><Capability>Adapter` form: `RingSystemRandom` → `RingSystemRandomAdapter` and `DeterministicRandom` → `DeterministicRandomAdapter`. The rename SHALL update both the struct definitions and their `impl SecureRandom for …` blocks, and the module doc-comment SHALL be updated to reflect the convention. The deferred wasm adapter, when it lands in a future change, SHALL be named `GetrandomSystemRandomAdapter` from the start. No grandfathered exceptions SHALL remain.

#### Scenario: the ring adapter is renamed

- **WHEN** `crates/adapters-entropy/src/lib.rs` is inspected
- **THEN** it SHALL define `pub struct RingSystemRandomAdapter` (behind the `ring` feature) and `impl SecureRandom for RingSystemRandomAdapter`

#### Scenario: the deterministic adapter is renamed

- **WHEN** `crates/adapters-entropy/src/lib.rs` is inspected (behind the `deterministic` feature)
- **THEN** it SHALL define `pub struct DeterministicRandomAdapter` and `impl SecureRandom for DeterministicRandomAdapter`

#### Scenario: no un-suffixed adapter name remains

- **WHEN** `crates/adapters-entropy/src/lib.rs` is inspected for the identifiers `RingSystemRandom` or `DeterministicRandom` without the `Adapter` suffix
- **THEN** none SHALL appear as a struct or `impl` name

### Requirement: The naming guard derives the port set from the attribute and enforces the adapter suffix (test-time)

`identus-conformance` SHALL include a naming guard (`#[cfg(test)]`) that derives the port set from source and enforces the adapter suffix rule without a hand-maintained port registry. The guard SHALL parse workspace `crates/*/src/**/*.rs` files with `syn` (a workspace-level dependency, referenced as a dev-dependency of `identus-conformance`), inspect `syn::ItemTrait` `attrs` to discover port traits (those carrying the `#[identus::port]` attribute), and MAY redundantly assert that no discovered port trait name ends in `Port` (the authoritative port-name check is compile-time via the attribute). The guard SHALL then scan `crates/adapters-*/src/**/*.rs` for `syn::ItemImpl` blocks whose implemented trait is a discovered port and assert the implementing `pub struct`'s name ends in `Adapter`. The guard SHALL fail closed: a `syn` parse error on a scanned file SHALL fail the test rather than silently passing.

#### Scenario: the guard discovers the port set from the attribute

- **WHEN** the naming guard scans `crates/crypto/src/securerandom.rs`
- **THEN** it SHALL recognize `SecureRandom` as a port (via the `#[identus::port]` attribute on `ItemTrait`) for adapter-suffix enforcement

#### Scenario: the guard rejects an adapter struct without the suffix

- **WHEN** an `identus-adapters-*` crate defines `impl SecureRandom for RingSystemRandom` (name lacking the `Adapter` suffix)
- **THEN** the naming guard SHALL fail, reporting `RingSystemRandom` as an adapter of `SecureRandom` whose name does not end in `Adapter`

#### Scenario: a syn parse error fails the guard

- **WHEN** a scanned `crates/*/src/**/*.rs` file fails to parse with `syn::parse_file`
- **THEN** the naming guard SHALL fail (closed), surfacing the parse error rather than silently skipping the file

#### Scenario: syn is a dev-dependency only

- **WHEN** `crates/conformance/Cargo.toml` is inspected
- **THEN** `syn` SHALL appear under `[dev-dependencies]` (referenced via `syn.workspace = true`) and SHALL NOT appear under `[dependencies]`

### Requirement: The naming guard reuses the conformance crate structure

The naming guard SHALL reside at `crates/conformance/src/guard/naming.rs` and SHALL use the shared helpers (file walking, `syn` parsing) from `crates/conformance/src/guard/mod.rs` per the `conformance-crate-structure` capability. It SHALL NOT duplicate file-walking or parsing logic that already exists in the shared guard helpers.

#### Scenario: the naming guard lives in its own module

- **WHEN** `crates/conformance/src/guard/` is inspected
- **THEN** it SHALL contain `naming.rs` (the naming guard) as a sibling of `dep_graph.rs` (the existing layer/dep-direction guard), both declared from `guard/mod.rs`

#### Scenario: the naming guard reuses shared parsing helpers

- **WHEN** `crates/conformance/src/guard/naming.rs` is inspected
- **THEN** it SHALL obtain parsed `syn::File` values and workspace source file paths via the shared helpers in `guard/mod.rs`, not via its own file-walking or `syn::parse_file` calls