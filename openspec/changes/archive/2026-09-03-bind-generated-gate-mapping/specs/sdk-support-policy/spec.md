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
