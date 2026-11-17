## MODIFIED Requirements

### Requirement: Bounded extensible DID resolution options

The DID Core capability SHALL provide immutable `ResolutionOptions` containing
optional `accept`, `expandRelativeUrls`, `noCache`, `versionId` and
`versionTime` values plus method/extension members. Common members SHALL reuse
their existing typed validators. Absence and explicit false SHALL remain
distinct, extensions SHALL NOT shadow common keys, and native construction and
serde SHALL enforce the same bounded metadata policy.

#### Scenario: common and method options remain portable

- **WHEN** a PRISM- or Midnight-shaped resolver receives common options and a
  method extension
- **THEN** it SHALL observe the exact typed values and preserved extension JSON
  without importing chain, method, transport or runtime types

#### Scenario: malformed resolution options fail closed

- **WHEN** an option map contains a malformed media type, datetime or version,
  a reserved extension collision, excessive input or invalid open JSON
- **THEN** every public construction path SHALL reject it with the existing
  redaction-safe DID resolution error boundary
