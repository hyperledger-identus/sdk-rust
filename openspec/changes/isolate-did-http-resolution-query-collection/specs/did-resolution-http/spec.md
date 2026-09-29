# DID Resolution HTTP query collection ownership

## ADDED Requirements

### Requirement: Resolution query fields have one private collection owner

The adapter MUST keep source-order decoded-name uniqueness, common option
conversion, extension collection, mutually exclusive version selection, and
validated option construction behind one cohesive private query-field boundary
while the unchanged coordinator owns representation and empty-query projection.

#### Scenario: A bounded query is collected

- **WHEN** a negotiated representation and bounded valid query satisfy the
  existing structure, component, typed-option, and correlation rules
- **THEN** the resolver receives the same exact accepted representation, common
  values, and ordered string extensions once.

#### Scenario: Multiple query phases are invalid

- **WHEN** raw size/count, structure, decoding, control, duplication, known
  option, version correlation, or final construction contain two or more
  simultaneous failures
- **THEN** the same existing `invalidOptions` response is selected from the
  earliest current phase
- **AND** the resolver is not invoked.

#### Scenario: Collection remains bounded and transport-specific

- **WHEN** the private owner consumes query parameters
- **THEN** existing raw/member/name/value ceilings, strict percent decoding,
  literal-plus behavior, source order, duplicate rules, one ordered extension
  map, and redacted diagnostics remain authoritative
- **AND** no generic form framework, new dependency, allocation class, ambient
  I/O, dynamic dispatch, synchronization, metric waiver, or helper-per-option
  chain is added.
