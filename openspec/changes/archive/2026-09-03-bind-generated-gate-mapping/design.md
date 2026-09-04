# Design: bind generated gate mapping

## Context

The generator parses `gates.toml`, maps `manifest.gates` into name/value pairs,
converts them with `listToAttrs`, and returns the result as `checks`. The
validator already proves the parse, collection, and returned publication
links, but its mapping expression accepts any body. Issue #61 closes that
remaining structural gap.

## Goals

- Fail closed when mapped names are constant or otherwise detached from the
  current gate.
- Fail closed when mapped values are detached from `makeGate gate`.
- Preserve fast, deterministic, offline validation and diagnostic benchmark
  behavior.

## Non-goals

- Changing the declarative manifest or any generated derivation.
- Implementing a general Nix parser or evaluator in Python.
- Changing SDK APIs, targets, features, toolchains, or consumer repositories.

## Decision

The narrow wiring validator will isolate the immediate outer `perSystem` `let`
body, split only its top-level statements, and recognize the canonical mapping
expression as one complete statement: `map (gate: { inherit (gate) name; value
= makeGate gate; }) manifest.gates`, passed directly to `listToAttrs` and bound
to `generatedChecks`. The manifest binding and `inherit (pkgs.lib)` statement
must be top-level statements in that same scope, and the outer `in` expression
must return `checks = generatedChecks`. Consequently an inner `let` cannot
redefine `map`, `listToAttrs`, or `manifest` and supply the accepted mapping.
The same immediate scope must not bind `builtins` or `pkgs`, because Nix `let`
bindings are recursive and could otherwise shadow the trusted roots used by
the manifest and helper-origin checks.
Static quoted names and dynamic first path components are also disallowed for
immediate bindings. The canonical generator needs neither form, and rejecting
them prevents normalization or interpolation from reconstructing a trusted
root outside the bare-identifier check.
The file prefix is constrained to the canonical module lambda and direct
attribute-set result, with `perSystem` as that result's binding. This prevents
an enclosing `let` or other lexical wrapper from rebinding roots before the
validated scope.

