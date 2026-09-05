## ADDED Requirements

### Requirement: Runtime edges use canonical package identity

The dependency guard SHALL resolve each top-level and target-specific runtime
dependency through its declared `package` identity when present and otherwise
through its dependency key. It SHALL compare the resolved identity to canonical
workspace package names, deduplicate repeated identities, and apply layer and
verification-leaf rules to the resulting set. Renaming a dependency SHALL NOT
hide an internal runtime edge. Development and build dependencies SHALL remain
outside this runtime-edge set. When a member dependency declares
`workspace = true`, the guard SHALL resolve its identity from the matching root
`[workspace.dependencies]` entry, including that root entry's `package` rename,
and SHALL ignore a member-local `package` field just as Cargo 1.85 does.

#### Scenario: Renamed internal dependency reaches the layer guard

- **WHEN** a runtime dependency alias declares `package = "identus-core"`
- **THEN** the guard SHALL inspect an edge to `identus-core` rather than omit
  the declaration or inspect the alias as a package

#### Scenario: Repeated aliases resolve once

- **WHEN** top-level or target-specific runtime tables declare multiple aliases
  for the same internal package
- **THEN** the collector SHALL return that canonical package identity once

#### Scenario: Renamed edge cannot bypass a verification-leaf invariant

- **WHEN** `identus-wallet-conformance` adds a renamed runtime dependency whose
  package is not `identus-wallet`
- **THEN** its exact wallet-only dependency assertion SHALL fail

#### Scenario: Inherited dependency identity comes from the workspace root

- **WHEN** a member declares
  `identus-did = { workspace = true, package = "identus-derive" }` and the root
  `identus-did` workspace dependency resolves to package `identus-did`
- **THEN** the guard SHALL inspect an edge to `identus-did`, not
  `identus-derive`, because Cargo ignores the member-local package override
