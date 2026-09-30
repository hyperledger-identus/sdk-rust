# language-adapter-mappings Specification

## Purpose

Define the canonical, versioned compatibility boundary between SDK-Rust
capabilities and temporary language-SDK DTO, error, and lifecycle facades.
## Requirements
### Requirement: Rust contracts remain canonical

Every language-adapter mapping SHALL identify a canonical Rust crate,
module/symbol or public error code and its version origin. A language DTO,
exception, error string, or runtime convention SHALL NOT constrain the Rust
core; compatibility SHALL be implemented by an outer, versioned mapping.

#### Scenario: a legacy language DTO has an extra derived field

- **WHEN** the field can be calculated from the canonical Rust value
- **THEN** the mapping records it as derived instead of adding it to the Rust
  domain type

### Requirement: Mapping fidelity and unsupported behavior are explicit

Every mapping SHALL declare its direction, compatibility class, and lossless
or lossy fidelity. A lossy mapping SHALL name each lost distinction and define
deterministic unsupported or migration behavior rather than silently normalize
canonical data.

#### Scenario: a legacy DID URL cannot preserve an empty query

- **WHEN** reverse conversion would lose absent-versus-empty information
- **THEN** the mapping marks the case unsupported or routes it to an additive
  replacement shape without changing `identus-did`

### Requirement: Errors preserve stable codes and redaction

Error mappings SHALL retain the canonical stable Rust code, bounds, and
redaction policy even when a legacy language facade uses one exception class
or human message for multiple failures. Human error text SHALL NOT be a stable
compatibility identifier.

#### Scenario: two Rust parse failures map to one legacy class

- **WHEN** `did.invalid_did` and `did.invalid_did_url` both surface as
  `InvalidDIDString`
- **THEN** adapter observability still distinguishes the stable Rust code and
  rejected caller input remains redacted

### Requirement: Compatibility has a bounded lifecycle

Every active mapping SHALL record its source revision, supported version
window, deprecation phase, consumers, vector evidence, exact selectors,
migration action, replacement, observability, fallback, rollback, and removal
gate. The window SHALL use an ordered `>=x.y.z,<x.y.z` interval containing the
pinned language version. Stable mapping IDs SHALL only evolve through explicit
versioning or replacement.

#### Scenario: the last supported consumer adopts the canonical shape

- **WHEN** the removal gate's evidence is satisfied
- **THEN** the legacy mapping may enter removal through its declared migration
  and rollback process rather than disappearing silently

### Requirement: Adapter bounds cannot weaken Rust policy

Every mapping SHALL resolve its input ceiling to a named literal public Rust
`usize` constant in repository source. The adapter ceiling MAY be stricter and
SHALL NOT exceed that canonical value.

#### Scenario: an adapter raises its accepted DID size

- **WHEN** the declared adapter ceiling exceeds `MAX_DID_BYTES`
- **THEN** validation fails before the mapping can be accepted

### Requirement: Registry and human view remain deterministic

The machine registry SHALL use a closed versioned schema and fail on unknown or
incoherent metadata. The repository SHALL retain a deterministic human-readable
rendering of every governed field, and validation SHALL fail when it drifts
from the registry. Required seed records SHALL NOT prevent additional
schema-conforming language mappings.

#### Scenario: a registry field changes without regenerating documentation

- **WHEN** the checked-in Markdown no longer equals deterministic rendering
- **THEN** the factory gate rejects the change
