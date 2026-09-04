# Tasks

## 1. Contract

- [x] 1.1 Link issue #61 to the late PR #60 review evidence.
- [x] 1.2 Define the mapped-name and mapped-value fail-closed scenarios.
- [x] 1.3 Complete a semantic review with no unresolved blocker.

## 2. Implementation

- [x] 2.1 Bind the accepted mapping shape to `gate.name` and `makeGate gate`.
- [x] 2.2 Add constant-name and detached-value mutation regressions.
- [x] 2.3 Require mapping helpers from `pkgs.lib` and cover local replacements.
- [x] 2.4 Require all mapping inputs and output in the immediate outer
      `perSystem` scope and cover nested shadowing.
- [x] 2.5 Reject same-scope shadowing of `builtins` and `pkgs` with exact
      mutations.
- [x] 2.6 Strip comments only outside Nix strings and cover line/block markers
      embedded in valid strings.
- [x] 2.7 Reject enclosing lexical wrappers around the canonical `perSystem`
      module result.
- [x] 2.8 Recognize escaped delimiters in indented Nix strings with exact
      positive regressions.
- [x] 2.9 Reject quoted and dynamic immediate binding roots.
- [x] 2.10 Track interpolation nesting and nested strings in both Nix string
      forms.
- [x] 2.11 Reject `builtins` in immediate `perSystem` function formals.
- [x] 2.12 Skip path/URI tokens during scope scanning and require token context
      for indented-string openers.
- [x] 2.13 Continue path scanning after balanced interpolation expressions.
- [ ] 2.14 Recognize unprefixed relative path literals at token boundaries.
- [ ] 2.15 Consume the escaped character after an indented-string control
      escape prefix.

## 3. Verification and integration

- [x] 3.1 Pass focused policy, Ruff, factory, benchmark, and full Nix gates.
- [x] 3.2 Record a distinct local review and immutable evidence receipt.
- [x] 3.3 Sync the capability specification and archive the completed change.
- [x] 3.4 Prepare the signed, issue-linked PR handoff and its green-CI/no-blocking-
      review integration conditions.
- [x] 3.5 Rerun complete local evidence after the nested-shadow correction and
      prepare the final hosted review-thread clearance gate.
- [x] 3.6 Rerun complete local evidence after trusted-root and lexical-comment
      corrections and prepare both threads for hosted clearance.
- [x] 3.7 Rerun complete local evidence after enclosing-scope and
      indented-string corrections and prepare both threads for hosted clearance.
- [x] 3.8 Rerun complete local evidence after quoted-root and interpolation
      corrections and prepare both review threads for hosted clearance.
- [x] 3.9 Rerun complete local evidence after formal-root and token-context
      corrections and prepare all three review threads for hosted clearance.
- [x] 3.10 Rerun complete local evidence after the interpolated-path correction
      and prepare its review thread for hosted clearance.
- [ ] 3.11 Rerun complete local evidence after relative-path and control-escape
      corrections and prepare both review threads for hosted clearance.
