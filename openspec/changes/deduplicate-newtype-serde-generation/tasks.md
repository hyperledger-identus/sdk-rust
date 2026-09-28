# Tasks

## 1. Planning

- [x] 1.1 Inspect the duplicated token templates, category-specific behavior,
      expansion/runtime tests, output-safety contract, and code-health entry.
- [x] 1.2 Record the routine research decision, constraint impact, ADR, design,
      and capability delta.
- [x] 1.3 Pass strict planning readiness, commit planning alone, and write the
      exact-head pre-implementation receipt for issue #402.

## 2. Implementation

- [x] 2.1 Add positive compile evidence for validated and unvalidated string
      and numeric serde expansion before removing duplication.
- [x] 2.2 Extract one private scalar serde generator and delegate from the
      string and numeric category modules without changing emitted behavior.
- [x] 2.3 Refresh syntax-aware code-health evidence and retire the resolved
      duplicate disposition without weakening other signals.

## 3. Verification and delivery

- [x] 3.1 Run focused derive/trybuild tests, output-safety evidence, strict
      Clippy, workspace tests, factory checks, and relevant Nix gates.
- [x] 3.2 Perform a distinct exact-diff architecture/Rust review and resolve
      every blocking finding.
- [ ] 3.3 Archive the completed change, open an issue-linked signed/DCO PR to
      `develop`, publish metrics, and merge only after required CI is green.
