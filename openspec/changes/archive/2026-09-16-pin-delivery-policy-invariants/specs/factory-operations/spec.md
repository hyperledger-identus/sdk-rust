# factory-operations delta

## ADDED Requirements

### Requirement: Accepted delivery-policy values are validated exactly

The factory SHALL derive one target plan from an exact base/head diff and
delivery profile. The active-development PR requirement SHALL remain the
single Linux `fast` gate. Slow promotion SHALL block exactly, in order,
`production-promotion`, `publication`, and `release-preparation`; an omitted,
reordered, or additional blocker SHALL fail policy validation.

Slice decomposition guidance SHALL remain exactly 12 changed files and 1,000
changed text lines with `decomposition-note` as the advisory action. A lower or
higher threshold SHALL fail policy validation. Crossing the canonical values
SHALL request a decomposition note and SHALL NOT alone approve or reject work.

#### Scenario: Slow blocker policy drifts

- **WHEN** a blocker is omitted, reordered, or added
- **THEN** policy validation fails closed before a target plan is emitted

#### Scenario: Decomposition guidance drifts

- **WHEN** either threshold is lower or higher than its accepted value, or the
  advisory action changes
- **THEN** policy validation fails closed without turning the threshold into a
  correctness decision

#### Scenario: Canonical policy is planned

- **WHEN** the exact accepted blocker set and decomposition guidance are loaded
- **THEN** the existing fast/slow plan shape and single required `fast` status
  remain unchanged
