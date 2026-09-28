# Tasks

## 1. Planning and characterization

- [ ] 1.1 Inventory public exports, validation order, budgets, cleanup paths,
      lifecycle invariants, port semantics, and internal consumers.
- [ ] 1.2 Record private module ownership, dependency direction, compatibility,
      non-goals, risk, and rollback.
- [ ] 1.3 Pass research/constraint/planning checks, commit planning alone, and
      write the exact-head pre-implementation receipt for issue #406.
- [ ] 1.4 Run the complete pre-move `identus-did` all-feature suite and capture
      public/source invariants.

## 2. Implementation

- [ ] 2.1 Move bounded public JSON policy with validation and cleanup intact.
- [ ] 2.2 Move identifiers, actions, jobs, and request models without API drift.
- [ ] 2.3 Move lifecycle/result validation and the registrar port without
      changing method, continuation, metadata, or runtime semantics.
- [ ] 2.4 Prove public/error/source equivalence and refresh code-health evidence
      without weakening unrelated ownership.

## 3. Verification and delivery

- [ ] 3.1 Run DID/workspace tests, strict Clippy/format, public/error/source
      contracts, factory checks, portable targets, and relevant Nix gates.
- [ ] 3.2 Perform a distinct exact-diff architecture/security/Rust review and
      resolve every blocking finding.
- [ ] 3.3 Archive the change, open an issue-linked signed/DCO PR to `develop`,
      publish metrics, and merge only after required CI is green.
