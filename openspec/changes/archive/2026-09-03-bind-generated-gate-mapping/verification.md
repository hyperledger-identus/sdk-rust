# Verification evidence

- **Issue:** #61, follow-up to #24 / PR #60
- **Develop base:** `69d38874e3d3f669d60e129ed6882e835aedafe6`
- **Reviewed implementation head:**
  `20f87a0223ce7cd2d439e1d8904bf1fc0b6204aa`
- **Local platform:** macOS 26.2, aarch64-darwin, Python 3.14.3
- **Result:** every applicable local gate passed

## Contract and mutation evidence

- `python3 scripts/tests/support-policy.py` passed 49/49 tests. Six focused
  mutations cover constant-name gate collapse, a value detached from
  `makeGate gate`, a nested `generatedChecks` shadow between mapping and
  returned publication, local replacements for `map` and `listToAttrs`, and a
  returned inner scope that shadows every mapping input behind valid outer
  decoys.
- `./scripts/check-support-policy.py` passed the canonical 23-gate manifest and
  generator.
- Ruff lint and format checks passed for the validator and mutation suite.
- `python3 scripts/benchmark-support-policy.py --samples 20` passed after the
  final correction with warm p50/p95 of 7.832/8.352 ms and process-cold
  p50/p95 of 47.852/48.469 ms.
- `./scripts/factory check` passed all 18 active-change/canonical items.
- `nix flake check --print-build-logs` passed every compatible
  aarch64-darwin check; x86_64-linux execution is intentionally supplied by
  hosted CI.
- `./scripts/factory receipt bind-generated-gate-mapping` passed on branch
  `codex/issue-61-bind-gate-mapping`, reviewed head `20f87a0`, and exact
  develop merge base `69d3887` before the capability was synced and archived.

## Review conclusion

The late P1 is confirmed and fixed. The implementation also closes the
adjacent lexical-shadow route found during the distinct local review. The
accepted source shape is intentionally fail-closed and may reject a future
semantically equivalent Nix refactor until its contract changes with review.
No unresolved blocker remains.

Hosted review's mapping-helper finding is resolved by proving both identifiers
come from the immediate `inherit (pkgs.lib)` binding. The exact local-`map`
collapse and symmetric local-`listToAttrs` mutations fail closed.

The second hosted-review finding is resolved at implementation head
`98983bc9e96974db4e6e3800b665141e2a38f2b9`: helper inheritance, manifest
parsing, generated mapping, and publication are now accepted only from the
same immediate outer `perSystem` scope. The exact all-input inner-shadow
mutation fails. Ruff, the 17-item factory contract, the performance guard, and
the complete compatible aarch64-darwin Nix graph all pass after the correction.

## Compatibility and isolation

- No Nix generator, manifest, derivation, Rust API, wire behavior, dependency,
  support surface, release policy, or security boundary changed.
- No donor source or fixture was used.
- No downstream repository was read or modified for this follow-up.
- Reserved `main` remains untouched at
  `2c267d65af5c6b6dc9c8fd6826266c8ad0c3256a`.

## Final trusted-root and lexical evidence

The trusted-root and lexical-comment correction is implemented at
`795a5cb5557020adb9e25e302670941bca9d43bf`. The mutation suite passes 53/53:
exact `builtins` and `pkgs` shadows fail, comment delimiters in double-quoted
and indented strings pass, and unmatched string delimiters inside line/block
comments cannot mask live source. Ruff, canonical policy, the 17-item factory
contract, and the complete compatible aarch64-darwin Nix graph pass. The final
benchmark reports warm p50/p95 9.338/9.706 ms and process-cold p50/p95
48.457/50.035 ms, within the diagnostic guard.

## Final enclosing-scope and indented-string evidence

