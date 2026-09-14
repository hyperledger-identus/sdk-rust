## ADDED Requirements

### Requirement: Code-health evidence separates authored populations

The repository SHALL provide a deterministic, version-pinned audit command and
canonical report format. It SHALL report authored production, external-test,
and inline-test populations separately and SHALL identify generated exclusions.
Inline code SHALL leave production only when syntax-aware evaluation proves its
conditional-compilation predicate false with `test = false`; unknown feature
and target predicates SHALL remain production. Predicate comments SHALL NOT
alter parsing of retained string values; raw string and raw identifier tokens
SHALL be accepted. Nested `cfg_attr` SHALL apply recursively when its predicate
is true, do nothing when false, and remain production when applicability could
change inclusion. A test-only out-of-line module
declaration SHALL recursively classify its ordinary Rust module tree as test
code while preserving nested inline-module context. Only Cargo `tests/` and
`benches/` target trees SHALL be intrinsically external-test code; a file under
`src`, including `src/tests.rs`, SHALL require syntax-proven test-only module
reachability to leave production. Generated exclusion SHALL require an exact
policy path and exact header marker. Outer doc comments immediately preceding a
test-only item SHALL share that item's population. Test-only reachability SHALL
fail closed rather than guess `#[path]` module overrides.
An active or unknown production module edge SHALL override test-only
reachability to the same file and its ordinary module descendants.

#### Scenario: Test-only inline module

- **WHEN** a source item is guarded by `cfg(test)` or an `all` expression that
  is false when `test = false`
- **THEN** its lines and functions appear in inline-test evidence and not the
  production population

#### Scenario: Test is one alternative

- **WHEN** an item uses `cfg(any(test, feature = "diagnostics"))`
- **THEN** the unknown non-test predicate keeps the item in production evidence

#### Scenario: Cfg predicate contains comments and raw tokens

- **WHEN** a cfg predicate contains nested comments, raw string values, or a raw
  identifier such as `r#test`
- **THEN** valid metadata is evaluated without treating comment-like literal
  content as a comment

#### Scenario: Cfg attr generates a false cfg

- **WHEN** `cfg_attr(not(test), cfg(any()))` is evaluated with `test = false`
- **THEN** the generated false cfg makes the item inline-test evidence

#### Scenario: Cfg attr applicability is unknown

- **WHEN** an unknown feature predicate conditionally generates a false cfg
- **THEN** the item remains production because the attribute may not apply

#### Scenario: Test-only out-of-line module

- **WHEN** `cfg(test)` guards `mod guard;` and `guard` declares nested ordinary
  out-of-line modules
- **THEN** the resolved module tree is inline-test evidence and not production

#### Scenario: Test-shaped source filename lacks a test-only declaration

- **WHEN** `src/tests.rs` exists but no definitively test-only module declaration
  reaches it
- **THEN** it remains production evidence

#### Scenario: Nested test helper collides with shipping source

- **WHEN** `cfg(test)` guards `mod tests { mod helper; }` and both
  `src/tests/helper.rs` and `src/helper.rs` exist
- **THEN** only `src/tests/helper.rs` becomes inline-test evidence and the
  shipping `src/helper.rs` remains production

#### Scenario: Test item has outer documentation

- **WHEN** `///` or `/** */` outer documentation immediately precedes a
  definitively test-only item
- **THEN** the documentation is inline-test evidence rather than production

#### Scenario: Test-only brace-delimited item macro

- **WHEN** a definitively test-only item invokes a qualified macro with a
  brace-delimited token tree
- **THEN** the balanced macro item is inline-test evidence and the following
  shipping item remains production

#### Scenario: Test-only module overrides its source path

- **WHEN** a definitively test-only module uses `#[path = "..."]`
- **THEN** v1 audit fails closed rather than resolving a default same-named file

#### Scenario: Module is reachable in test and production configurations

- **WHEN** `cfg(test)` and an active or unknown production predicate declare an
  out-of-line module that resolves to the same file
- **THEN** that file and its ordinary descendants remain production evidence

#### Scenario: Ordinary prose resembles a generated marker

- **WHEN** a Rust comment contains "do not edit" outside an exact policy
  path-and-marker entry
- **THEN** the file remains authored evidence

### Requirement: Baseline evidence is bound to policy and Git content

The policy SHALL pin the baseline revision, authored-source fingerprint and
canonical report digest. Fast validation SHALL resolve the revision and
recompute source fingerprint, generated exclusions, and line/file populations.
Weekly slow validation SHALL use the pinned analyzer version to regenerate and
compare the entire report, including function counts and signals. Report schema
keys SHALL be closed recursively.

#### Scenario: Canonical report field is forged

- **WHEN** revision, fingerprint, population, signal or generated-exclusion
  content is changed while retaining canonical JSON
- **THEN** policy, Git-tree, or full regeneration validation fails closed

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
