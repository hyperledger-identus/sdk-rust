## ADDED Requirements

### Requirement: The published crypto API origin remains immutable

The SDK SHALL preserve the generated public-API baseline of an already-published
`identus-crypto` artifact byte-for-byte. Candidate qualification SHALL compare
current generated API to that origin, allow reported additions, and fail on any
removed or changed baseline occurrence.

#### Scenario: Current API adds validation methods

- **WHEN** generated comparison retains every published API occurrence and adds
  public-key validation methods
- **THEN** qualification preserves the origin, emits the exact additive report,
  and may continue

#### Scenario: Current API removes or changes a published item

- **WHEN** the current line multiset omits any occurrence from the published
  origin
- **THEN** release-candidate validation fails before readiness can advance

#### Scenario: Initialization targets a published origin

- **WHEN** baseline initialization is requested while the descriptor identifies
  a published origin
- **THEN** the candidate command fails without modifying the baseline