The enclosing-scope and indented-string correction is implemented at
`769b3dfdd27995c7847d0ded98a335d38b3d4ab7`. The mutation suite passes 55/55:
an enclosing `builtins` shadow fails, while `''${`, `'''`, and `''\` escape
prefixes inside valid indented strings preserve later comment-marker data.
Ruff, canonical policy, the 17-item factory contract, and the complete
compatible aarch64-darwin Nix graph pass. The final benchmark reports warm
p50/p95 9.577/10.952 ms and process-cold p50/p95 47.908/49.668 ms, within the
diagnostic guard.

## Final quoted-root and interpolation evidence

The quoted-root and interpolation correction is implemented at
`399a91f769bc2897611144c5497351afddd92b26`. The mutation suite passes 58/58:
exact quoted and dynamic binding roots fail closed, while nested double-quoted
and indented strings inside interpolation preserve comment-marker data. Ruff
lint and format checks, canonical policy validation, the 17-item factory
contract, and the complete compatible aarch64-darwin Nix graph pass. The final
isolated benchmark reports warm p50/p95 9.202/10.003 ms and process-cold
p50/p95 48.118/51.430 ms, within the diagnostic guard.

## Final formal-root and token-context evidence

The formal-root and token-context correction is implemented at
`4b64ee37c7f6b88ef4b01feef27c62bac43a89f0`. The mutation suite passes 61/61:
an injected `builtins` formal fails closed, while path/URI values containing
scope keywords and an identifier containing adjacent apostrophes remain valid.
`nix-instantiate --parse` independently accepts all three positive token forms.
Ruff lint and format checks, canonical policy validation, the 17-item factory
contract, and the complete compatible aarch64-darwin Nix graph pass. The final
isolated benchmark reports warm p50/p95 11.329/12.116 ms and process-cold
p50/p95 51.667/53.976 ms, within the diagnostic guard.

## Final interpolated-path evidence

The interpolated-path correction is implemented at
`0d2134c348b680d1e5245d1cc4d00decd13aa779`. The mutation suite passes 62/62,
including `./prefix/${"x"}/let/file`; `nix-instantiate --parse` independently
accepts that fixture. Ruff lint and format checks, canonical policy validation,
the 17-item factory contract, and the complete compatible aarch64-darwin Nix
graph pass. The final isolated benchmark reports warm p50/p95 12.083/12.789 ms
and process-cold p50/p95 51.310/53.004 ms, within the diagnostic guard.

## Final complete-path and control-escape evidence

The complete-relative-path and control-escape correction is implemented at
`12a39bfd86fc44d135d1f5772ac9a9c4ba623063`. The mutation suite passes 64/64:
`prefix/let/file` remains a single path token and `''\$` remains indented-string
data rather than opening interpolation. Ruff lint and format checks, canonical
policy validation, the 17-item factory contract, and the complete compatible
aarch64-darwin Nix graph pass. The final isolated benchmark reports warm
p50/p95 14.112/15.076 ms and process-cold p50/p95 54.056/57.701 ms, within the
diagnostic guard.

## Final path-classifier performance evidence

The hot-path correction is implemented at
`46443a63e5c4ddabfba5f93db23b45bdbcfbedbe`. The 64/64 mutation suite, Ruff,
canonical policy validation, and factory contract remain green. Isolated
performance after the correction is warm p50/p95 11.456/13.591 ms and
process-cold p50/p95 52.846/55.720 ms. Compared with the preceding local
evidence, warm p50 improved by 18.8% while retaining every lexical case.

## Final punctuation-prefixed path evidence

The punctuation-prefixed path correction is implemented at
`0eeea86b75079c4b60e18bb10c3de7b44a7c6908`. The mutation suite passes 65/65,
including the independently Nix-parsed `.let/file` fixture. Ruff lint and
format checks, canonical policy validation, the factory contract, and the
complete compatible aarch64-darwin Nix graph pass. The isolated benchmark
reports warm p50/p95 11.338/11.721 ms and process-cold p50/p95 52.676/54.017
ms, retaining margin below the hosted material-regression ceiling.

## Final provider and path-comment evidence

The package-provider and path-comment corrections are implemented at
`19fe2f9c6b8f4feaf141859b3e3493e85dc968c9`. The mutation suite passes 67/67:
the exact `pkgs.lib.map` override fails closed, while a path immediately
followed by `# let` remains accepted and is independently Nix-parsed. Ruff,
canonical policy validation, the factory contract, and the complete compatible
aarch64-darwin Nix graph pass. The isolated benchmark reports warm p50/p95
11.242/12.091 ms and process-cold p50/p95 54.271/56.245 ms, retaining margin
below the hosted material-regression ceiling.

