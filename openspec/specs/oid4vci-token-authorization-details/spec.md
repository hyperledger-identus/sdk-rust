# oid4vci-token-authorization-details Specification

## Purpose
TBD - created by archiving change add-oid4vci-token-authorization-details. Update Purpose after archive.
## Requirements
### Requirement: validation is an explicit state transition

The SDK SHALL preserve the existing presence-only `TokenResponseCore` parser
and SHALL expose a separate consuming validation transition.

#### Scenario: existing partial core remains compatible

- **WHEN** bounded structurally valid unknown Authorization Details are parsed
- **THEN** the core SHALL still succeed and report presence
- **AND** semantic validation SHALL occur only when explicitly requested.

### Requirement: recognized credential entries are closed and bounded

The validated state SHALL require a non-empty Authorization Details array and
at least one `openid_credential` entry. Each recognized entry SHALL contain a
bounded non-empty `credential_configuration_id` and a non-empty bounded array
of bounded non-empty `credential_identifiers`.

#### Scenario: exact limits succeed and one-over fails

- **WHEN** entry, identifier and decoded string values are at their limits
- **THEN** validation SHALL succeed
- **AND WHEN** any value exceeds its limit
- **THEN** validation SHALL fail with a stable redacted error.

#### Scenario: ambiguous identifiers fail closed

- **WHEN** a credential identifier repeats within or across recognized entries
- **THEN** validation SHALL fail before exposing a selectable dataset.

### Requirement: extensions remain bounded and least authority

The SDK SHALL traverse unknown fields on recognized entries and
authorization-detail objects with an unknown `type` within the existing byte,
depth and node limits, then ignore rather than retain them.

#### Scenario: mixed response preserves recognized entries

- **WHEN** a bounded response contains recognized credential details plus an
  unrelated authorization-detail type and unknown fields
- **THEN** recognized entries SHALL validate in source order
- **AND** unknown values SHALL confer no typed authority.

### Requirement: sensitive values are redacted

Identifiers and exact response JSON SHALL NOT appear in Debug, Display, stable
errors or workspace error bridges.

#### Scenario: distinct canaries remain absent

- **WHEN** every caller-controlled field contains a distinct canary and every
  success/error state is formatted
- **THEN** no canary SHALL appear.

