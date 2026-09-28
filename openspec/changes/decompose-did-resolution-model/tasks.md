# Tasks

## 1. Planning and characterization

- [x] 1.1 Inventory public exports, values, result states, validation order,
      resource budgets, cleanup paths, internal consumers, and governance.
- [x] 1.2 Record the private module map, minimal sibling visibility, duplicate
      cleanup boundary, routine constraint impact, and rollback contract.
- [x] 1.3 Pass research/constraint/planning checks, commit planning alone, and
      write the exact-head pre-implementation receipt for issue #405.
- [x] 1.4 Run the complete existing `identus-did` all-feature suite and capture
      public/source invariants before moving code.

## 2. Implementation

- [ ] 2.1 Move scalar values and raw-wire preflight to cohesive private owners
      while preserving exact syntax, limits, errors, and parse order.
- [ ] 2.2 Move operation and document metadata with adjacent validation,
      rejection guards, and iterative cleanup intact.
- [ ] 2.3 Move resolution and dereferencing envelopes, preserve state matrices,
      and remove only the three characterized mechanical duplicates.
- [ ] 2.4 Prove public/wire/error/source equivalence and refresh code-health
      evidence without weakening unrelated ownership.

## 3. Verification and delivery

- [ ] 3.1 Run DID and workspace tests, strict Clippy/format, public/error/source
      contracts, factory checks, portable targets, and relevant Nix gates.
- [ ] 3.2 Perform a distinct exact-diff architecture/security/Rust review and
      resolve every blocking finding.
- [ ] 3.3 Archive the completed change, open an issue-linked signed/DCO PR to
      `develop`, publish metrics, and merge only after required CI is green.
