# sdk-dependency-boundary Specification

## Purpose

Keep the generic Identus SDK dependency graph independent from chains,
products and evidence-source repositories through deterministic manifest,
source, path and resolved-closure enforcement.

## Requirements
### Requirement: SDK dependencies remain chain and product neutral

Every SDK workspace crate SHALL remain independent of Midnight, Compact,
PRISM/Cardano, NeoPRISM, Lace ID Portal, Oxid and consumer product runtimes.
The repository SHALL reject prohibited direct and resolved package identities
without requiring network or donor access.

#### Scenario: Neutral dependency is added

- **WHEN** a workspace crate uses a general-purpose crates.io dependency whose
  identity and resolved source do not match a prohibited family
- **THEN** the chain-neutral dependency guard accepts it

#### Scenario: Prohibited package enters the resolved closure

- **WHEN** the lockfile contains a package in a prohibited chain or product
  family
- **THEN** the guard fails and names the package and matched policy family

### Requirement: Every Cargo dependency surface is inspected

The guard SHALL inspect normal, development and build dependencies at both the
top level and under every target-specific Cargo table. It SHALL also inspect
root Cargo `[patch]` and `[replace]` source overrides. It SHALL evaluate the
dependency alias and an explicit `package` identity after normalizing ASCII
case and underscore/hyphen spelling.

#### Scenario: Alias hides a chain package

- **WHEN** a neutral dependency key renames a prohibited `package`
- **THEN** validation fails on the package identity rather than accepting the
  neutral alias

#### Scenario: Target table hides a product dependency

- **WHEN** a prohibited dependency is declared only for one target in a
  development or build dependency table
- **THEN** validation fails with the manifest section and dependency identity

#### Scenario: Patch redirects a neutral package outside the SDK

- **WHEN** a Cargo patch or replacement redirects a neutral package to a path
  outside the repository or to a prohibited Git source
- **THEN** validation fails even when the resolved lockfile has no source field

### Requirement: Dependency locations cannot couple donor repositories

A direct dependency Git URL SHALL NOT reference a prohibited donor or consumer
repository. Every direct local path SHALL resolve inside the SDK repository,
and workspace-member dependency paths SHALL resolve beneath `crates/`.

#### Scenario: Neutral alias points at a donor Git repository

- **WHEN** a dependency uses a neutral alias and a Git URL for NeoPRISM,
  midnight-identity, Lace ID Portal, Oxid or Apollo
- **THEN** validation fails and identifies the prohibited source

#### Scenario: Local path escapes the repository

- **WHEN** a dependency path resolves outside the SDK repository
- **THEN** validation fails even when the dependency alias is neutral

#### Scenario: Workspace crate uses its declared internal path

- **WHEN** a workspace dependency resolves to its declared package beneath
  `crates/`
- **THEN** the location guard accepts it

### Requirement: Boundary rules have adversarial regression evidence

The repository SHALL carry deterministic positive and negative tests for
aliases, package identities, dependency-section variants, target tables, Git
sources, local paths and resolved lockfile entries. The real workspace SHALL
be asserted clean by the same rule implementation.

#### Scenario: Bypass regression is introduced

- **WHEN** a future change stops inspecting one required Cargo surface
- **THEN** at least one synthetic adversarial test fails before integration

#### Scenario: Current workspace is checked

- **WHEN** the conformance suite runs against the repository manifests and
  lockfile
- **THEN** it reports no prohibited dependency or source
