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
- Consume the escaped character after an indented-string `''\` prefix.
- Keep richer path classification within the existing hosted warm-latency
  regression ceiling.
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