## Final root-import evidence

The root-import correction is implemented at
`73d2d6451647e4d5c645a610fe341f0fd997f98b`. The mutation suite passes 68/68,
including the exact enclosing `import` replacement that poisons `pkgs.lib.map`.
Ruff, canonical policy validation, the factory contract, and the complete
compatible aarch64-darwin Nix graph pass. The isolated benchmark reports warm
p50/p95 11.306/11.894 ms and process-cold p50/p95 55.216/57.497 ms, retaining
margin below the hosted material-regression ceiling.

## Final check-wrapper composition evidence

The wrapper-composition correction is implemented at
`4601449dd43d6866adc28ba7235c728f8471cce6`. The mutation suite passes 69/69,
including the exact `pkgs.lib.mkForce` override that previously erased imported
gates. Ruff, canonical policy validation, the factory contract, and the
complete compatible aarch64-darwin Nix graph pass. The isolated benchmark
reports warm p50/p95 11.927/12.579 ms and process-cold p50/p95 55.173/56.551
ms, retaining margin below the hosted material-regression ceiling.

## Final module-graph and general-URI evidence

The module-graph and general-URI corrections are implemented at
`2aaece7feae5051d872eb979000b9b0b2c40eef4`, with source-mask caching at
`11f9db0a374108fb4f9ec45c6632ef1333d53830`. The mutation suite passes 71/71:
the exact sibling `mkForce` override fails closed, while
`mailto:let@example.org` remains accepted and is independently Nix-parsed.
Ruff, canonical policy validation, the factory contract, and the complete
compatible aarch64-darwin Nix graph pass. The isolated benchmark reports warm
p50/p95 7.741/8.284 ms and process-cold p50/p95 58.176/60.540 ms, retaining
margin below the hosted material-regression ceiling.

## Final effective module-override evidence

The effective-override correction is implemented at
`49b7ca00d2b951293f50ec79249d3f006e27ec5b`. The mutation suite passes 76/76:
the exact quoted-constructor, parent-relative-import, and raw-override hosted
findings fail closed; a directly dynamic constructor selection also fails;
and equivalent text inside indented-string data remains accepted. The three
hosted fixtures parse independently with Nix. Ruff lint and format, canonical
policy validation, the 17-item factory contract, and the complete compatible
aarch64-darwin Nix graph pass. Against `develop` at
`69d38874e3d3f669d60e129ed6882e835aedafe6`, the 20-sample benchmark reports
warm p50/p95 10.527/11.011 ms and process-cold p50/p95 60.259/62.391 ms, with
no material regression.

## Final effective module-graph evidence

The module-graph escape-hatch correction is implemented at
`6c4ba739d87c0b09a75a190babc75f2966b3b23b`. The mutation suite passes 80/80:
the exact `disabledModules`, computed-import, and indented raw-tag findings
fail closed, as does a computed entry inside an otherwise literal imports
list. The three hosted fixtures parse independently with Nix. Ruff lint and
format, canonical policy validation, the 17-item factory contract, and the
complete compatible aarch64-darwin Nix graph pass. Against `develop` at
`69d38874e3d3f669d60e129ed6882e835aedafe6`, the 20-sample benchmark reports
warm p50/p95 15.128/16.124 ms and process-cold p50/p95 66.689/69.369 ms, with
no material regression under the repository guard.

## Final module-analysis performance evidence

Pure module-source analysis is cached at
`f5c1f0d9aaff3c84265413d36cded26b3406f80f`, with cached import paths exposed
immutably. The 80/80 mutation suite, Ruff, canonical policy validation, the
17-item factory contract, and the complete compatible aarch64-darwin Nix graph
remain green. Against `develop` at
`69d38874e3d3f669d60e129ed6882e835aedafe6`, the exact 20-sample rerun reports
warm p50/p95 7.998/8.851 ms and process-cold p50/p95 65.817/68.682 ms, with no
material regression and no threshold change.

