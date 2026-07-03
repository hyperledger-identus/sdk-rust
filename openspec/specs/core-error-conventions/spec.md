## Purpose

`identus-core` is the zero-workspace-dependency foundation crate that owns the shared error and result contract for the Identus Rust SDK workspace. Its invariants are enduring architecture principles every later crate and every future binding depends on:

- **Redaction safety is structural, not policy-driven.** `IdentusError` carries only `&'static str` fields, and its `Display` renders only a stable code and a public message. Secret or internal context never enters the value; it stays in adapter-local logs. The guarantee cannot be bypassed by a future constructor without changing the type itself.
- **Stable typed codes are a compatibility contract.** `ErrorCode` is a `&'static str` newtype; conformance fixtures and language bindings match on these codes. Changing or removing a catalogued code requires an explicit compatibility decision across TypeScript, Swift, Kotlin, Node, WASM, React, and React Native consumers.
- **Capability attribution.** `CapabilityId` names the owning capability on every error, so failures are attributable without leaking internals.
- **Two-surface bridging.** Domain crates keep an idiomatic local error type for `FromStr` and additionally expose `to_identus_error()` and `parse_with_core_error()` for the core surface. This lets Rust-idiomatic APIs coexist with a stable, cross-language error surface.
- **Deferred binding surface.** `ResultEnvelope`, `ErrorEnvelope`, `RedactionPolicy`, and `to_error_envelope()` are deliberately out of scope here; they are added by the bindings change, which is the first change with a binding consumer. Redaction safety does not depend on the deferred `RedactionPolicy` enum — it is structural.

## Requirements

### Requirement: Redaction-safe error display

The `identus-core` crate SHALL provide an `IdentusError` type whose `Display` implementation renders only a stable error code and a public message. `IdentusError` SHALL carry only `&'static str` message text and SHALL NOT store secret or internal context.

#### Scenario: Display renders code and public message only

- **WHEN** an `IdentusError` is constructed via `IdentusError::public(code, kind, capability, message)` and displayed
- **THEN** the rendered string SHALL equal `"{code}: {public_message}"` and SHALL NOT include any other field

#### Scenario: Internal error carries no capability

- **WHEN** an `IdentusError` is constructed via `IdentusError::internal(code, message)`
- **THEN** its `kind` SHALL be `ErrorKind::Internal` and its `capability` SHALL be `None`

### Requirement: Stable typed error codes

The crate SHALL provide an `ErrorCode` newtype over `&'static str` as the stable compatibility contract for conformance fixtures and bindings. `ErrorCode` SHALL be constructible via `ErrorCode::new(&'static str)` and expose `as_str()`.

#### Scenario: Error code is a stable string

- **WHEN** `ErrorCode::new("invalid_did_method")` is constructed and `as_str()` is called
- **THEN** it SHALL return `"invalid_did_method"`

### Requirement: Error families

The crate SHALL provide an `ErrorKind` enum with exactly these families: `InvalidInput`, `Unsupported`, `NotFound`, `Conflict`, `PolicyViolation`, `VerificationFailed`, `Transport`, `Storage`, `Crypto`, `Trust`, `Internal`.

#### Scenario: All planned families are present

- **WHEN** the `ErrorKind` enum is inspected
- **THEN** all eleven families above SHALL be present and no others

### Requirement: Capability attribution

The crate SHALL provide a `CapabilityId` newtype over `&'static str` so errors attribute their owning capability. `IdentusError::capability()` SHALL return `Option<CapabilityId>`.

#### Scenario: Public error carries its capability

- **WHEN** an `IdentusError::public(...)` is constructed with `CapabilityId::new("did")`
- **THEN** `error.capability()` SHALL return `Some(CapabilityId)` whose `as_str()` is `"did"`

### Requirement: Workspace result type

The crate SHALL provide `IdentusResult<T> = Result<T, IdentusError>` for use at public crate boundaries.

