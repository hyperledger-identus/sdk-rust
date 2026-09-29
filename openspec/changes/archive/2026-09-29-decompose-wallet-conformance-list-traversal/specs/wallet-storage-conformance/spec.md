# Wallet storage conformance ownership

## ADDED Requirements

### Requirement: List traversal separates adapter calls from bounded evidence

The SDK MUST keep asynchronous list-port orchestration separate from the
deterministic bounded evidence that validates returned pages, entries,
continuation progress, termination, and final membership. Both responsibilities
MUST remain private behind the existing list-capable checker functions.

#### Scenario: Adapter returns a valid paginated index

- **WHEN** every page respects the requested bound, entries are unique, cursors
  progress, traversal terminates, and the observed set equals the fixture
- **THEN** the scenario preserves the exact request sequence and operation count
- **AND** returns the same aggregate report as before decomposition.

#### Scenario: Adapter violates a pagination invariant

- **WHEN** the adapter fails, exceeds the page bound, duplicates an entry,
  returns wrong membership, repeats a cursor, or keeps returning continued
  pages beyond the expected membership
- **THEN** the scenario fails at the existing static list step with the existing
  closed failure kind and priority reachable through validated page values
- **AND** no scope, entry, cursor, or adapter error enters diagnostics.

#### Scenario: Evidence remains resource bounded

- **WHEN** the scenario accepts pages and continuation cursors
- **THEN** observed entries and cursor history remain bounded by the validated
  fixture cardinality and fixture-derived request limit
- **AND** no callback table, dynamic dispatch, executor, or new dependency is
  introduced.

#### Scenario: Touched function signal is removed semantically

- **WHEN** code-health evidence is refreshed from the protected implementation
- **THEN** no list-scenario function exceeds a configured function threshold
- **AND** the improvement comes from one cohesive evidence owner rather than
  forwarding helpers, generated code, moved tests, a waiver, or weaker policy.
