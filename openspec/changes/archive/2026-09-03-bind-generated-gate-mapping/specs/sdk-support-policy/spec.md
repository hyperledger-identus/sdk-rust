## ADDED Requirements

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
