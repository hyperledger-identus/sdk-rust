## MODIFIED Requirements

### Requirement: Bounded single-pass parsing and component views

`Did` SHALL reject input larger than 2,048 bytes and `DidUrl` SHALL reject
input larger than 4,096 bytes before scanning or allocation. Validation SHALL
perform one linear ASCII byte pass without regex, URL or external DID parser
dependencies. Successful values SHALL cache byte offsets so method,
method-specific identifier, DID, path, query and fragment reads return borrowed
slices without allocation. `TryFrom<String>` SHALL reuse the supplied string
allocation; converting `Did` into the equivalent `DidUrl` SHALL move it.

An external grammar engine SHALL replace this implementation only when pinned,
attributable differential evidence proves the same accepted language, exact
serialization, pre-allocation limits, immutable facade, allocation reuse,
borrowed views, redacted errors and supported-target behavior without adding
unused protocol semantics or unresolved unsafe reach.

#### Scenario: oversized input is rejected before semantic parsing

- **WHEN** a bare DID or DID URL exceeds its public SDK byte limit
- **THEN** it SHALL be rejected as too long without scanning the full grammar
  or reflecting the input in an error

#### Scenario: repeated component access does not allocate

- **WHEN** any component accessor is called repeatedly on a valid value
- **THEN** it SHALL return a borrowed slice from the one owned representation

#### Scenario: candidate mismatch retains the local boundary

- **WHEN** a proposed parser differs on grammar, exact storage, resource work,
  facade invariants, allocation reuse, errors, targets, dependency cohesion or
  safety reach
- **THEN** the local parser SHALL remain and the candidate SHALL be recorded
  with a bounded reconsideration trigger rather than wrapped into production
