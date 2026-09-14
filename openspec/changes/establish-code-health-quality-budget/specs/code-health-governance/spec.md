## ADDED Requirements

### Requirement: Code-health evidence separates authored populations

The repository SHALL provide a deterministic, version-pinned audit command and
canonical report format. It SHALL report authored production, external-test,
and inline-test populations separately and SHALL identify generated exclusions.
Inline code SHALL leave production only when syntax-aware evaluation proves its
conditional-compilation predicate false with `test = false`; unknown feature
and target predicates SHALL remain production.

#### Scenario: Test-only inline module

- **WHEN** a source item is guarded by `cfg(test)` or an `all` expression that
  is false when `test = false`
- **THEN** its lines and functions appear in inline-test evidence and not the
  production population

#### Scenario: Test is one alternative

- **WHEN** an item uses `cfg(any(test, feature = "diagnostics"))`
- **THEN** the unknown non-test predicate keeps the item in production evidence

### Requirement: Metrics prompt review rather than dictate architecture

The audit SHALL report module concentration, function SLOC, cognitive and
cyclomatic signals, and declared duplication seams. Threshold crossings SHALL
NOT fail solely because of a number. Every declared hotspot SHALL instead have
exactly one disposition: `decompose`, `deduplicate`, `document-exception`, or
`defer-with-owner`, with evidence and an owner.

#### Scenario: Large cohesive grammar

- **WHEN** a parser exceeds an attention threshold but owns one grammar,
  cursor state, error projection, and change axis
- **THEN** it may be a documented exception rather than an artificial split

#### Scenario: Large module has independent invariants

- **WHEN** one module owns separately changing model, validation, and wire
  preflight responsibilities
- **THEN** it is classified for cohesive private decomposition even though its
  public API may remain unchanged

### Requirement: Touched semantic scope is ratcheted without metric gaming

For a code-health refactor, review SHALL compare the semantic responsibility
and relevant call cluster at base and head. A new or worsened signal SHALL be
classified with an owner. Moves, renames, forwarding wrappers, formatting,
generated output, macro hiding, arbitrary file splits, or deletion/movement of
tests SHALL NOT count as improvement. Shared extraction SHALL require matching
invariants, bounds, error projection, ownership, and likely change cadence.

#### Scenario: Function becomes a forwarding wrapper

- **WHEN** branches move into a new helper but the same responsibility and call
  cluster retain equal behavior and complexity
- **THEN** the review does not claim reduced complexity from the wrapper alone

#### Scenario: Similar protocol token rules

- **WHEN** two grammars look alike but own different bounds, error types, or
  normative change cadence
- **THEN** duplication evidence does not authorize a shared abstraction

### Requirement: Standard DID resolution failures have one private constructor

DID cache and registry failure paths SHALL use one crate-private constructor in
the resolution ownership module. The public result, standard error kind,
metadata, content, serialized JSON, feature surface, and dependency direction
SHALL remain unchanged.

#### Scenario: Registry has no method binding

- **WHEN** a DID uses an unregistered method
- **THEN** resolution returns the existing `methodNotSupported` standard
  failure with the same empty content and JSON representation

#### Scenario: Cache dependency fails closed

- **WHEN** the configured cache policy converts an adapter failure to a closed
  result
- **THEN** resolution returns the existing `internalError` standard failure
  with the same empty content and JSON representation
