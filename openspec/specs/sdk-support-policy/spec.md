# sdk-support-policy Specification

## Purpose

Define and continuously verify the SDK's Rust floor, etalon toolchain, host,
target, feature, FFI and performance-commitment states without overstating
compile-only or bootstrap evidence.
## Requirements
### Requirement: One machine-readable policy defines compatibility

The repository SHALL contain one normative machine-readable support policy for
the declared MSRV, pinned development toolchain, host systems, compile-only
targets, eligible packages, feature surfaces, FFI status, binary-size status
and build-time status. It SHALL contain a separate versioned declarative gate
manifest defining the expected Crane operation, toolchain class and structured
Cargo selection for every named compatibility gate. Nix and the offline
validator SHALL consume that same manifest. A human-readable policy SHALL
explain the evidence tiers and SHALL NOT make a stronger claim than the machine
contracts.

#### Scenario: Contributor evaluates a target claim

- **WHEN** a contributor reads the support policy and referenced gate entry
- **THEN** the target has an explicit evidence tier, required gate, eligible
  package set, structured execution selection and limitation

#### Scenario: Policy omits a required dimension

- **WHEN** MSRV, etalon, hosts, targets, features, FFI, size or build time is
  absent from the machine contract
- **THEN** structural validation fails with the missing dimension

#### Scenario: Gate manifest entry is malformed or ambiguous

- **WHEN** a gate repeats a name, uses an unknown field or enum, or combines
  contradictory workspace, package, target or feature modes
- **THEN** offline validation fails closed before Nix evidence is accepted

### Requirement: MSRV and etalon toolchain are independent gates

The SDK SHALL compile every machine-declared default, minimal and opt-in
feature surface using Rust `1.85.0`. It SHALL separately run the pinned
NeoPRISM-etalon nightly `2026-03-18` checks. Passing a feature surface on the
newer toolchain SHALL NOT substitute for the corresponding MSRV gate. The MSRV
Crane builder SHALL be wired to the machine-declared stable toolchain rather
than trusted by variable name.

#### Scenario: Nightly-only language use enters the SDK

- **WHEN** source builds on the etalon nightly but not Rust `1.85.0`
- **THEN** the independent MSRV gate fails

#### Scenario: Opt-in feature raises its Rust floor

- **WHEN** an isolated minimal or opt-in feature surface requires a Rust
  version newer than `1.85.0`
- **THEN** that surface's independent MSRV gate fails even when its nightly
  gate passes

#### Scenario: Etalon pin drifts from policy

- **WHEN** Nix selects a nightly other than the machine-declared etalon
- **THEN** policy validation fails even if compilation succeeds

#### Scenario: MSRV builder is rewired to nightly

- **WHEN** the MSRV-named Crane library wraps the etalon toolchain instead of
  the declared stable toolchain
- **THEN** structural validation fails before nightly results can be accepted
  as MSRV evidence

### Requirement: Evidence tiers do not overstate support

Host-tested systems SHALL run the repository quality gates. Compile-checked
targets SHALL compile only their declared eligible packages with their declared
feature selection and SHALL NOT be described as runtime-tested, certified or
production-supported. Planned targets SHALL have no compatibility promise. A
not-supported surface SHALL not be inferred from a placeholder package.

#### Scenario: Browser or mobile compile check passes

- **WHEN** an eligible package compiles for browser WASM, Android ARM64 or iOS
  ARM64
- **THEN** the evidence proves target compilation only and records that runtime
  integration remains downstream evidence

#### Scenario: Placeholder bindings crate exists

- **WHEN** the inherited bindings package still has no accepted FFI contract
- **THEN** the support policy reports FFI as not supported

### Requirement: Supported feature surfaces are isolated

The target policy SHALL enumerate the default, minimal and opt-in feature
surfaces that are required to compile or test. Checks SHALL exercise compatible
surfaces independently rather than relying only on Cargo feature unification.

#### Scenario: Minimal crypto surface regresses

