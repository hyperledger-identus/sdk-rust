## ADDED Requirements

### Requirement: Credential envelope activation is inventoried

The machine-readable and human bootstrap inventories SHALL classify
`identus-credentials` as implemented experimental credential semantics after
its bounded envelope replaces the marker. The inventory SHALL name issue #71,
describe the active public surface without claiming verification or format
support, and remove the crate from the placeholder set.

#### Scenario: Inventory checker recognizes the implemented credential crate

- **WHEN** the bootstrap inventory validator inspects the workspace
- **THEN** `identus-credentials` SHALL be accepted as implemented, SHALL NOT be
  subjected to marker-only source shape, and the exact remaining placeholder
  set SHALL still satisfy its contract
