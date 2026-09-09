## ADDED Requirements

### Requirement: Core inherited input boundaries have a crate-level inventory

`identus-core` SHALL maintain an evidence-backed inventory of every public
externally constructible or deserializable value. The inventory SHALL state the
retained representation, intrinsic byte/range/work bound, allocation caveat and
deterministic evidence for each input-bearing surface. Static-only and no-input
surfaces SHALL be classified explicitly.

The crate-level inventory SHALL NOT be represented as completion of the
repository-wide inherited resource-bound audit.

#### Scenario: Serialized time values enforce the u64 range

- **WHEN** `UnixTimestampMillis` or `DurationMillis` deserializes a negative,
  fractional or greater-than-`u64` JSON number
- **THEN** deserialization SHALL fail without constructing the value
- **AND** the maximum `u64` JSON number SHALL remain accepted

#### Scenario: Monotonic time has no wire input

- **WHEN** the public `MonotonicTimestampMillis` surface is inspected
- **THEN** it SHALL have no serde serialization or deserialization contract

#### Scenario: Crate audit preserves the wider limitation

- **WHEN** every current `identus-core` surface has evidence
- **THEN** `SDK-LIM-007` SHALL remain effective for unaudited SDK crates and
  allocation performed before SDK validation