- **WHEN** `identus-crypto` no longer compiles without default features
- **THEN** its isolated minimal-feature gate fails

#### Scenario: Entropy feature surface regresses

- **WHEN** the empty, deterministic or system-random entropy feature surface
  fails independently
- **THEN** the corresponding feature-matrix gate fails

### Requirement: Size and build-time commitments are explicit

Binary size and build time SHALL carry an explicit policy state. Until an
immutable candidate defines reproducible artifacts and baselines, both SHALL
be measurement-only and SHALL NOT be represented as compatibility budgets.

#### Scenario: Bootstrap checks record duration or artifact size

- **WHEN** CI or a developer observes a build duration or intermediate artifact
  size
- **THEN** the observation does not become a stable threshold or release claim

### Requirement: Configuration drift fails deterministically

An offline repository validator SHALL compare the policy to Cargo MSRV, Nix
toolchain pins, flake host systems, declared target components, eligible
packages, feature sets and the declarative gate manifest. For every feature
gate, the validator SHALL compare the complete effective Cargo package
selection, including workspace exclusions, default feature mode and activated
feature set, without parsing Cargo semantics from Nix expression text.
Workspace-wide surfaces SHALL explicitly select the workspace. Nix SHALL
construct each named Crane check from the same manifest fields. The validator
SHALL bind each gate to its manifest-declared Crane operation and toolchain,
each compile-checked target to its structured Cargo target, and each feature
surface to its complete structured feature selection. Host systems, target
triples, feature-surface names and gate names SHALL be unique. The validator
SHALL run in the structural factory path.

#### Scenario: Cargo MSRV changes alone

- **WHEN** `Cargo.toml` changes the workspace Rust version without a reviewed
  policy update
- **THEN** structural validation fails

#### Scenario: Required gate disappears

- **WHEN** a gate named by the machine policy is removed or renamed in the
  declarative manifest
- **THEN** structural validation fails before the compatibility claim can merge

#### Scenario: Check graph is detached from the flake

- **WHEN** `flake.nix` stops importing the root check module
- **THEN** structural validation rejects the detached execution contract

#### Scenario: Manifest consumption is detached

- **WHEN** the reachable root check module no longer consumes the manifest to
  publish generated checks
- **THEN** structural validation rejects the detached execution contract

#### Scenario: Returned checks are replaced behind a live decoy

- **WHEN** the generator still mentions `checks = generatedChecks` in an
  assertion or another live expression but its returned top-level `checks`
  value is replaced
- **THEN** structural validation rejects the decoy and the unpublished gates

#### Scenario: Nix source contains a gate-name decoy

- **WHEN** a removed manifest gate name remains only in a comment, multiline
  string, interpolated expression or dead `_module.args` value
- **THEN** the decoy cannot satisfy the missing structured gate

#### Scenario: Workspace gate excludes a package

- **WHEN** a workspace-wide gate manifest entry excludes any workspace package
- **THEN** structural validation rejects the gate's incomplete effective
  package selection

#### Scenario: Workspace gate relies on implicit defaults

- **WHEN** a workspace-wide feature gate drops its explicit workspace selector
- **THEN** structural validation rejects the ambiguous package selection

#### Scenario: Policy repeats an identity

- **WHEN** two host, target or feature entries declare the same normative key
- **THEN** structural validation rejects the ambiguous machine policy instead
  of silently choosing one entry

#### Scenario: Target backend feature disappears

- **WHEN** a compile-checked target gate stops selecting a feature required by
  its machine-declared target surface
- **THEN** structural validation fails even when target and package selections
  are unchanged

#### Scenario: Test gate becomes build-only

- **WHEN** a test gate changes from the declared test operation to a build
  operation while retaining its name and Cargo arguments
- **THEN** structural validation rejects the semantic operation drift

#### Scenario: Target token moves outside structured selection

- **WHEN** a target gate selects another triple but retains the declared triple
  only in unrelated Nix text or metadata
- **THEN** structural validation rejects the effective target selection

