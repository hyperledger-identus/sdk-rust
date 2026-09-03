## ADDED Requirements

### Requirement: Generated gate mappings preserve manifest identity

The offline support-policy validator SHALL require the Nix gate generator to
derive every mapped attribute name from the current manifest entry's `name`
field and every mapped attribute value from `makeGate` applied to that same
entry. The complete mapping SHALL remain connected from `manifest.gates`
through `listToAttrs` to the returned top-level `checks` value.

#### Scenario: Constant mapped name collapses the gate graph

- **WHEN** the generator replaces the current entry's name with a constant
  attribute name
- **THEN** structural validation fails before multiple manifest gates can
  collapse into one published Nix check

#### Scenario: Mapped value bypasses gate construction

- **WHEN** the generator maps a manifest entry to a value not produced by
  `makeGate` for that same entry
- **THEN** structural validation rejects the detached gate implementation
