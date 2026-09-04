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
forms before determining an outer string's terminator.

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