#### Scenario: Isolated feature silently broadens

- **WHEN** a minimal or isolated gate drops `no_default_features`, selects a
  different package or adds another feature
- **THEN** structural validation rejects the broadened surface even if Nix text
  retains the old evidence token

#### Scenario: Dynamic Nix syntax does not change manifest meaning

- **WHEN** Nix implementation details use interpolation, comments or quoting
  unrelated to manifest-derived gate construction
- **THEN** offline validation remains deterministic and does not interpret those
  expressions as Cargo selection semantics

### Requirement: Validator performance remains observable

The repository SHALL provide a deterministic benchmark that runs at least 20
successful process-cold and in-process-warm support-policy validations and
reports sample count, platform, revision, p50 and p95. Linux and macOS CI SHALL
run the benchmark as diagnostic evidence. These observations SHALL NOT become
a compatibility or release budget during bootstrap.

#### Scenario: Agent evaluates validator overhead

- **WHEN** the benchmark runs on a supported host
- **THEN** it reports at least 20 samples for both modes and compares the result
  with the recorded #23 baseline where the same platform is available

#### Scenario: Runner timing varies

- **WHEN** normal hosted-runner variance changes a percentile
- **THEN** the result remains diagnostic unless it breaches a broad documented
  pathological-regression ceiling applied to the robust p50 measurements;
  isolated p95 outliers remain reported diagnostics

### Requirement: Generated gate mappings preserve manifest identity

The offline support-policy validator SHALL require the Nix gate generator to
derive every mapped attribute name from the current manifest entry's `name`
field and every mapped attribute value from `makeGate` applied to that same
entry. The mapping and attribute-set conversion functions SHALL resolve to
`map` and `listToAttrs` inherited from `pkgs.lib`, not local replacements. The
helper inheritance, manifest binding, and complete mapping SHALL be immediate
bindings of the same outer `perSystem` `let` scope, connected from
`manifest.gates` through `listToAttrs` to that scope's returned top-level
`checks` value. That scope SHALL NOT shadow the trusted `builtins` or `pkgs`
roots, and the canonical module result SHALL NOT be wrapped in an enclosing
lexical scope that can shadow those roots. Lexical scanning SHALL distinguish
comments from comment delimiters inside valid Nix strings and SHALL recognize
escaped delimiters in indented strings.
Quoted or dynamic immediate binding roots SHALL NOT bypass trusted-root
validation. Interpolation scanning SHALL balance nested expressions and string
forms before determining an outer string's terminator. Root and reachable
repository-local imports SHALL form a complete statically traversable graph;
only static external `inputs.<name>.flakeModule` entries MAY remain outside the
repository. Outside the independently constrained gate generator, reachable
modules SHALL NOT inherit imports or protected keys, use reflective attribute
access, or dynamically construct attribute sets. Import discovery SHALL inspect
only immediate bindings of each returned module attribute set. Reachable local
modules SHALL NOT evaluate `import` expressions or bind explicit top-level
`config`; module dependencies SHALL use the traversed literal `imports` list.
Inherited `_type` SHALL be treated as a raw priority record.

#### Scenario: Constant mapped name collapses the gate graph

- **WHEN** the generator replaces the current entry's name with a constant
  attribute name
- **THEN** structural validation fails before multiple manifest gates can
  collapse into one published Nix check

#### Scenario: Mapped value bypasses gate construction

- **WHEN** the generator maps a manifest entry to a value not produced by
  `makeGate` for that same entry
- **THEN** structural validation rejects the detached gate implementation

#### Scenario: Mapping helper is locally replaced

- **WHEN** `map` or `listToAttrs` is removed from the `pkgs.lib` inheritance
  and replaced by a local function that collapses or discards manifest entries
- **THEN** structural validation rejects the shadowed mapping helper

#### Scenario: Mapping inputs are shadowed in a returned inner scope

- **WHEN** an inner `let` redefines `map`, `listToAttrs`, or `manifest` and
  contains a canonical-looking mapping while outer bindings remain as decoys
