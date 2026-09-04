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

The implementation at `795a5cb5557020adb9e25e302670941bca9d43bf`
meets that contract. Immediate assignments, attribute-path bindings, and
inherit statements are checked for trusted-root definitions. Comment scanning
and string scanning now share the same lexical precedence: strings are skipped
before comment recognition, and comment contents cannot open strings. A local
contradiction-focused pass found no remaining trusted-root or comment-order
bypass and no delivery blocker.

## Enclosing-scope and indented-string correction contract

The requested review on `0593cad` found two more reproducible boundaries. An
enclosing `let` around the returned module can shadow `builtins` before the
validated `perSystem` scope, collapsing the manifest to no gates. Separately,
the indented-string scanner treats escape prefixes including `''${`, `'''`,
and `''\` as closing delimiters, causing false rejection when later `#` data is
misclassified as a comment. The first finding is P1 and the second P2; both
block merge.

The validator SHALL require the canonical module lambda to return the module
attribute set directly, without an enclosing lexical wrapper before
`perSystem`. The indented-string scanner SHALL skip Nix escape prefixes before
accepting a closing `''`. An exact enclosing `builtins` mutation must fail and
valid escaped-delimiter strings containing comment markers must pass.

The implementation at `769b3dfdd27995c7847d0ded98a335d38b3d4ab7`
meets that contract. The accepted source prefix now proves the module lambda
returns the direct attribute set containing `perSystem`; an enclosing `let`
cannot satisfy it. Indented-string scanning skips dollar, quote, and backslash
escape prefixes before recognizing a close. The exact review mutations pass a
contradiction-focused local review with no remaining blocker.

## Quoted-root and interpolation correction contract

The automated review of `720064f` (reported against the no-content retry head
`ebf0897`) found two additional reproducible edges. A static quoted immediate
binding named `"builtins"` shadows the global root but bypasses the bare-name
matcher. A nested quoted string inside `${...}` is mistaken for the outer
closing quote, allowing a later `#` in the nested string to truncate valid
source. The findings are P1 and P2 respectively and block merge.

The validator SHALL reject quoted or dynamic first path components in the
immediate accepted `let`, since the canonical source needs neither. String
scanning SHALL balance interpolation braces and recursively consume nested Nix
strings and comments before returning to the outer string. Exact quoted-root
shadowing must fail; nested quoted and indented strings containing comment
markers inside interpolation must preserve canonical acceptance.

The implementation at `399a91f769bc2897611144c5497351afddd92b26`
meets that contract. Immediate quoted trusted roots are normalized into the
existing shadow check, every quoted or dynamic first binding component is
rejected fail-closed, and interpolation scanning balances braces while
recursively consuming nested strings and comments. Exact quoted-root and
dynamic-root mutations fail, both nested-string forms pass, and a final local
contradiction-focused review found no remaining bypass or delivery blocker.

## Formal-root and token-context correction contract

The exact-head review of `51f2106` found three reproducible lexical boundaries.
An injected `builtins` argument in the `perSystem` formals shadows the global
root before the validated `let`. A `let` or `in` path component is interpreted
as a scope keyword. Adjacent apostrophes within a valid identifier are
interpreted as an indented-string opener. The first finding is P1 and the other
two are P2; all block merge.

The validator SHALL reject `builtins` in the immediate `perSystem` formals and
accept only the canonical simple-formal shape. It SHALL skip path and URI
tokens before scope-keyword recognition and SHALL open an indented string only
at a valid token boundary. Exact injected-formal mutation must fail; unrelated
path/URI values containing `let` or `in` and identifiers containing adjacent
apostrophes must preserve canonical acceptance.

The implementation at `4b64ee37c7f6b88ef4b01feef27c62bac43a89f0`
meets that contract. The direct formal set is extracted from the same canonical
header and rejects `builtins` or non-simple entries. A bounded path/URI scanner
runs before string, comment, and scope recognition, while indented strings
require a non-identifier boundary. The exact three review mutations pass, Nix
itself parses all positive fixtures, and a final contradiction-focused local
review found no remaining token-context bypass or delivery blocker.

## Interpolated-path correction contract

The exact-head review of `463579b` found that a valid path token containing
`${...}` is stopped at the interpolation opening brace. A later `/let/` suffix
is then misread as a lexical scope keyword and causes a false rejection. This
P2 finding blocks merge.

The path scanner SHALL consume a balanced interpolation as part of the current
path and resume scanning its remaining components. An exact interpolated-path
fixture with a later `let` component must preserve canonical acceptance.
