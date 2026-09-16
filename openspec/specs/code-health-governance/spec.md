# code-health-governance Specification

## Purpose
TBD - created by archiving change establish-code-health-quality-budget. Update Purpose after archive.
## Requirements
### Requirement: Code-health evidence separates authored populations

The repository SHALL use a non-published Rust classifier based on the locked
workspace `syn`/`proc-macro2` family for authored Rust population semantics.
The classifier SHALL own cfg/cfg_attr evaluation, AST node boundaries, line
projection, module resolution, and reachability through a bounded versioned
protocol. Python orchestration SHALL NOT parse Rust boundaries. Unknown or
unsupported inclusion SHALL remain production or fail with an actionable audit
error; it SHALL never silently become test-only.

The classifier SHALL cover ordinary items, fields, variants, parameters,
generic parameters, statements/expressions, match arms, and represented macro
nodes. Macro token streams SHALL remain opaque. A source line SHALL be
inline-test only when every non-whitespace authored byte is proven test-only;
mixed lines remain production. Raw and ordinary module identifiers SHALL map
to the same path. Literal path overrides MAY be supported only with contained,
unambiguous resolution. Active or unknown production reachability SHALL win
over test-only reachability and propagate to descendants.

#### Scenario: Full Rust node syntax is classified

- **WHEN** cfg attributes guard supported items, fields, variants, parameters,
  statements, match arms, or macro nodes containing generics, shifts, labels,
  Unicode identifiers, nested comments, raw strings, or recursive cfg_attr
- **THEN** AST boundaries determine the candidate span without consuming a
  following shipping node

#### Scenario: Mixed source line remains production

- **WHEN** a proven test-only AST node and any other authored non-whitespace
  source share a line
- **THEN** the entire line remains production evidence

#### Scenario: Shared module has production reachability

- **WHEN** raw/ordinary declarations or path overrides reach one file through
  both test-only and active/unknown production edges
- **THEN** the file and its reachable descendants remain production evidence

#### Scenario: Rust source cannot be represented safely

- **WHEN** parsing, span projection, or module resolution is invalid,
  ambiguous, escaping, or exceeds a protocol bound
- **THEN** the audit fails with a path/location diagnostic or retains the
  affected source as production; it never excludes it as test-only

### Requirement: Baseline evidence is bound to policy and Git content

The canonical policy/report SHALL record classifier name, protocol version,
locked implementation evidence, and an exact digest of per-file population
projection in addition to analyzer identity. Fast validation SHALL execute the
small classifier through the pinned Nix shell and recompute that complete
Git-tree projection without invoking the heavyweight metric engine. Aggregate
counts alone SHALL NOT satisfy the binding. Weekly/manual validation SHALL use
the same classifier and the pinned metric engine. A parser or protocol change
SHALL require a governed baseline migration and exhaustive explained
population delta. The baseline source revision SHALL be a durable ancestor of
the target branch. It MAY predate the classifier implementation because the
classifier identity, protocol, locked dependencies, and exact projection
digest independently bind its interpretation.

#### Scenario: Fast source validation runs

- **WHEN** protected PR validation checks canonical code-health evidence
- **THEN** the pinned AST classifier recomputes source populations while
  `rust-code-analysis-cli` remains absent from the fast execution path

#### Scenario: Parser migration changes a population

- **WHEN** the v2 AST population differs from the v1 conservative baseline
- **THEN** a checked-in migration report names and explains every delta before
  the new baseline can be accepted

#### Scenario: Per-file lines move without changing totals

- **WHEN** classifier output changes exact inline-test line sets while retaining
  the same aggregate file and line counts
- **THEN** fast validation rejects the projection digest mismatch

#### Scenario: Baseline branch is removed after merge

- **WHEN** a migration branch will be deleted after integration
- **THEN** the baseline source revision remains reachable from `develop`
  regardless of whether integration uses merge, squash, or rebase

#### Scenario: Conditional module path is unresolved

- **WHEN** an active or unknown `cfg_attr` may apply a module `path` override
- **THEN** classification fails closed rather than selecting only the default
  or conditional candidate

#### Scenario: Test-only state is nested below an expression

- **WHEN** a local module is nested under a test-only statement, expression,
  or match arm
- **THEN** the module inherits test-only reachability unless another active or
  unknown production path reaches it

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