- **THEN** structural validation rejects the cross-scope mapping contract

#### Scenario: Trusted root is shadowed in the accepted scope

- **WHEN** an immediate `let` binding redefines `builtins` or `pkgs`
- **THEN** structural validation rejects the untrusted root before accepting
  manifest parsing or helper inheritance

#### Scenario: Comment delimiter belongs to string data

- **WHEN** an unrelated valid Nix string contains `#`, `/*`, or `*/`
- **THEN** lexical preprocessing preserves the string and the canonical gate
  mapping remains accepted

#### Scenario: Enclosing scope shadows a trusted root

- **WHEN** a `let` around the returned module redefines `builtins` or another
  trusted root before `perSystem`
- **THEN** structural validation rejects the non-canonical module wrapper

#### Scenario: Indented string contains an escaped delimiter

- **WHEN** valid unrelated string data uses `''${`, `'''`, or another Nix
  indented-string escape before a comment marker
- **THEN** lexical preprocessing finds the true closing delimiter and preserves
  canonical gate validation

#### Scenario: Quoted binding normalizes to a trusted root

- **WHEN** an immediate binding uses a quoted or dynamic first path component
  that can resolve to `builtins` or `pkgs`
- **THEN** structural validation rejects the ambiguous binding root

#### Scenario: Interpolation contains a nested string

- **WHEN** a quoted or indented string interpolation contains nested strings,
  braces, comments, or comment-marker data
- **THEN** lexical preprocessing returns to the outer string only after the
  complete interpolation and preserves canonical gate validation

#### Scenario: Per-system formal shadows the global builtins root

- **WHEN** the immediate `perSystem` function formals bind `builtins`
- **THEN** structural validation rejects the injected trusted root

#### Scenario: Path tokens contain scope keywords

- **WHEN** unrelated valid path or URI tokens contain `let` or `in` components
- **THEN** lexical preprocessing does not open or close a lexical scope

#### Scenario: Identifier contains adjacent apostrophes

- **WHEN** an unrelated valid identifier contains adjacent apostrophes
- **THEN** lexical preprocessing does not treat those apostrophes as an
  indented-string opener

#### Scenario: Interpolated path suffix contains a scope keyword

- **WHEN** a valid path contains `${...}` followed by a `let` or `in` component
- **THEN** lexical preprocessing resumes the path token after interpolation and
  does not interpret the suffix as a lexical scope

#### Scenario: Unprefixed relative path contains a scope keyword

- **WHEN** a valid unprefixed relative path contains a `let` or `in` component
- **THEN** lexical preprocessing recognizes the complete path token and does
  not interpret its components as lexical scopes

#### Scenario: Unprefixed relative path begins with punctuation

- **WHEN** a valid unprefixed relative path begins with punctuation accepted
  by the Nix path-prefix grammar and contains a `let` or `in` component
- **THEN** lexical preprocessing recognizes the complete path token and does
  not interpret its components as lexical scopes

#### Scenario: Indented-string control escape precedes interpolation syntax

- **WHEN** `''\` escapes a character that would otherwise begin `${...}`
- **THEN** lexical preprocessing consumes the escaped character as string data
  and does not open interpolation

#### Scenario: Package provider replaces an inherited mapping helper

- **WHEN** `flake.nix` wraps or extends the package set supplied to
  `perSystem` so `pkgs.lib.map` or `pkgs.lib.listToAttrs` can be replaced
- **THEN** structural validation rejects the non-canonical package provider
  before trusting the inherited helpers

#### Scenario: Line comment immediately follows a path token

- **WHEN** a valid path is immediately followed by a `#` line comment whose
  data contains a `let` or `in` token
- **THEN** path scanning stops at `#` and comment preprocessing prevents the
  comment data from changing lexical scope

#### Scenario: Enclosing scope replaces the builtin import

- **WHEN** the root flake or its `outputs` result is wrapped in a lexical scope
  that redefines `import` while retaining a canonical-looking package provider