Source masking first identifies quoted and indented strings, then removes only
line and block comments outside those strings while preserving offsets. Scope
scanning masks the same strings. A URL fragment or comment delimiter in an
unrelated string therefore remains valid source data rather than truncating
the parser input or becoming a structural decoy.
Within an indented string, `''${`, `'''`, and `''\` are escape prefixes rather
than closing delimiters and are skipped accordingly before searching for the
true terminating `''`.
Unescaped `${...}` enters an interpolation scanner that balances braces,
skips comments, and recursively consumes nested quoted or indented strings.
An inner string delimiter therefore cannot terminate its outer string.
The accepted `perSystem` formal set is parsed as simple identifier arguments;
`builtins` is rejected because the mapping contract requires the global root,
while the canonical `pkgs` argument remains required for `pkgs.lib` helpers.
The root flake must supply that argument through the canonical direct
`import nixpkgs` expression with the current system and Rust overlay. A wrapped
or extended package set is rejected because it can replace `lib.map` or
`lib.listToAttrs` before the generator inherits them.
The root flake itself must be a plain attribute set, and the `outputs` lambda
must return `flake-parts.lib.mkFlake` directly. This excludes an enclosing
`let` that can shadow the builtin `import` while preserving the provider's
canonical-looking text.
The `nix/checks/default.nix` wrapper must define its local checks as one plain
attribute set and must not use module-priority constructors such as `mkForce`
or `mkOverride`. Such a priority override can otherwise win module merging and
erase every generated check despite retaining the generator import.
Every repository-local module reachable from the root flake's imports is
walked recursively and held to the same no-priority-override rule. A sibling or
nested module therefore cannot erase generated checks during flake-parts
composition.
Literal import discovery preserves the complete `./` or `../` prefix and
resolves it against the importing module before enforcing the repository
boundary. Priority detection operates on effective constructor positions, so
quoted selections such as `pkgs.lib."mkForce"` cannot disappear when ordinary
string contents are masked. Raw module values whose `_type` is `"override"`
are rejected as the representation produced by `mkOverride`, even when no
constructor identifier remains in source.
Repository-local modules may not contribute `disabledModules`, because any
such contribution can remove the required generator after the structural
import check. Every `imports` binding must be a direct literal list whose local
paths the scanner can resolve; aliases and other computed expressions fail
closed instead of creating an incomplete graph. Raw module `_type` fields are
also rejected regardless of their value spelling. This deliberately covers
the internal representations of priority constructors without attempting to
evaluate double-quoted, indented, or computed tag strings.
Lexical scope scanning skips path and URI tokens before interpreting `let` or
`in`, and recognizes an indented-string opener only at a token boundary. Valid
path components and apostrophes within identifiers therefore cannot create
phantom scopes or strings.
Path scanning treats `${...}` as an embedded expression, delegates its balanced
contents to the interpolation scanner, and then resumes the surrounding path.
Scope keywords in a suffix after interpolation therefore remain path data.
An unprefixed token containing a slash is also classified as a relative Nix
path when it begins at a token boundary, including when its first component
begins with punctuation accepted by the path-prefix grammar. For
indented-string control escapes,
`''\` and its following character are consumed together so an escaped dollar
cannot subsequently open interpolation.
Path scanning stops before `#`, allowing the existing line-comment scanner to
mask the remainder of the line before lexical scope processing.
URI recognition follows Nix's general `scheme:data` form and requires a
non-empty URI body; `//` is not required. This covers `mailto:` and similar
valid literals while retaining token-boundary checks.
The path classifier is on multiple character-scanning hot loops. Its common
non-path branch must use constant-time leading-character and token-boundary
checks, with prefix scanning only for plausible path/URI starts; this preserves
the lexical contract without paying regular-expression and generator overhead
for every source character.
Module binding, override, delimiter, and import analysis is also a pure
function of immutable source text and the importing directory. Those results
are memoized across warm validator samples; cached import sets are immutable so
callers cannot corrupt subsequent validation.

Two fixture mutations independently replace the mapped name with a constant
and the mapped value with an empty attribute set. Both must return a stable
validation diagnostic without a traceback. This deliberately constrains the
small generator to a reviewable canonical form instead of attempting to infer
semantic equivalence across arbitrary Nix expressions.

## Alternatives considered

- Evaluate the generator with Nix and inspect returned attribute names. This
  would strengthen semantic evidence but would make the offline Python
  contract depend on Nix evaluation, materially increasing its latency and
  complicating isolated mutation fixtures.
- Keep a permissive mapping-body wildcard and inspect only manifest policy.
  This is rejected because it permits the exact gate-collapse failure found in
  the post-merge review.

## Risks and mitigations

- Canonical-shape validation rejects semantically equivalent refactors. This
  is intentional fail-closed behavior; a reviewed validator/spec update can
  accompany a future generator refactor.
- Regex matching can accept source decoys when not anchored to the complete
  binding. A bounded lexical scanner identifies the matching outer `in` and
  top-level statement boundaries while ignoring strings and balanced nested
  delimiters; canonical regexes then match complete statements instead of the
  whole source.
- A future trusted root could become shadowable without being listed. The
  accepted mapping contract currently relies only on `builtins` and `pkgs`;
  changes that introduce another root must update this explicit fail-closed
  set and its review evidence.
- Static validation cannot prove arbitrary computed imports. The protected
  module surface therefore permits only direct literal import lists it can
  resolve, rejects `disabledModules`, and rejects raw module `_type` tags in
  the resulting reachable graph.

## Verification

- support-policy mutation suite, including both new exact mutations;
- canonical support-policy checker and benchmark diagnostics;
- Ruff lint and format;
- factory structural validation and receipt;
- full local and hosted Nix checks.