## Final static module-boundary evidence

The static module-boundary correction is implemented at
`fc51d79049d5b77e9385578ab58a018771cb9b25`. The mutation suite passes 85/85:
the quoted-import, computed-priority, and interpolated-import hosted findings
fail closed, as do competing static and inherited checks contributions; the
canonical generator's dynamic attribute operations remain accepted. All three
hosted fixtures parse independently with Nix. Ruff lint and format, canonical
policy validation, the 17-item factory contract, and the complete compatible
aarch64-darwin Nix graph pass. Against `develop` at
`69d38874e3d3f669d60e129ed6882e835aedafe6`, the 20-sample benchmark reports
warm p50/p95 8.518/8.991 ms and process-cold p50/p95 73.274/75.066 ms, with no
material regression.

## Final closed module-authoring boundary evidence

The closed-boundary correction is implemented at
`9093a493de364e96c1405825d9dbf9b64ce225c5`. The mutation suite passes 92/92:
all five hosted bypasses fail closed, as do inherited `disabledModules` and
serialized dynamic construction controls. Nix independently parses every
hosted fixture. Ruff lint and format, canonical policy validation, all 17
factory contracts, and the complete compatible aarch64-darwin Nix graph pass.
Against `develop` at
`69d38874e3d3f669d60e129ed6882e835aedafe6`, the exact 20-sample benchmark
reports warm p50/p95 8.407/9.276 ms and process-cold p50/p95 71.298/74.014 ms,
with no material regression.

## Final module-result isolation evidence

Module-result isolation is implemented at
`b2b48f906ef67a18c857bb3996bc3754b24c83be`. The mutation suite passes 94/94:
the exact nested import decoy fails and a positive control confirms inert
import-shaped data does not become a graph edge. Ruff lint and format,
canonical policy validation, all 17 factory contracts, and the complete
compatible aarch64-darwin Nix graph pass. Against `develop` at
`69d38874e3d3f669d60e129ed6882e835aedafe6`, the exact 20-sample benchmark
reports warm p50/p95 9.101/27.849 ms and process-cold p50/p95 85.482/86.727 ms,
with no material regression; p95 remains diagnostic by contract.

## Final imported-config and inherited-priority evidence

The imported-config and inherited-priority correction is implemented at
`879425c8c683c676d9e2885c2cfdf4aefc742e57`. The mutation suite passes 100/100:
both hosted exploits fail, direct and quoted VM-override controls fail, explicit
config without import fails, and import-like string data remains accepted. Nix
independently parses both hosted fixture shapes. Ruff lint and format,
canonical policy validation, all 17 factory contracts, and the complete
compatible aarch64-darwin Nix graph pass. Against `develop` at
`69d38874e3d3f669d60e129ed6882e835aedafe6`, the exact 20-sample benchmark
reports warm p50/p95 9.282/21.365 ms and process-cold p50/p95 85.307/87.589 ms,
with no material regression; p95 remains diagnostic by contract.

## Final canonical root-import and static-name evidence

Canonical root anchoring and static computed-name traversal are implemented at
`fb7e5ffb05b77fb60ed9aa80fd5c0a6ed25b07a0`. The mutation suite passes 103/103:
the effective-root/inert-`mkFlake` decoy is rejected, and both `${"imports"}`
and `${''imports''}` bindings are traversed to their priority-override target.
All three fixtures parse independently with Nix 2.34.6; direct
`''imports'' = [...]` is intentionally not a fixture because that spelling is
invalid Nix. Ruff lint and format, canonical policy validation, all 17 factory
contracts, and the complete compatible aarch64-darwin Nix graph pass. Against
`develop` at `69d38874e3d3f669d60e129ed6882e835aedafe6`, the exact 20-sample
benchmark reports warm p50/p95 9.139/10.147 ms and process-cold p50/p95
83.521/87.827 ms, with no material regression.

