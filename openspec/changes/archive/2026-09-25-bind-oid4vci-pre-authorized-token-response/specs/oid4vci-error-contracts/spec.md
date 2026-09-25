# oid4vci-error-contracts Specification

## ADDED Requirements

### Requirement: Append-only protocol errors may evolve without rewriting the v1 fixture

Issue #375 SHALL append exactly eight fieldless Pre-Authorized Token HTTP
response diagnostics after every existing variant: invalid limits and status,
then oversized/invalid Content-Type, Cache-Control and Pragma pairs. Each SHALL
have an exact stable code, validation kind and static message in a focused
catalogue. The immutable 171-variant v1 fixture prefix and all prior live
variants SHALL remain ordered and unchanged.

#### Scenario: pre-authorized response errors extend the catalogue

- **WHEN** the compile-time inventory is compared with the frozen fixture
- **THEN** the fixture remains an exact prefix and every appended variant is
  unique, explicitly routed and value-free
