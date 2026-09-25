## ADDED Requirements

### Requirement: IDR-023 completes with qualified generic vector evidence

The program SHALL use issue #376 after issue #375 delivers the final required
wallet-side response binder to provide clean-room generic interoperability vectors,
provenance drift validation and immutable consumer-design mapping. IDR-023 may
be marked `delivered` only when the reviewed Final matrix has no missing
required row.

#### Scenario: generic vector closeout passes

- **WHEN** every repository-authored vector passes its public-API expectation
  and the matrix has zero missing rows
- **THEN** IDR-023 retains closed issue #376 as durable delivery evidence
- **AND** M4 may close as the bounded wallet-core milestone

#### Scenario: functional completion is not product adoption

- **WHEN** IDR-023 and M4 are functionally complete
- **THEN** publication, official certification, current app-team approval,
  live Oxid/Portal interoperability and downstream dependency adoption remain
  separate work
