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

## Verification

- support-policy mutation suite, including both new exact mutations;
- canonical support-policy checker and benchmark diagnostics;
- Ruff lint and format;
- factory structural validation and receipt;
- full local and hosted Nix checks.