- **THEN** structural validation rejects the non-canonical root or outputs
  shape before trusting the package provider

#### Scenario: Check wrapper forces generated checks away

- **WHEN** the module that imports the generated checks uses a priority
  override such as `mkForce` or `mkOverride` for its checks contribution
- **THEN** structural validation rejects the wrapper before module merging can
  erase the required generated gates

#### Scenario: Sibling module forces generated checks away

- **WHEN** another repository-local module reachable from the root import
  graph uses a module-priority override that can replace checks
- **THEN** structural validation rejects the competing module before merging
  can erase the required generated gates

#### Scenario: URI literal omits double slashes

- **WHEN** a valid general URI such as `mailto:let@example.org` contains a
  lexical scope keyword without using `//`
- **THEN** lexical preprocessing recognizes the complete URI token and does
  not interpret its data as a lexical scope

#### Scenario: Quoted priority constructor erases generated checks

- **WHEN** a reachable local module selects `mkForce` or `mkOverride` through
  a quoted attribute name
- **THEN** structural validation rejects the effective priority constructor
  before module merging can erase generated gates

#### Scenario: Parent-relative module contains a priority override

- **WHEN** a reachable module imports a repository-local module using `../`
  and that module contains a priority override
- **THEN** graph traversal resolves the complete parent-relative path and
  structural validation rejects the override

#### Scenario: Raw override record erases generated checks

- **WHEN** a reachable local module contributes a raw module value whose
  `_type` is `"override"`
- **THEN** structural validation rejects the semantic priority override even
  when no priority-constructor identifier appears in source

#### Scenario: Local module disables the gate generator

- **WHEN** a repository-local module contributes `disabledModules` that can
  remove the required gate generator
- **THEN** structural validation rejects the disabling contribution before
  evaluating the effective module graph

#### Scenario: Import graph contains a computed edge

- **WHEN** a reachable local module assigns `imports` from an alias or another
  expression instead of a direct literal list
- **THEN** graph traversal fails closed rather than accepting an incomplete
  repository-local module graph

#### Scenario: Raw module tag uses another string form

- **WHEN** a reachable local module contains a raw `_type` tag using an
  indented or computed Nix string
- **THEN** structural validation rejects the raw module value independently of
  the tag value's source spelling

#### Scenario: Import binding uses a static quoted name

- **WHEN** a reachable module spells `imports` as a statically quoted attribute
- **THEN** graph traversal normalizes the binding and traverses every local
  literal edge

#### Scenario: Import path contains interpolation

- **WHEN** a reachable imports list contains a local path with `${...}`
- **THEN** graph traversal fails closed instead of resolving the unevaluated
  source spelling as a filesystem path

#### Scenario: Sibling computes an attribute selection

- **WHEN** a reachable module outside the canonical gate generator uses a
  computed attribute selection or contributes another `checks` binding
- **THEN** structural validation rejects the ambiguous module contribution
  before it can erase generated checks

#### Scenario: Root import expression hides a local edge

- **WHEN** the root import list contains a computed or otherwise unresolved
  entry instead of a repository-local literal path or static external flake
  module
- **THEN** structural validation rejects the incomplete root graph

#### Scenario: Reachable module inherits imports

- **WHEN** a reachable local module inherits `imports` from another expression
- **THEN** graph traversal fails closed because it cannot prove every effective
  repository-local edge

#### Scenario: Priority helper is retrieved reflectively

- **WHEN** a reachable module outside the canonical generator obtains a
  priority constructor through `getAttr`
- **THEN** structural validation rejects the reflective attribute access

#### Scenario: Protected module key is constructed dynamically

- **WHEN** a reachable module outside the canonical generator uses
  `listToAttrs` to synthesize `checks`, `disabledModules`, or another effective
  module key
- **THEN** structural validation rejects the dynamic attribute-set construction

#### Scenario: Protected key is inherited or computed

