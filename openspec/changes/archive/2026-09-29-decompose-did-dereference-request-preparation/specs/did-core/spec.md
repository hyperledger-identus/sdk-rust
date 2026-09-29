# DID dereferencing request-preparation ownership

## ADDED Requirements

### Requirement: Generic dereferencing request preparation preserves phased authority

The SDK MUST keep generic DID URL request preparation separate from one private
owner that projects dereferencing options, applies a completely parsed sorted
query map, constructs resolution options, and validates selector combinations.

#### Scenario: A valid request is prepared

- **WHEN** a DID URL and dereferencing options contain compatible bounded
  resolution parameters, extensions, and generic resource selectors
- **THEN** the resolver receives the same base DID and identical typed
  `ResolutionOptions` exactly once as before decomposition
- **AND** subsequent dereferencing retains the same successful result.

#### Scenario: Multiple preparation invariants are invalid

- **WHEN** input violates more than one option-collision, query grammar,
  parameter, resolution-option, or selector-combination invariant
- **THEN** preparation returns the same existing static failure selected by the
  current phase and decoded-name sorted parameter order
- **AND** the resolver is not invoked and no caller value enters diagnostics.

#### Scenario: Preparation remains bounded and cohesive

- **WHEN** the private owner clones options, parses and applies parameters, and
  returns a prepared request
- **THEN** existing DID URL, query text, typed value, and extension JSON limits
  remain authoritative
- **AND** no new decoding path, ambient I/O, allocation class, dynamic dispatch,
  synchronization, dependency, generic framework, metric waiver, or forwarding-
  only helper chain is added.
