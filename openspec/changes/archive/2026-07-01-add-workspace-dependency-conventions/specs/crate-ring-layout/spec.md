## ADDED Requirements

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