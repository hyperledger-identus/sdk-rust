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

The narrow wiring validator will recognize the canonical mapping expression as
one structural unit: `map (gate: { inherit (gate) name; value = makeGate gate;
}) manifest.gates`, passed directly to `listToAttrs` and bound to
`generatedChecks`. It will also resolve the outer `perSystem` let binding and
require both mapping helpers in its immediate `inherit (pkgs.lib)` binding, so
the canonical identifiers cannot refer to local replacements. The existing
returned-module check continues to require `checks = generatedChecks` at the
generator return tail.

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
  binding. The expression requires the mapped parameter, name, value,
  `manifest.gates` input, `listToAttrs` output, and terminating binding in one
  match, while publication remains anchored to the returned module tail.

## Verification

- support-policy mutation suite, including both new exact mutations;
- canonical support-policy checker and benchmark diagnostics;
- Ruff lint and format;
- factory structural validation and receipt;
- full local and hosted Nix checks.
