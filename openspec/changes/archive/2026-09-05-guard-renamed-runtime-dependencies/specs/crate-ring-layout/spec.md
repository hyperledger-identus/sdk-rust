## ADDED Requirements

### Requirement: Runtime edges use canonical package identity

The dependency guard SHALL resolve each top-level and target-specific runtime
dependency through its declared `package` identity when present and otherwise
through its dependency key. It SHALL compare the resolved identity to canonical
workspace package names, deduplicate repeated identities, and apply layer and
verification-leaf rules to the resulting set. Renaming a dependency SHALL NOT
hide an internal runtime edge. Development and build dependencies SHALL remain
outside this runtime-edge set.

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
