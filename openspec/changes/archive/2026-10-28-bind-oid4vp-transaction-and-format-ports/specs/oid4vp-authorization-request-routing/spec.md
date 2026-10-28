## MODIFIED Requirements

### Requirement: Authorization semantics are one consuming composed transition

The SDK SHALL consume one verified and verifier-authorized Request Object,
deserialize its already bounded duplicate-safe payload once, and return one
state that preserves JAR evidence, owns one valid DCQL query, and retains any
validated transaction-data linkage through caller-supplied capabilities.

#### Scenario: one supported request is valid

- **WHEN** a verified request has supported authorization fields, coherent
  routing, one valid DCQL query, and supported transaction/format semantics
- **THEN** the result owns composed redacted evidence without exposing raw payload or foreign DTOs

#### Scenario: a caller still needs query-only evidence

- **WHEN** the existing DCQL-only transition is used with the baseline empty-metadata profile
- **THEN** its narrower evidence claim and no-transaction behavior remain available