#### Scenario: Result alias is available

- **WHEN** a crate returns `IdentusResult<Self>` from a public parse function
- **THEN** the return type SHALL resolve to `Result<Self, IdentusError>`

### Requirement: Component metadata

Each **runtime** crate SHALL expose a `pub const COMPONENT: identus_core::Component` with a stable `name` and `summary`. `identus-core` SHALL define its own `COMPONENT`. A crate whose `Cargo.toml` declares `[lib] proc-macro = true` is a build-time proc-macro crate, not a runtime crate; it SHALL NOT be required to expose `COMPONENT` (proc-macro crates cannot export non-macro items to downstream crates, so a `COMPONENT` there would have no consumer).

#### Scenario: identus-core self-describes

- **WHEN** `identus_core::COMPONENT.name` is inspected
- **THEN** it SHALL equal `"identus-core"`

#### Scenario: Proc-macro crates are exempt from COMPONENT

- **WHEN** a crate's `Cargo.toml` declares `[lib] proc-macro = true` (e.g. `identus-derive`)
- **THEN** that crate's `src/lib.rs` SHALL contain no `pub const COMPONENT`, and the workspace SHALL NOT require one of it

### Requirement: Domain-facing bridging convention

Domain crates that parse input SHALL keep an idiomatic local error type for `FromStr` and SHALL additionally expose a `parse_with_core_error` function and a `to_identus_error` method returning `IdentusError` with a stable `ErrorCode` and `CapabilityId`. This convention is documented for later adopters; no adopter ships in this change.

#### Scenario: Convention is documented

- **WHEN** the `design.md` of this change is inspected
- **THEN** it SHALL describe the `to_identus_error()` / `parse_with_core_error()` bridging pattern that later domain crates adopt

### Requirement: Deferred binding surface is out of scope

This change SHALL NOT introduce `ResultEnvelope`, `ErrorEnvelope`, `RedactionPolicy`, or `to_error_envelope()`. Those SHALL be added by the bindings change, which is the first change with a binding consumer.

#### Scenario: Binding-facing types are absent

- **WHEN** the `identus-core` public API is inspected
- **THEN** `ResultEnvelope`, `ErrorEnvelope`, and `RedactionPolicy` SHALL NOT be present

### Requirement: Foundation crate has no workspace dependencies

`identus-core` SHALL declare no `identus-*` runtime dependencies. A build-time dependency on a workspace crate whose `Cargo.toml` declares `[lib] proc-macro = true` (e.g. `identus-derive`) is permitted and does not count as a runtime dependency for this requirement.

#### Scenario: Core is runtime-dependency-free

- **WHEN** `crates/core/Cargo.toml` is inspected
- **THEN** every `identus-*` entry in its `[dependencies]` SHALL be a workspace crate whose `Cargo.toml` declares `[lib] proc-macro = true`, and there SHALL be no other `identus-*` dependency

#### Scenario: Core may depend on identus-derive

- **WHEN** `crates/core/Cargo.toml` declares `identus-derive.workspace = true` and `crates/derive/Cargo.toml` declares `[lib] proc-macro = true`
- **THEN** this requirement SHALL be satisfied (the edge is a permitted build-time proc-macro dependency, not a runtime `identus-*` dependency)

### Requirement: Workspace checks remain green after rename

The `identus-ssi` → `identus-core` rename SHALL be transparent to the `nix-tooling` checks. The dormant `[workspace.metadata.crane] name` in the root `Cargo.toml` SHALL be updated to `"identus-core"`.

#### Scenario: Nix flake check stays green after rename

- **WHEN** `nix flake check` is run after the rename
- **THEN** all checks SHALL pass (the rename is transparent to the `members = ["crates/*"]` glob)

#### Scenario: Crane metadata points at the renamed crate

- **WHEN** the root `Cargo.toml` is inspected after the rename
- **THEN** `[workspace.metadata.crane] name` SHALL equal `"identus-core"`