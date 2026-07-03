## ADDED Requirements

### Requirement: External dependencies are workspace-declared

Every external (non-`workspace.internal`, non-`path`) dependency used by any crate in the workspace SHALL be declared once in the root `Cargo.toml` `[workspace.dependencies]` block, and referenced from each consuming crate's manifest via `<dep>.workspace = true`. The `version` field SHALL be specified only on the root entry and SHALL NOT be repeated per-crate. Per-crate manifests MAY add `features = [...]` on top of the `.workspace = true` reference; per-crate manifests SHALL NOT specify `version` for an external dependency. A `path`-internal `identus-*` dependency (already governed by `crate-ring-layout`'s workspace-dependency map) is not an external dependency for this requirement. A dependency declared with a `path` key (a vendored or local-path crate, whether or not it also carries a `version`) is likewise out of scope: path-style entries are a deliberate local choice that does not participate in the workspace single-source-of-truth, so the guard excludes any entry carrying a `path` field from this check.

#### Scenario: External dep referenced via workspace form

- **WHEN** a crate manifest's `[dependencies]` declares an external crate `toml`
- **THEN** the entry SHALL be `toml.workspace = true` and the root `[workspace.dependencies]` SHALL contain a `toml` entry with its `version`

#### Scenario: Inline external version is rejected

- **WHEN** a crate manifest declares an external dependency with an inline `version = "..."` (e.g. `toml = "0.8"`)
- **THEN** that manifest SHALL NOT conform to this requirement

#### Scenario: Per-crate features are permitted on top of a workspace entry

- **WHEN** a crate needs a subset of an external dep's features beyond the workspace-declared ones
- **THEN** the crate manifest MAY use `toml = { workspace = true, features = ["parse"] }` and the entry SHALL still conform

### Requirement: default-features is single-sourced at workspace level

When an external dependency requires `default-features = false`, the `default-features = false` field SHALL be declared on the root `[workspace.dependencies]` entry, not per-crate. A consuming crate MAY override to `default-features = true` locally as a documented escape hatch for a crate that needs the defaults on; this is the exception, not the norm, and the workspace entry remains the single source for the off-state. This requirement is review-enforced, not guard-enforced: the `crate-ring-layout` dep-graph guard checks only the `version` source (inline vs `workspace = true`) and does not inspect `default-features`, so a non-conforming `default-features` placement is caught by code review rather than by the `#[test]`.

#### Scenario: default-features off-state lives at workspace level

- **WHEN** the workspace wants `k256` with defaults off
- **THEN** the root `[workspace.dependencies]` entry SHALL be `k256 = { version = "...", default-features = false, features = [...] }` and consuming crates SHALL reference it via `k256.workspace = true` without restating `default-features`

#### Scenario: A crate may override default-features on

- **WHEN** a single crate needs `k256`'s default features despite the workspace off-state
- **THEN** that crate MAY declare `k256 = { workspace = true, default-features = true, features = [...] }` as a deliberate exception, and other crates SHALL remain unaffected

### Requirement: Optional and target-specific external entries conform identically

An external dependency marked `optional = true` (as required for Cargo feature-gating) SHALL be declared at workspace level and referenced via `.workspace = true` identically to a non-optional external dependency. An external dependency declared under a target-specific `[target.'cfg(...)'.dependencies]`, `[target.'cfg(...)'.dev-dependencies]`, or `[target.'cfg(...)'.build-dependencies]` section, or under a top-level `[build-dependencies]` section, SHALL conform to the same workspace-declaration rule as a `[dependencies]` entry. The `optional`, `target`, and `build` placement of an external dep is orthogonal to whether its `version` is single-sourced.

#### Scenario: Feature-gated optional external dep is workspace-declared

- **WHEN** a crate feature-gates an external crate via `ed25519 = ["ed25519-dalek"]` in `[features]`
- **THEN** the `[dependencies]` entry SHALL be `ed25519-dalek = { workspace = true, optional = true }` and the root `[workspace.dependencies]` SHALL contain the `ed25519-dalek` entry with its `version`

#### Scenario: Target-specific external dep is workspace-declared

- **WHEN** a crate declares an external dep under `[target.'cfg(target_family = "wasm")'.dependencies]`
- **THEN** that entry SHALL use `<dep>.workspace = true` and SHALL NOT pin `version = "..."` inline

#### Scenario: Build-dependency external dep is workspace-declared

- **WHEN** a crate declares an external build-dependency
- **THEN** the `[build-dependencies]` entry SHALL use `<dep>.workspace = true` and SHALL NOT pin `version = "..."` inline