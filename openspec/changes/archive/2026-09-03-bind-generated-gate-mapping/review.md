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

The implementation at `0d2134c348b680d1e5245d1cc4d00decd13aa779`
meets that contract. Path scanning delegates `${...}` to the balanced
interpolation scanner and then resumes from its closing brace. The exact hosted
review fixture passes both the 62-test mutation suite and independent Nix parse
validation. A final contradiction-focused local review found no remaining
interpolated-suffix bypass or delivery blocker.

## Complete-relative-path and control-escape correction contract

The exact-head review of `21e3510` found two further valid token forms. Nix
accepts an unprefixed relative path such as `prefix/let/file`, but the scanner
does not classify it as a path and misreads `let`. Separately, `''\` escapes
the following character in an indented string, but the scanner consumes only
the prefix and can reinterpret an escaped dollar as `${...}`. Both P2 findings
block merge.

The validator SHALL recognize an unprefixed slash-bearing relative path at a
token boundary. Its indented-string scanner SHALL consume both the `''\`
prefix and the escaped character. Exact fixtures containing a `let` path
component and an escaped interpolation opener must preserve canonical
acceptance.

The implementation at `12a39bfd86fc44d135d1f5772ac9a9c4ba623063`
meets that contract. A slash-bearing token at a lexical boundary is recognized
as an unprefixed relative path, and the indented control-escape branch advances
past both its prefix and escaped character. Both exact hosted-review fixtures
pass in the 64-test mutation suite. A final contradiction-focused local review
found no remaining complete-path or escaped-interpolation blocker.

## Path-classifier performance correction contract

Hosted macOS evidence for `fa90102` measured warm p50 18.312 ms against a
17.866 ms material-regression ceiling. The full gate therefore failed even
though the richer 64-test lexical contract passed. This is a real non-functional
blocker; the concurrent security-agent installation warning is unrelated.

The classifier SHALL retain every accepted path/URI form while avoiding
regular-expression and iterator work on common non-path characters. Exact
mutation behavior must remain 64/64 and the hosted warm-latency gate must pass.

The implementation at `46443a63e5c4ddabfba5f93db23b45bdbcfbedbe`
meets the local correction contract. Path classification is memoized across
the repeated lexical passes, punctuation exits before prefix matching, and the
relative-prefix test no longer allocates a generator. All 64 mutations remain
green. Isolated warm p50 fell from 14.112 ms to 11.456 ms, restoring margin
below the hosted ceiling pending exact-head hosted confirmation.

## Punctuation-prefixed path correction contract

The exact-head review of `cbb39a7` found that Nix accepts an unprefixed
relative path such as `.let/file`, but the classifier limits unprefixed token
starts to alphanumeric characters and underscore. The later scope scan can
therefore misread `let` as a lexical keyword and reject a canonical generator.
This P2 finding blocks merge.

The validator SHALL treat every leading character accepted by
`NIX_UNPREFIXED_PATH_PREFIX` as a plausible unprefixed path start when it
appears at a token boundary. An exact `.let/file` fixture must preserve
canonical acceptance without weakening the existing path and URI cases.

The implementation at `0eeea86b75079c4b60e18bb10c3de7b44a7c6908`
meets that contract. The classifier's constant-time plausible-start check now
uses the same punctuation set as its compiled prefix grammar, and the exact
fixture passes both the 65-test mutation suite and independent Nix parse
validation. A final contradiction-focused local review found no remaining
punctuation-prefixed path blocker.

## Package-provider and path-comment correction contract

The exact-head review of `1372bb7` found that the root flake can supply a
wrapped `pkgs` value whose `lib.map` collapses generated gates while the
generator retains its canonical inheritance. It also found that a valid path
immediately followed by `# let` consumes the comment marker as path data and
later misreads `let` as a scope keyword. The provider finding is P1 and the
path-comment finding is P2; both block merge.

The validator SHALL require the package-set provider in the root flake's
canonical `perSystem` block to be the direct `import nixpkgs` expression with
the current system and Rust overlay. It SHALL terminate a path token before
`#` so the comment masker can consume the rest of the line. An exact
helper-override mutation must fail, while an exact path-followed-by-comment
fixture must preserve canonical acceptance.

The implementation at `19fe2f9c6b8f4feaf141859b3e3493e85dc968c9`
meets that contract. Provider validation anchors the canonical direct import
to the end of the root flake module, and path scanning yields before `#`. The
exact helper-override mutation fails and the independently Nix-parsed comment
fixture passes in the 67-test mutation suite. A final contradiction-focused
local review found no remaining provider or comment-boundary blocker.

## Root-import correction contract

The exact-head review of `72ed8c8` found that the provider's textual suffix can
remain canonical beneath an enclosing `let` that replaces the builtin
`import`. The replacement delegates to the real importer and then poisons
`pkgs.lib.map`, so the 23 manifest gates can still collapse while the validator
passes. This P1 finding blocks merge.