- **WHEN** a reachable module inherits a protected module key or computes a raw
  `_type` attribute name
- **THEN** structural validation rejects the ambiguous module contribution

#### Scenario: Nested import data impersonates a graph edge

- **WHEN** a module removes its effective generator import but retains a
  canonical-looking `imports` assignment inside inert nested data
- **THEN** graph traversal ignores the nested decoy and rejects the missing
  effective generator edge

#### Scenario: Imported explicit config erases generated checks

- **WHEN** a reachable module binds `config` from an imported local fragment
- **THEN** structural validation rejects both the explicit config composition
  and executable import before the hidden checks can merge

#### Scenario: Priority record fields are inherited

- **WHEN** a checks value inherits `_type`, `priority`, and `content` from a
  module priority constructor result
- **THEN** structural validation rejects the inherited raw priority record

#### Scenario: Inert mkFlake call impersonates root imports

- **WHEN** the canonical `outputs` call uses an unresolved imports expression
  and inert nested data contains a second canonical-looking `mkFlake` call
- **THEN** root graph discovery remains anchored to the effective canonical
  call and rejects the unresolved imports

#### Scenario: Imports use a statically computed string attribute name

- **WHEN** a reachable module binds `imports` using the valid static
  `${''imports''} = [...]` or `${"imports"} = [...]` spelling
- **THEN** graph traversal follows the imported repository-local module and
  applies all protected-surface checks to it

#### Scenario: Mapped gate constructor is replaced with a no-op

- **WHEN** the mapped call remains `makeGate gate` but `makeGate` or one of its
  immediate Cargo argument helpers no longer performs canonical
  manifest-selected Crane dispatch
- **THEN** structural validation rejects the disconnected gate implementation

#### Scenario: External flake module resolves from repository-local source

- **WHEN** an allowed `inputs.<name>.flakeModule` root import is retained but
  that input's direct URL is replaced with a repository-local path
- **THEN** structural validation rejects the non-canonical external module
  provenance before trusting the root import exemption

#### Scenario: Crane operation provider is replaced upstream

- **WHEN** the gate generator remains canonical but `nix/rust-toolchain.nix`
  wraps or replaces the etalon or MSRV Crane library operations
- **THEN** structural validation rejects the non-canonical direct toolchain and
  Crane provider contract

#### Scenario: Nested provider decoy impersonates the effective provider

- **WHEN** the effective root `perSystem` package provider is changed and inert
  nested data retains a canonical-looking provider
- **THEN** structural validation validates only the immediate `perSystem`
  statement of the canonical `mkFlake` root module and rejects the mutation

#### Scenario: Executable builtin uses a statically quoted selector

- **WHEN** a reachable local module invokes `getAttr`, `listToAttrs`, `import`,
  or another protected builtin through a statically quoted attribute selection
- **THEN** structural validation normalizes the selector and applies the same
  fail-closed rule as for its bare spelling

#### Scenario: Quoted attribute name contains static interpolation

- **WHEN** a reachable module spells `imports` as `"${"imports"}"` and points
  it at a repository-local module
- **THEN** graph traversal normalizes the static name, follows the local edge,
  and applies every protected-surface check to the imported module

#### Scenario: Canonical provider result has trailing composition

- **WHEN** the canonical-looking `perSystem` provider result is followed by a
  merge operator or any other expression before the binding terminator
- **THEN** structural validation rejects the trailing composition instead of
  accepting only its first attribute set

#### Scenario: Locked trusted input changes provenance

- **WHEN** a root input retains its canonical `flake.nix` URL but its direct
  `flake.lock` mapping or locked GitHub owner/repository changes
- **THEN** structural validation rejects the lock graph before trusting that
  input as a gate provider or external module

#### Scenario: Indented attribute name contains static interpolation

- **WHEN** a reachable module spells a protected binding such as `inputs` as
  `''${"inputs"}''`
- **THEN** structural validation normalizes the static name and applies the
  same trusted-root shadowing rule as for the bare spelling