## Final generator-dispatch binding evidence

Manifest-selected gate construction is bound at
`46d966a03f07eacc13cee69a652406c0a2f39f67`. The mutation suite passes 107/107:
the hosted no-op `makeGate` replacement fails, as do detached `cargoArgs`, an
altered operation-to-argument mapping, and a shadowed Crane provider. All four
fixtures parse independently with Nix 2.34.6. Ruff lint and format, canonical
policy validation, all 17 factory contracts, and the complete compatible
aarch64-darwin Nix graph pass. Against `develop` at
`69d38874e3d3f669d60e129ed6882e835aedafe6`, the exact 20-sample benchmark
reports warm p50/p95 8.911/9.509 ms and process-cold p50/p95 88.359/90.275 ms,
with no material regression.

## Final hosted generator-normalization performance evidence

Direct canonical-source comparison is implemented at
`5f7b65120454a519e01aa23782e6c78b100f9354`. All 107 mutations, Ruff, 17
factory contracts, and the local compatible aarch64-darwin Nix graph remain
green. Local comparison against `develop` reports warm p50/p95 8.879/9.368 ms
and process-cold p50/p95 82.743/91.740 ms with no material regression. Hosted
Ubuntu run `33851811403`, job `100956092337`, reports warm p50/p95
12.654/21.236 ms and process-cold p50/p95 117.731/119.596 ms against baseline
57.412/58.009 ms; the unchanged material-regression gate passes.

## Final provider-provenance and static-selector evidence

Trusted input, root-provider, Crane-provider, quoted-selector, and static-name
hardening is implemented at
`de59d4edf8019b17d4e370e4b3ec7731b2e55310`. The mutation suite passes 116/116:
all seven hosted findings fail, as do a local Crane input replacement and an
immediate `inputs` shadow in the toolchain scope. Every exact hosted fixture
parses independently with Nix 2.34.6. Ruff lint/format, canonical policy
validation, all 17 factory contracts, and the compatible aarch64-darwin Nix
graph pass. Against `develop` at
`69d38874e3d3f669d60e129ed6882e835aedafe6`, the exact 20-sample local benchmark
reports warm p50/p95 9.000/9.475 ms and process-cold p50/p95 87.371/89.488 ms;
the unchanged material-regression gate passes.

## Final provider-result boundary evidence

Complete provider-result consumption is enforced at
`020be7dc69dddda8863a1149ae1327974899de07`. The mutation suite passes 117/117:
the exact Nix-valid trailing merge that replaces the effective `craneLib`
fails, while the canonical provider result remains accepted. Ruff lint and
format, canonical policy validation, the full factory-contract suite, and the
compatible aarch64-darwin Nix graph pass. Against PR base
`0ed7fe42519df5d6778444fe31f3769ad05fc70c`, the exact 20-sample benchmark
reports warm p50/p95 9.200/10.197 ms and process-cold p50/p95 91.650/93.936 ms;
the unchanged material-regression gate passes.

## Final locked-provenance and indented-name evidence

Locked source provenance and indented interpolated names are enforced at
`6a5b13ea10f3eae8f137c6dd7892d3fcb967b265`. The mutation suite passes 120/120:
a changed Crane lock owner, a root mapping redirected to devshell, and an
indented interpolated `inputs` shadow all fail; the Nix-valid shadow fixture is
independently parsed. Ruff lint and format, canonical policy validation, the
full factory-contract suite, and the compatible aarch64-darwin Nix graph pass.
Against PR base `0ed7fe42519df5d6778444fe31f3769ad05fc70c`, the exact 20-sample
benchmark reports warm p50/p95 9.661/11.771 ms and process-cold p50/p95
94.573/97.972 ms; the unchanged material-regression gate passes.

## Final root-output boundary evidence

