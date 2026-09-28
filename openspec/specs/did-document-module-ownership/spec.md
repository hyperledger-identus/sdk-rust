# did-document-module-ownership Specification

## Purpose
TBD - created by archiving change decompose-did-document. Update Purpose after archive.
## Requirements
### Requirement: DID document responsibilities have cohesive private owners

The SDK MUST keep wire cardinality, recursive extension budgets, verification
material, services, and aggregate document behavior in cohesive private owners
behind the unchanged crate-root DID document vocabulary.

#### Scenario: A context or extension value is accepted

- **WHEN** callers construct or deserialize JSON-LD contexts, suite/service
  properties, endpoint maps, or document extensions
- **THEN** existing depth, node, collection, property, name, and string limits
  apply in the same order with the same errors
- **AND** rejected hostile values are cleaned without recursive drop risk.

#### Scenario: Verification material and relationships are validated

- **WHEN** callers construct verification methods or document relationships
- **THEN** open type, public JWK, canonical multibase, known-material
  exclusivity, resource uniqueness, and relationship uniqueness invariants
  remain enforced
- **AND** no DID method, suite support, trust, or authorization claim is added.

#### Scenario: Services and the aggregate document are validated

- **WHEN** callers construct services or a DID document through native or serde
  paths
- **THEN** wire cardinality, endpoint, type, extension, collection,
  cross-resource, duplicate, and raw preflight invariants remain equivalent
- **AND** native, wire, and semantic round-trip behavior stays aligned.

### Requirement: Decomposition is compatibility preserving

The SDK MUST preserve existing public paths, signatures, visibility, derives,
constness, serde representation, limits, errors, validation ordering, cleanup,
redaction, dependencies, features, and target support.

#### Scenario: Consumers rebuild after the decomposition

- **WHEN** existing code imports DID document vocabulary from `identus_did`
- **THEN** it compiles without source changes
- **AND** no private child module becomes a supported public path.

#### Scenario: Code-health evidence is refreshed

- **WHEN** the ownership move is complete
- **THEN** the DID document hotspot is removed from the governed baseline
- **AND** no new over-threshold descendant or unrelated waiver is introduced.

