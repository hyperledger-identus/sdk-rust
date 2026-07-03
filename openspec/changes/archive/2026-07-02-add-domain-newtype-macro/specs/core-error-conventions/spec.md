## MODIFIED Requirements

### Requirement: Foundation crate has no workspace dependencies

`identus-core` SHALL declare no `identus-*` runtime dependencies. A build-time dependency on a workspace crate whose `Cargo.toml` declares `[lib] proc-macro = true` (e.g. `identus-derive`) is permitted and does not count as a runtime dependency for this requirement.

#### Scenario: Core is runtime-dependency-free

- **WHEN** `crates/core/Cargo.toml` is inspected
- **THEN** every `identus-*` entry in its `[dependencies]` SHALL be a workspace crate whose `Cargo.toml` declares `[lib] proc-macro = true`, and there SHALL be no other `identus-*` dependency

#### Scenario: Core may depend on identus-derive

- **WHEN** `crates/core/Cargo.toml` declares `identus-derive.workspace = true` and `crates/derive/Cargo.toml` declares `[lib] proc-macro = true`
- **THEN** this requirement SHALL be satisfied (the edge is a permitted build-time proc-macro dependency, not a runtime `identus-*` dependency)

### Requirement: Component metadata

Each **runtime** crate SHALL expose a `pub const COMPONENT: identus_core::Component` with a stable `name` and `summary`. `identus-core` SHALL define its own `COMPONENT`. A crate whose `Cargo.toml` declares `[lib] proc-macro = true` is a build-time proc-macro crate, not a runtime crate; it SHALL NOT be required to expose `COMPONENT` (proc-macro crates cannot export non-macro items to downstream crates, so a `COMPONENT` there would have no consumer).

#### Scenario: identus-core self-describes

- **WHEN** `identus_core::COMPONENT.name` is inspected
- **THEN** it SHALL equal `"identus-core"`

#### Scenario: Proc-macro crates are exempt from COMPONENT

- **WHEN** a crate's `Cargo.toml` declares `[lib] proc-macro = true` (e.g. `identus-derive`)
- **THEN** that crate's `src/lib.rs` SHALL contain no `pub const COMPONENT`, and the workspace SHALL NOT require one of it