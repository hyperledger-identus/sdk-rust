## ADDED Requirements

### Requirement: Code-health evidence separates authored populations

The repository SHALL provide a deterministic, version-pinned audit command and
canonical report format. It SHALL report authored production, external-test,
and inline-test populations separately and SHALL identify generated exclusions.
Inline code SHALL leave production only when syntax-aware evaluation proves its
conditional-compilation predicate false with `test = false`; unknown feature
and target predicates SHALL remain production. Predicate comments SHALL NOT
alter parsing of retained string values; raw string and raw identifier tokens
and Unicode cfg identifiers SHALL be accepted. Stable `true` and `false` cfg
literals SHALL be evaluated exactly. Syntax outside the pinned semantic
evaluator SHALL remain production. Nested `cfg_attr` SHALL apply recursively
when its predicate is true, avoid parsing applied attributes when false, and
remain production when applicability or applied syntax could change inclusion.
A test-only out-of-line module
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
An inline-test span SHALL exist only when the v1 whitelist proves a semicolon,
zero-relative-depth comma, recognized block-item, or recognized item-macro end.
Ambiguous angles or container delimiters and unsupported nested/member,
statement, arm, generic, or macro forms SHALL remain production. A mixed
test/shipping source line SHALL be production. No test span SHALL consume a
following production node. Attribute-like tokens inside a macro definition or
invocation token tree SHALL remain production; only an outer cfg applying to a
recognized macro invocation may classify that invocation inline-test.
Inner `#![cfg(...)]` scopes SHALL remain production in v1.
Only byte-contiguous `#[` attributes SHALL enter the v1 classifier. Rust-valid
whitespace-separated forms such as `# [cfg(test)]` SHALL remain production and
SHALL NOT seed test-only out-of-line module inheritance. Issue #275 owns exact
classification of those forms.

#### Scenario: Attribute opener contains whitespace

- **WHEN** a test cfg attribute is written with whitespace between `#` and `[`
- **THEN** its item and any out-of-line module tree remain production evidence

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

#### Scenario: Cfg syntax exceeds the pinned evaluator

- **WHEN** valid current or future cfg metadata uses an unrecognized predicate
  form, or an unknown `cfg_attr` condition may apply such metadata
- **THEN** the item remains production and an inactive `cfg_attr` branch is not
  evaluated

#### Scenario: Unsupported comma-less member precedes shipping code

- **WHEN** a definitively test-only final field, variant, or parameter omits its
  trailing comma before the enclosing delimiter
- **THEN** v1 retains the member and following shipping node in production

#### Scenario: Unsupported nested block expression precedes shipping code

- **WHEN** a definitively test-only block statement or comma-less block-bodied
  match arm precedes a production statement or arm
- **THEN** v1 retains the unsupported nested construct and following node in
  production

#### Scenario: Test and shipping code share a line

- **WHEN** a proven test-only node and shipping code have non-whitespace source
  characters on the same source line
- **THEN** the whole line and every function starting on it remain production

#### Scenario: Macro consumes a cfg-looking token

- **WHEN** a macro definition or invocation token tree contains tokens that
  resemble a cfg attribute on a shipping item
- **THEN** v1 keeps the token-tree source in production rather than treating it
  as an active source attribute

#### Scenario: Inner cfg scopes a nested source region

- **WHEN** a module or block contains an inner `#![cfg(test)]` attribute
- **THEN** v1 conservatively retains that scope in production and issue #275
  owns broader syntax classification

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
Slow validation SHALL use the pinned analyzer version to regenerate and compare
the entire report, including function counts and signals, when invoked locally
or by explicit external scheduling. The repository SHALL NOT treat a workflow
cron declaration as active execution evidence while reserved empty `main`
remains GitHub's default branch and the workflow lives on `develop`. Issue #276
SHALL own schedule activation. Report schema keys SHALL be closed recursively.

#### Scenario: Default branch does not contain the slow workflow

- **WHEN** reserved empty `main` remains GitHub's default branch and the
  ready-to-run slow workflow exists on `develop`
- **THEN** documentation identifies local or external invocation as the active
  path and does not claim that GitHub is executing the declared schedule

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
