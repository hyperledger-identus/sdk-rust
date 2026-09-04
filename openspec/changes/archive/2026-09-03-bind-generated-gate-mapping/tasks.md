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
- [x] 2.14 Recognize unprefixed relative path literals at token boundaries.
- [x] 2.15 Consume the escaped character after an indented-string control
      escape prefix.
- [x] 2.16 Optimize the path-classifier hot branch without weakening its
      lexical cases.
- [x] 2.17 Recognize punctuation-prefixed unprefixed relative paths at token
      boundaries.
- [x] 2.18 Validate the canonical `perSystem` package-set provider in
      `flake.nix` and reject helper overrides.
- [x] 2.19 Stop path scanning at line-comment markers.
- [x] 2.20 Reject lexical wrappers that can shadow the root flake's builtin
      `import`.
- [x] 2.21 Reject check-wrapper priority overrides that can erase imported
      generated checks.
- [x] 2.22 Walk the recursively reachable local flake module graph and reject
      competing priority overrides.
- [x] 2.23 Recognize general non-double-slash Nix URI literals.
- [x] 2.24 Detect quoted priority-constructor selections and raw module
      override records throughout the reachable graph.
- [x] 2.25 Resolve parent-relative as well as child-relative local imports.
- [x] 2.26 Reject `disabledModules` in the repository-local module graph.
- [x] 2.27 Fail closed on imports values that are not direct literal lists.
- [x] 2.28 Reject raw module `_type` tags across all value spellings.
- [x] 2.29 Cache immutable module-source analysis without weakening graph
      validation.
- [x] 2.30 Normalize quoted import bindings and reject interpolated import
      paths.
- [x] 2.31 Reject computed attribute selectors and competing sibling checks
      contributions outside canonical gate modules.
- [x] 2.32 Preserve unresolved root-import evidence while allowing only static
      external flake-module entries.
- [x] 2.33 Reject inherited imports whose repository-local graph cannot be
      traversed from source.
- [x] 2.34 Reject reflective priority-constructor access outside the canonical
      generator.
- [x] 2.35 Reject dynamic attribute-set construction that can synthesize
      protected module keys.
- [x] 2.36 Reject inherited protected keys and cover computed raw `_type`
      spelling explicitly.
- [x] 2.37 Restrict import discovery and required-generator validation to
      immediate bindings of the returned module attribute set.
- [x] 2.38 Reject executable imports and explicit top-level config composition
      in reachable repository modules.
- [x] 2.39 Detect inherited raw module `_type` fields.
- [x] 2.40 Anchor root imports to the canonical `outputs` `mkFlake` result.
- [x] 2.41 Traverse immediate imports bound through a statically computed
      indented-string attribute name.
- [x] 2.42 Bind `makeGate`, its Cargo argument helpers, and their trusted
      library imports to canonical manifest-driven Crane dispatch.
- [x] 2.43 Replace cold-path statement normalization and hashing with direct
      canonical formatted-source comparison preserving every significant byte.
- [x] 2.44 Bind exempt external flake modules to canonical direct input URLs.
- [x] 2.45 Bind etalon and MSRV Crane providers to canonical direct toolchain
      construction.
- [x] 2.46 Anchor the package-provider check to the immediate root-module
      `perSystem` statement.
- [x] 2.47 Normalize statically quoted executable builtin selections and quoted
      attribute names containing a complete static interpolation.
- [x] 2.48 Reject trailing expression composition after the canonical
      `perSystem` provider result.
- [x] 2.49 Validate canonical root-input mappings and direct locked GitHub
      provenance.
- [x] 2.50 Normalize indented-string protected names containing one static
      interpolation.
- [x] 2.51 Reject trailing expression composition after the canonical root
      `mkFlake` invocation.
- [x] 2.52 Normalize indented-string layout whitespace when matching protected
      binding roots.
- [x] 2.53 Validate the canonical topology and provenance of every reachable
      executable provider lock edge.
- [x] 2.54 Require exactly one canonical provider-result statement.
- [x] 2.55 Reject nontrivial interpolation in quoted attribute-name strings.
- [x] 2.56 Normalize indented and statically interpolated priority-constructor
      selections.
- [x] 2.57 Traverse literal imports from every reachable deferred `perSystem`
      result module.