The validator SHALL require the flake to begin as a plain attribute set and
the canonical `outputs` lambda to return `flake-parts.lib.mkFlake` directly.
An exact enclosing-import mutation must fail before the package provider is
accepted.

The implementation at `73d2d6451647e4d5c645a610fe341f0fd997f98b`
meets that contract. Root validation rejects wrappers before the plain flake
attribute set and requires the only `outputs` binding to return `mkFlake`
directly from the canonical input formals. The exact shadowed-import mutation
fails in the 68-test suite. A final contradiction-focused local review found
no remaining lexical route to replace the provider's builtin imports.

## Check-wrapper composition correction contract

The exact-head review of `140513c` found that `nix/checks/default.nix` can keep
the generator import but replace its local checks definition with
`pkgs.lib.mkForce { }`. Flake-parts module merging then selects the forced empty
set and silently removes the 23 generated gates while validation passes. This
P1 finding blocks merge.

The validator SHALL require exactly one plain wrapper `checks` attribute-set
binding and reject module-priority constructors in the wrapper. An exact
forced-empty-checks mutation must fail before the import graph is accepted.

The implementation at `4601449dd43d6866adc28ba7235c728f8471cce6`
meets that contract. The wrapper must expose exactly one plain `checks`
attribute-set binding and cannot contain `mkForce` or `mkOverride`. The exact
forced checks mutation fails in the 69-test suite. A final
contradiction-focused local review found no remaining priority route that can
erase the imported generated checks.

## Module-graph and general-URI correction contract

The exact-head review of `6357cb1` found two additional boundaries. A sibling
module already imported by the root flake can use `mkForce` to erase generated
checks even though `nix/checks/default.nix` remains canonical. Separately, Nix
accepts URI literals such as `mailto:let@example.org`, but the scanner requires
`://` and misreads `let` as a lexical scope. The module finding is P1 and the
URI finding is P2; both block merge.

The validator SHALL recursively inspect every repository-local module
reachable through the root import graph and reject module-priority
constructors. URI scanning SHALL recognize a token-boundary `scheme:` prefix
followed by a non-empty Nix URI body without requiring `//`. An exact sibling
`mkForce` mutation must fail, while an exact `mailto:` fixture must preserve
canonical acceptance.

The implementation at `2aaece7feae5051d872eb979000b9b0b2c40eef4`
meets the functional contract. The local import graph is traversed recursively,
priority constructors are rejected in every reachable repository module, and
URI classification accepts the general non-empty `scheme:data` grammar. The
exact sibling override fails and the independently Nix-parsed `mailto:` fixture
passes in the 71-test suite. The initial graph traversal raised warm p50 above
the material-regression ceiling; `11f9db0a374108fb4f9ec45c6632ef1333d53830`
caches pure source masking and restores warm p50 to 7.741 ms. A final
contradiction-focused local review found no remaining sibling-override,
general-URI, or latency blocker.

## Effective module-override correction contract

The exact-head review of `7b4fffc` found three further ways to erase generated
checks. A quoted attribute selection such as `pkgs.lib."mkForce"` disappears
under ordinary string masking; a `../override.nix` import is misread from its
second dot as a child-relative path; and the raw module value
`{ _type = "override"; priority = 0; content = { }; }` has `mkOverride`
semantics without retaining the constructor name. All three findings are P1
and block merge.

The validator SHALL detect bare and quoted priority constructors plus raw
module override records in every reachable repository-local module. Literal
graph traversal SHALL preserve and resolve complete `./` and `../` import
prefixes. Exact mutations for all three hosted findings must fail with the
stable module-graph diagnostic before integration.

The implementation at `49b7ca00d2b951293f50ec79249d3f006e27ec5b`
meets that contract. A shared lexical predicate recognizes bare, statically
quoted, and directly dynamic priority-constructor selections plus raw
`_type = "override"` values without treating string data as executable source.
Local import extraction now uses the existing Nix path scanner and preserves
complete child- and parent-relative tokens before resolution. The three exact
hosted mutations fail closed, an additional dynamic-selection mutation fails,
and a string-data control remains accepted in the 76-test suite. All fixtures
parse with Nix, and a final contradiction-focused local review found no
remaining blocker within the defined effective-override contract.

## Effective module-graph correction contract

The exact-head review of `c5b518f` found three additional graph-composition
bypasses. A sibling can contribute `disabledModules` for the required
`rust-gates.nix`; a reachable module can hide a local override module behind a
computed `imports` binding; and the raw override tag can use an indented string
as `_type = ''override''`. Each mutation leaves the structural validator green
while erasing generated checks, so all three P1 findings block merge.

