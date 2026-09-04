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
