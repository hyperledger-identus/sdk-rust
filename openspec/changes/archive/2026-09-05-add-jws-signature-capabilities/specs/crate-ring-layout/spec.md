# crate-ring-layout Specification

## MODIFIED Requirements

### Requirement: In-source layer rulebook

`identus-conformance` SHALL encode the layer rules as an in-source
`pub(crate) const LAYER_RULES`. The credential-semantics layer SHALL contain
`identus-credentials`, `identus-presentations` and `identus-jose`.
`identus-jose` MAY depend inward on `identus-core` and `identus-crypto`; no
protocol, orchestration, outer-boundary, chain or product dependency is
permitted.

#### Scenario: JOSE uses only accepted inward foundations

- **WHEN** the rulebook and `identus-jose` manifest are inspected
- **THEN** `identus-jose` appears exactly once in credential semantics and its
  only `identus-*` runtime dependencies are `identus-core` and
  `identus-crypto`