The validator SHALL reject repository-local `disabledModules` contributions,
require every reachable `imports` binding to be a directly traversable literal
list, and reject raw module `_type` tags independent of value spelling. Exact
mutations for all three hosted findings must fail closed before integration.

The implementation at `6c4ba739d87c0b09a75a190babc75f2966b3b23b`
meets that contract. Reachable local modules reject any static
`disabledModules` binding and any raw `_type` field before value spelling is
considered. Import scanning balances a direct list, requires its terminating
semicolon, resolves every local path token, and fails closed if executable
residue or an appended expression remains. The three exact hosted mutations
and an additional computed-list-entry mutation fail in the 80-test suite. All
three hosted fixtures parse with Nix, and a final contradiction-focused local
review found no remaining blocker within the defined static module-graph
contract.

## Module-analysis performance correction contract

Hosted macOS CI on `890afed` passed the complete functional validator but
measured warm p50 at 24.373 ms against a 23.806 ms material-regression ceiling.
The repeated binding, override, delimiter, and import scans are pure over
immutable source inputs and currently rerun across every warm sample.

The implementation SHALL memoize those pure results, expose cached import
paths immutably, preserve all 80 functional mutations, and pass the unchanged
hosted material-regression guard. Threshold relaxation is not an acceptable
fix.

The implementation at `f5c1f0d9aaff3c84265413d36cded26b3406f80f`
meets that contract. The four pure module-source analyses are cached and local
import results are returned as a `frozenset`. All 80 mutations remain green.
The exact local 20-sample rerun reduced warm p50 from 15.128 ms to 7.998 ms,
with no material regression against `develop`; the unchanged hosted threshold
remains the integration gate.

## Static module-boundary correction contract

A delayed review thread on `890afed` and exact-head review of `c9d4166` found
three further graph spellings. Static `"imports" = [...]` is hidden by string
masking, `pkgs.lib.${"mk" + "Force"}` computes a priority constructor, and
`./${"override"}.nix` is resolved as an unevaluated source spelling and then
silently skipped. Each permits a reachable sibling to erase generated checks,
so all three P1 findings block merge.

The validator SHALL normalize bare and statically quoted attribute assignments
for graph discovery, reject import paths containing interpolation, and reject
computed attribute selections outside the canonical generator. It SHALL also
reject any competing static `checks` binding outside the two canonical check
modules. Exact mutations for all three hosted findings must fail while the
generator's independently validated dynamic operation lookup remains accepted.

The implementation at `fc51d79049d5b77e9385578ab58a018771cb9b25`
meets that contract. Central assignment discovery returns the value offset for
bare, quoted, and directly dynamic names. Import traversal rejects interpolated
paths and any executable list residue. Outside `rust-gates.nix`, executable
`${...}` attributes are rejected; outside the two canonical gate modules,
static or inherited `checks` contributions are rejected. The three hosted
mutations plus competing static/inherited checks regressions pass in the
85-test suite. All hosted fixtures parse with Nix, and a final
contradiction-focused local review found no remaining static or computed route
within the defined local module boundary.

## Closed module-authoring boundary correction contract

The settled-head review of `3c9c291` and two delayed threads from `c9d4166`
identified five remaining bypasses: unresolved computed imports in the root
list, computed raw `_type` names, inherited `imports`, priority helpers reached
through `getAttr`, and protected keys synthesized through `listToAttrs`. Each
can leave source-level discovery incomplete while Nix erases the manifest-
generated checks, so all five P1 findings block merge.

The validator SHALL preserve unresolved root-import evidence while explicitly
allowing static external `inputs.<name>.flakeModule` entries. It SHALL reject
inherited imports, inherited protected module keys, attribute reflection, and
dynamic attribute-set construction outside the canonical generator. Exact
mutations for every hosted finding must fail before implementation evidence is
accepted.

The implementation at `9093a493de364e96c1405825d9dbf9b64ce225c5`
meets that contract. Root traversal retains unresolved residue after removing
only static external flake-module selections. Shared cached predicates reject
inherited import edges, reflective attribute lookup, computed keys, inherited
protected keys, and runtime attribute-set constructors everywhere except the
independently constrained generator. All five hosted fixtures plus inherited
`disabledModules` and serialized-construction controls pass in the 92-test
suite. A contradiction-focused local review found no unresolved issue within
the defined closed boundary.

## Module-result isolation correction contract

A delayed review thread from `3c9c291` demonstrated that a nested inert
`imports = [ ./rust-gates.nix ];` value can satisfy global source discovery
after the effective top-level import is removed. The validator then accepts a
generator edge that flake-parts never evaluates. This P1 finding blocks merge.

The validator SHALL isolate the direct returned module attribute set and
discover imports only from its immediate bindings. The same structural import
result SHALL prove the wrapper's required generator edge. An exact nested-decoy
mutation must fail closed before implementation evidence is accepted.
