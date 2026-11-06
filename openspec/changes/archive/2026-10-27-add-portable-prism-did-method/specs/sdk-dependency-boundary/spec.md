# sdk-dependency-boundary Delta

## MODIFIED Requirements

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

A method-specific crate MAY own portable deterministic method semantics when
separately accepted by ADR and OpenSpec. The exact `identus-prism-did`
identity SHALL be accepted only as a source-less in-tree workspace package;
registry, Git, prefixed and ledger-family variants SHALL remain prohibited.

#### Scenario: Exact local portable method package is used

- **WHEN** `identus-prism-did` resolves through the workspace path beneath
  `crates/` and its lockfile entry has no external source
- **THEN** the chain-neutral dependency guard accepts it

#### Scenario: Same name comes from a registry

- **WHEN** `identus-prism-did` is version-only or its lockfile entry has a
  registry or Git source
- **THEN** the guard rejects it as prohibited PRISM coupling

#### Scenario: Expanded method or ledger package is used

- **WHEN** a dependency uses `identus-prism-did-*`, `prism-*`, Cardano,
  Pallas, Oura or Blockfrost identity/source families
- **THEN** the guard rejects it without treating the exact local exception as
  a wildcard
