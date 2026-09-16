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

The classifier SHALL cover ordinary and associated items, declaration,
struct-literal and struct-pattern fields, variants,
function/method/closure/bare-function-type parameters and variadics, generic
parameters,
statements/expressions, match arms, and represented macro nodes. Macro token
streams SHALL remain opaque. A source line SHALL be
inline-test only when every non-whitespace authored byte is proven test-only;
mixed lines remain production. Raw and ordinary module identifiers SHALL map
to the same path. Literal path overrides MAY be supported only with contained,
unambiguous resolution, including non-root source modules and path-adjusted
inline modules. Resolution SHALL preserve whether a source is entered as a
target root or nested module. Target roots SHALL come from Cargo manifests
bound to the audited revision, not source filenames or function names, and a
target root SHALL remain production even if a test-only edge reaches it.
Generated sources excluded from metrics SHALL remain available to module
resolution. Active or unknown
production reachability SHALL win over test-only reachability and propagate to
descendants. A module edge proven disabled in both production and test
configurations SHALL be omitted without resolving its source file.

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

#### Scenario: Module is disabled in every audited configuration

- **WHEN** a module edge is false with both `test = false` and `test = true`
- **THEN** the edge is omitted and its source file is not required for
  resolution

#### Scenario: Unrelated nested path metadata is not an override

- **WHEN** an unknown `cfg_attr` applies metadata such as `cfg(path)` but no
  direct `path = "..."` attribute
- **THEN** ordinary module resolution remains active and no conditional-path
  ambiguity is reported

#### Scenario: One source has multiple module roles

- **WHEN** one source is reached both as a target root and as a nested module
- **THEN** test edges are resolved in both roles and every distinct reachable
  fixture is classified

#### Scenario: Rust source cannot be represented safely

- **WHEN** parsing, span projection, or module resolution is invalid,
  ambiguous, escaping, or exceeds a protocol bound
- **THEN** the audit fails with a path/location diagnostic or retains the
  affected source as production; it never excludes it as test-only

#### Scenario: Nested Rust contexts preserve cfg and path state

- **WHEN** a cfg-gated associated item, variant, declaration or struct-literal
  field, ordinary/bare-function parameter, generic parameter, or path-adjusted
  inline module contains nested syntax or an out-of-line module declaration
- **THEN** the classifier preserves the inherited cfg and module-directory
  context defined by Rust

#### Scenario: Generated module is excluded from metrics

- **WHEN** an exact allowlisted generated source is reached by an authored
  module declaration
- **THEN** it remains available to module resolution while its lines remain
  absent from authored population metrics

#### Scenario: Test configuration selects a module path

- **WHEN** a proven test-only module uses `cfg_attr(test, path = "...")`
- **THEN** reachability follows the test-selected path; an unknown selection or
  a production edge whose path differs by configuration fails closed

#### Scenario: Binary target reaches a nested module

- **WHEN** a file-based target reaches another `src/bin` source as a module
- **THEN** the target's children resolve from the target directory, the nested
  source's children resolve from its module directory, and an independently
  shipping target root remains production

#### Scenario: Nested helper is not inferred as a target

- **WHEN** a nested module contains an ordinary function named `main`
- **THEN** only Cargo-declared or auto-discovered targets use root resolution
  and the nested source retains nested-module resolution

#### Scenario: Closure parameter is test-only

- **WHEN** a closure pattern parameter is removed by `cfg(test)`
- **THEN** its complete authored AST span is inline-test evidence without
  consuming a shipping parameter

#### Scenario: Pattern fields and variadic parameters are test-only

- **WHEN** a struct-pattern field, ordinary variadic, or bare-function-type
  parameter or variadic is removed by `cfg(test)`
- **THEN** its complete authored AST span is inline-test evidence without
  consuming a shipping field or parameter

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
- **THEN** full validation proves the baseline source revision is an ancestor
  of the reviewed checkout and it remains reachable from `develop` regardless
  of whether integration uses merge, squash, or rebase

#### Scenario: Maximum bounded projection remains linear

- **WHEN** a source contains many disjoint test-only spans
- **THEN** line projection advances monotonically through merged spans rather
  than searching the full span set for every authored byte

#### Scenario: Conditional module path is unresolved

- **WHEN** an active or unknown `cfg_attr` may apply a module `path` override
- **THEN** classification fails closed rather than selecting only the default
  or conditional candidate

#### Scenario: Test-only state is nested below an expression

- **WHEN** a local module is nested under a test-only statement, expression,
  or match arm
- **THEN** the module inherits test-only reachability unless another active or
  unknown production path reaches it
