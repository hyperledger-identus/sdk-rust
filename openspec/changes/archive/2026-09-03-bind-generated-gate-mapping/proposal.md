# Bind generated gate mapping

## Why

Post-merge review of issue #24 and PR #60 proved that the support-policy
validator accepts an arbitrary `map` body between `manifest.gates` and
`generatedChecks`. A constant mapped name can collapse 23 gates into one, and
a detached mapped value can stop using `makeGate`, while structural validation
still reports success.

## What changes

- Require every generated attribute name to come from the current manifest
  gate's `name` field.
- Require every generated attribute value to come from `makeGate gate`.
- Bind `makeGate` and its immediate Cargo argument helpers to the canonical
  manifest-driven Crane dispatch implementation.
- Require the manifest binding, mapping helpers, generated mapping, and
  published result to belong to the same immediate `perSystem` `let` scope.
- Reject immediate bindings that shadow the trusted `builtins` or `pkgs`
  roots used by that contract.
- Reject quoted or dynamic immediate binding roots that can normalize to a
  trusted identifier.
- Require `perSystem` to be the direct canonical module result so an enclosing
  lexical scope cannot shadow trusted roots.
- Tokenize comments with string awareness so comment markers in unrelated
  valid Nix strings cannot corrupt structural validation.
- Recognize Nix indented-string escape prefixes before treating `''` as the
  closing delimiter.
- Track interpolation braces and nested strings before closing outer quoted or
  indented strings.
- Reject a `builtins` binding in the immediate `perSystem` function formals.
- Distinguish path/URI tokens and apostrophes inside identifiers from lexical
  `let`/`in` keywords and indented-string delimiters.
- Continue path-token scanning across `${...}` interpolation boundaries.
- Recognize unprefixed relative paths containing `/`, including tokens whose
  first component begins with valid punctuation, as path tokens.
- Require the `perSystem` `pkgs` provider to be the canonical direct nixpkgs
  import before trusting helpers inherited from `pkgs.lib`.
- Require a plain root flake attribute set and direct `outputs` result so an
  enclosing lexical scope cannot replace the builtin `import`.
- Stop path tokens at a Nix line-comment marker so comment preprocessing owns
  the remaining source line.
- Require the check-wrapper module to contribute a plain checks attribute set
  without priority overrides that can erase imported generated checks.
- Reject priority overrides throughout the recursively reachable local flake
  module graph, not only in the check wrapper.
- Recognize quoted priority-constructor attribute selections and raw module
  override records, not only bare `mkForce` or `mkOverride` identifiers.
- Resolve both child-relative and parent-relative literal imports while
  traversing the repository-local module graph.
- Reject repository-local `disabledModules` contributions that can remove the
  required generator from the effective graph.
- Fail closed when a reachable module's `imports` value is not a directly
  traversable literal list.
- Normalize static quoted attribute bindings when discovering import edges.
- Reject interpolated local import paths because their effective filesystem
  targets cannot be proven by source traversal.
- Reject computed attribute selections and competing `checks` contributions
  outside the canonical, independently validated gate modules.
- Validate the complete root import expression, allowing only explicit static
  input modules in addition to repository-local literal paths.
- Reject inherited `imports` contributions because their effective graph
  cannot be traversed from source.
- Reject reflective attribute access and dynamic attribute-set construction in
  reachable local modules outside the canonical generator.
- Treat inherited `checks` and `disabledModules` contributions as protected
  module-surface mutations, not only direct assignments.
- Discover imports only from immediate bindings of the returned module
  attribute set so nested data cannot impersonate an effective graph edge.
- Reject executable `import` expressions and explicit top-level `config`
  composition in reachable repository modules; local module dependencies must
  use the statically traversed `imports` list.
- Treat inherited `_type` fields as raw module priority records.
- Anchor root import discovery to the already-validated canonical `mkFlake`
  call so an inert second call cannot impersonate the effective graph.
- Normalize statically computed indented-string attribute names when selecting
  immediate module `imports` bindings.
- Reject raw module `_type` tags independent of whether their values use
  double-quoted, indented, or computed Nix strings.
- Recognize general Nix scheme URI literals whose scheme is followed directly
  by URI data without `//`.
- Consume the escaped character after an indented-string `''\` prefix.
- Keep richer path classification within the existing hosted warm-latency
  regression ceiling.
- Cache pure module-source binding, override, delimiter, and import analyses so
  the closed graph contract retains its hosted warm-latency ceiling.
- Bind external flake-module exemptions to canonical direct input URLs and
  both Crane libraries to canonical direct toolchain construction.
- Anchor package-provider validation to the effective root module's immediate
  `perSystem` statement.
- Normalize statically quoted executable builtin selections and quoted static
  interpolations used as protected attribute names.
- Require the canonical provider result attribute set to consume the complete
  `perSystem` expression before its binding terminator.
- Add exact fail-closed regressions for constant-name collapse and detached
  mapped values, helper replacement, and nested input shadowing.
- Record the contract and verification evidence without changing the gate
  manifest, toolchains, support claims, Rust APIs, or downstream repositories.

## Capabilities

### Modified capabilities

- `sdk-support-policy`: bind the Nix generator's mapped names and values to
  each manifest entry before structural validation can pass.

## Impact

The change is limited to repository factory validation, its tests, and
contract evidence. The canonical Nix output does not change. No public or wire
compatibility, dependency, security, release, chain, or consumer behavior is
affected.