- [x] 2.58 Reject indented control escapes used to construct attribute names.
- [x] 2.59 Reject explicit root-module `config` composition.
- [x] 2.60 Reject indirect reflective priority lookup through path helpers.
- [x] 2.61 Require the exact canonical root-module statement set.
- [x] 2.62 Require the exact canonical first `mkFlake` argument.
- [x] 2.63 Reject executable `scopedImport` in reachable local modules.

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
- [x] 3.11 Rerun complete local evidence after relative-path and control-escape
      corrections and prepare both review threads for hosted clearance.
- [x] 3.12 Pass the local material-regression guard with all 64 lexical
      mutations intact and prepare the hosted performance rerun.
- [x] 3.13 Rerun complete local evidence after the punctuation-prefixed path
      correction and prepare its exact-head review thread for clearance.
- [x] 3.14 Rerun complete local evidence after package-provider and
      path-comment corrections and prepare both exact-head review threads for
      clearance.
- [x] 3.15 Rerun complete local evidence after the root-import correction and
      prepare its exact-head review thread for clearance.
- [x] 3.16 Rerun complete local evidence after the wrapper-composition
      correction and prepare its exact-head review thread for clearance.
- [x] 3.17 Rerun complete local evidence after module-graph and general-URI
      corrections and prepare both exact-head review threads for clearance.
- [x] 3.18 Rerun complete evidence after quoted-constructor,
      parent-relative-import, and raw-override corrections and prepare all
      three exact-head review threads for clearance.
- [x] 3.19 Rerun complete evidence after disabled-module, unresolved-import,
      and indented/raw-tag corrections and prepare all three exact-head review
      threads for clearance.
- [x] 3.20 Clear the hosted warm-latency regression without changing the
      material-regression threshold or any functional mutation.
- [x] 3.21 Rerun complete evidence after quoted-import, computed-selector, and
      interpolated-import corrections and prepare all three review threads for
      clearance.
- [x] 3.22 Rerun complete evidence after the closed module-authoring boundary
      correction and prepare all five review threads for clearance.
- [x] 3.23 Rerun complete evidence after module-result isolation and prepare
      the nested-import-decoy thread for clearance.
- [x] 3.24 Rerun complete evidence after imported-config and inherited-priority
      corrections and prepare both review threads for clearance.
- [x] 3.25 Rerun complete evidence after canonical-root anchoring and
      indented-string import normalization, then prepare both review threads
      for clearance.
- [x] 3.26 Rerun complete evidence after generator-dispatch binding and prepare
      the delayed no-op-gate review thread for clearance.
- [x] 3.27 Clear the hosted process-cold performance gate with all 107
      functional mutations intact.
- [x] 3.28 Rerun complete evidence after provider-provenance, root anchoring,
      quoted-selector, and static-name corrections, then prepare all seven
      review threads for clearance.
- [x] 3.29 Rerun complete evidence after provider-result boundary enforcement
      and prepare the exact-head review thread for clearance.
- [x] 3.30 Rerun complete evidence after locked-provenance and indented-name
      enforcement and prepare both delayed review threads for clearance.
- [x] 3.31 Rerun complete evidence after root-output boundary enforcement and
      prepare the exact-head review thread for clearance.
- [x] 3.32 Rerun complete evidence after indented-name and transitive-lock
      enforcement and prepare both exact-head review threads for clearance.
- [x] 3.33 Rerun complete evidence after provider-result closure and ambiguous
      quoted-name rejection, then prepare both delayed threads for clearance.
- [x] 3.34 Rerun complete evidence after normalized priority selection and
      deferred-module traversal, then prepare both exact-head threads for
      clearance.
- [x] 3.35 Rerun complete evidence after indented control-escape rejection and
      prepare the exact-head review thread for clearance.
- [x] 3.36 Rerun complete evidence after root-config rejection and prepare the
      exact-head review thread for clearance.
- [x] 3.37 Rerun complete evidence after indirect reflective-access rejection
      and prepare the exact-head review thread for clearance.
- [x] 3.38 Rerun complete evidence after closing the root-module statement set
      and prepare the exact-head review thread for clearance.
- [x] 3.39 Rerun complete evidence after canonical `mkFlake` input binding and
      `scopedImport` rejection, then prepare both exact-head review threads for
      clearance.
