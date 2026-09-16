# code-health-governance delta

## MODIFIED Requirements

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
and locked implementation evidence in addition to analyzer identity. Fast
validation SHALL execute the small classifier through the pinned Nix shell and
recompute Git-tree source populations without invoking the heavyweight metric
engine. Weekly/manual validation SHALL use the same classifier and the pinned
metric engine. A parser or protocol change SHALL require a governed baseline
migration and exhaustive explained population delta.

#### Scenario: Fast source validation runs

- **WHEN** protected PR validation checks canonical code-health evidence
- **THEN** the pinned AST classifier recomputes source populations while
  `rust-code-analysis-cli` remains absent from the fast execution path

#### Scenario: Parser migration changes a population

- **WHEN** the v2 AST population differs from the v1 conservative baseline
- **THEN** a checked-in migration report names and explains every delta before
  the new baseline can be accepted
