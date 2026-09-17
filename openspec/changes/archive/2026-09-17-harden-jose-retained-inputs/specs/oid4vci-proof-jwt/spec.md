## ADDED Requirements

### Requirement: Identified proof clients are construction-bounded

Every public identified-client alternative SHALL carry an opaque value
validated under `Oid4vciProofJwtLimits`. A raw `String` SHALL NOT directly
inhabit that alternative. The existing named fallible constructor SHALL reject
empty, control-containing, or excessive client identifiers before returning a
retained client mode, and proof construction under tighter limits SHALL
revalidate it.

#### Scenario: native and parsed identified clients agree

- **WHEN** native construction or bounded proof parsing receives the same
  non-empty client identifier at the configured byte ceiling
- **THEN** both paths produce the same identified mode and unchanged `iss` wire
  value without exposing its content through diagnostics

#### Scenario: one-over client input cannot reach signing

- **WHEN** a client identifier exceeds the configured ceiling by one byte or a
  previously accepted value is reused under a tighter proof limit
- **THEN** construction or preparation fails with a static proof-claims error
  and the signer observes zero calls
