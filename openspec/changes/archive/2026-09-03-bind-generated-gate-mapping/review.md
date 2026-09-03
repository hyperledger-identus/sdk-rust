# Pre-implementation semantic review

- **Date:** 2026-09-04
- **Issue:** #61, follow-up to #24 / PR #60
- **Develop base:** `69d38874e3d3f669d60e129ed6882e835aedafe6`
- **Result:** contract is complete and has no unresolved blocker

## Findings

The late review evidence is reproducible from the generator and current
validator: the wildcard mapping body proves only that some mapping consumes
`manifest.gates`, not that the published names or derivations preserve each
entry. The failure can silently reduce the required check graph, so P1 is the
correct severity and an immediate repository-local follow-up is warranted.

The bounded canonical-shape contract is proportional to this generator. It
keeps the validator offline and fast, requires the name and value expressions
in the same mapping body, and composes with the already-anchored returned
module check. Independent name and value mutations provide clearer evidence
than one compound fixture.

No Nix output, manifest data, public API, supported surface, dependency cone,
security boundary, release policy, donor provenance, or consumer repository
changes. No ADR or specialist review is required because the implementation
only makes an already-accepted execution contract fail closed.

## Implementation review

- **Reviewed implementation head:**
  `20f87a0223ce7cd2d439e1d8904bf1fc0b6204aa`
- **Reviewer:** distinct contradiction-focused local pass
- **Result:** one adjacent lexical-shadow finding resolved; no remaining
  architecture, compatibility, security, provenance, or delivery blocker

The implemented mapping pattern requires the canonical gate parameter,
inherited name, `makeGate gate` value, `manifest.gates` input, and
`listToAttrs` result in one expression. The exact late-review constant-name
mutation and an independent detached-value mutation fail with the intended
diagnostic.

The local review found that separately valid mapping and publication fragments
could still be separated by a nested `let` that shadows `generatedChecks`.
The validator now additionally requires the complete mapping binding and
returned top-level publication as one uninterrupted generator tail. A third
mutation proves the shadowed empty set cannot replace the mapped result.

The change remains validation-only. The Nix generator, declarative manifest,
derivations, support claims, Rust workspace, and consumers are byte-identical
to the merged base.

## Pull-request review correction

The hosted review found that canonical `map` and `listToAttrs` call spellings
could still resolve to local replacements after removing those names from the
`pkgs.lib` inheritance. The finding was confirmed. The validator now resolves
the immediate outer `perSystem` let inheritance and requires both helpers from
`pkgs.lib` before accepting the mapping expression.

Two focused mutations replace `map` with a constant single-entry function and
replace `listToAttrs` with a discarding function. Both fail deterministically;
the canonical generator continues to pass. The correction changes no Nix
generator expression or derived check.

## Final hosted-review correction contract

A second hosted review on `fa9e6a5` found that the independent outer-scope
searches and the concatenated mapping/publication regex can be satisfied by
different lexical scopes. An outer canonical helper inheritance and manifest
binding can therefore act as decoys while a returned inner `let` redefines all
three mapping inputs and publishes a collapsed check graph. The finding is
confirmed and remains a merge blocker.

The correction SHALL derive the matching immediate outer `perSystem` `let`
body, accept helper inheritance, manifest parsing, and `generatedChecks` only
as top-level statements in that body, and validate its matching `in` result.
An exact nested-shadow mutation must fail. This is still a bounded source-shape
contract, not a general Nix evaluator, and changes no generated derivation.

The implementation at `98983bc9e96974db4e6e3800b665141e2a38f2b9`
meets that contract. The scanner masks Nix strings, balances nested delimiters
and `let`/`in` pairs, and splits only immediate-scope statements. The exact
hosted-review mutation now fails, the canonical generator passes, and a local
contradiction-focused review found no remaining cross-scope acceptance path or
delivery blocker.

## Trusted-root and lexical-comment correction contract

The final hosted review on `09eb627` produced two reproducible findings. First,
an immediate recursive binding can redefine `builtins` or `pkgs`, allowing the
canonical manifest/helper expressions to resolve through attacker-controlled
roots. Second, comment removal runs before string masking, so `#` and block
comment delimiters inside valid strings can truncate otherwise valid source.
The trusted-root finding is P1; the false rejection is P2. Both block merge.

The validator SHALL reject immediate outer bindings named `builtins` or `pkgs`.
It SHALL identify string spans before removing comments and remove only comment
syntax outside those spans. Exact `builtins` and `pkgs` shadows must fail, while
unrelated double-quoted and indented strings containing line/block comment
markers must preserve canonical acceptance.
