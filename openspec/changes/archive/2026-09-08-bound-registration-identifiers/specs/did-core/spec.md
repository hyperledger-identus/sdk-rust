## ADDED Requirements

### Requirement: Registration identifier borrowed parsing is pre-allocation bounded

Every DID Registration opaque identifier borrowed parser SHALL validate the
original `&str` against its existing byte ceiling and grammar before allocating
the owned value retained on success. Owned `String` construction SHALL validate
and move the caller's allocation without cloning. Rejected input SHALL NOT be
retained or rendered in diagnostics.

#### Scenario: Exact and one-over identifier boundaries

- **WHEN** each generated identifier type parses valid input at its exact byte ceiling
- **THEN** borrowed and owned construction SHALL accept it
- **AND** one additional byte SHALL be rejected before grammar traversal and ownership

#### Scenario: Invalid borrowed identifier is rejected without retention

- **WHEN** borrowed input is empty, has surrounding whitespace, contains a control character, or exceeds its type's byte ceiling
- **THEN** parsing SHALL fail with the stable registration error
- **AND** local `Debug` and `Display` diagnostics SHALL contain none of the rejected input

#### Scenario: Existing registration behavior remains compatible

- **WHEN** a currently accepted identifier is supplied through borrowed or owned construction and used in its registration lifecycle type
- **THEN** its exact value and existing lifecycle behavior SHALL remain unchanged