Complete root-output consumption is enforced at
`7721da562937a186088d49bbbb1336f82db90b74`. The mutation suite passes 121/121:
the exact Nix-valid trailing import merge that erases generated checks fails,
while the canonical `mkFlake` output remains accepted. Ruff lint and format,
canonical policy validation, the full factory-contract suite, and the
compatible aarch64-darwin Nix graph pass. Against PR base
`0ed7fe42519df5d6778444fe31f3769ad05fc70c`, the exact 20-sample benchmark
reports warm p50/p95 9.388/10.384 ms and process-cold p50/p95 91.578/94.764 ms;
the unchanged material-regression gate passes.

## Final indented-name and transitive-lock evidence

Indented protected-name normalization and complete provider lock topology are
enforced at `08c808cf7836d7f7873a7e1d27e50d876669cf80`. The mutation suite passes
124/124: the exact Nix-valid layout-whitespace `inputs` shadow, transitive edge
redirection, and transitive target-owner replacement all fail, while canonical
source remains accepted. Ruff lint and format, canonical policy validation,
the full factory-contract suite, and the compatible aarch64-darwin Nix graph
pass. Against PR base `0ed7fe42519df5d6778444fe31f3769ad05fc70c`, the exact
20-sample benchmark reports warm p50/p95 9.085/9.582 ms and process-cold p50/p95
87.937/93.288 ms; the unchanged material-regression gate passes.

## Final closed provider-result and quoted-name evidence

The closed provider result and ambiguous quoted-name rejection are implemented
at `63f1e97dfc37062c1bd081c7c590c092bb7f4d72`. The mutation suite passes
126/126: a nested per-system import and quoted constant-concatenation
`disabledModules` binding fail, while unrelated priority-looking string data
remains accepted outside the provider result. Ruff lint and format, canonical
policy validation, the full factory-contract suite, and the compatible
aarch64-darwin Nix graph pass. Against PR base
`0ed7fe42519df5d6778444fe31f3769ad05fc70c`, the exact 20-sample benchmark
reports warm p50/p95 9.719/10.079 ms and process-cold p50/p95 92.157/94.625 ms;
the unchanged material-regression gate passes.

## Final deferred-import and normalized-priority evidence

Deferred `perSystem` import traversal and normalized priority selections are
implemented at `6da35d5b09914d9cc9a41b9ae34b4e19cbc0d82f`. The mutation
suite passes 132/132: indented and quoted-static-interpolated `mkForce`
selectors fail; imports from direct and `let`-wrapped deferred modules are
traversed; unresolved deferred imports fail closed; and nested import-shaped
data remains inert. Ruff lint and format, canonical policy validation, the
complete factory contract, and the compatible aarch64-darwin Nix graph pass.
Against PR base `0ed7fe42519df5d6778444fe31f3769ad05fc70c`, the exact
20-sample benchmark reports warm p50/p95 9.231/9.697 ms and process-cold
p50/p95 107.902/110.647 ms; the unchanged material-regression gate passes.

## Final indented control-escape evidence

Indented control escapes in executable attribute positions are rejected at
`bf99ffaeb23b2cf50a99b4f6d257741a4bd2abd4`. The mutation suite passes
134/134: exact protected-root and priority-selector escapes fail, while the
same syntax remains accepted in ordinary string data. Ruff lint and format,
canonical policy validation, the complete factory contract, and the compatible
aarch64-darwin Nix graph pass. Against PR base
`0ed7fe42519df5d6778444fe31f3769ad05fc70c`, the exact 20-sample
benchmark reports warm p50/p95 9.229/9.464 ms and process-cold p50/p95
106.358/109.089 ms; the unchanged material-regression gate passes.

## Final root explicit-config evidence

Root-module explicit config rejection is implemented at
`de7da620e6094fdc712c02e286f312952efd18f9`. The mutation suite passes
137/137: the exact imported config override plus inherited and quoted config
forms fail before provider acceptance. Ruff lint and format, canonical policy
validation, the complete factory contract, and the compatible aarch64-darwin
Nix graph pass. Against PR base
`0ed7fe42519df5d6778444fe31f3769ad05fc70c`, the exact 20-sample
benchmark reports warm p50/p95 9.269/9.553 ms and process-cold p50/p95
105.492/107.513 ms; the unchanged material-regression gate passes.
