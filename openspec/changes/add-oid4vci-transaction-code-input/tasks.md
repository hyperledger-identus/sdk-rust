## 1. Contract and provenance

- [x] 1.1 Create issue #123 before implementation and record exact base,
  normative hash, consumer revisions/status, source paths/hashes, license,
  public/error contract, non-scope, threats, bounds, targets, and rollback.
- [x] 1.2 Define and semantically review the additive capability and complete
  IDR-023 replacement with zero unresolved blockers.

## 2. Implementation

- [x] 2.1 Add positive Transaction Code input limits, immediate zeroizing
  ownership, exact presence agreement, and the consuming prepared-input state.
- [x] 2.2 Add stable fieldless errors, crate exports, redacted diagnostics, and
  crate-private-only raw access for later request construction.
- [x] 2.3 Update ADR, blueprint, and canonical IDR-023 ledger pointer without
  changing dependencies, manifests, features, parsers, or consumers.

## 3. Verification and integration

- [x] 3.1 Add positive, negative, exact-bound, multibyte, ownership, and
  diagnostic-canary tests for every contract branch.
- [ ] 3.2 Run focused all-feature/no-default tests, strict Clippy/docs,
  workspace/factory/preservation gates, and the full target/supply-chain Nix
  matrix; record exact evidence and consumer-isolation receipts.
- [ ] 3.3 Complete and record a distinct local review with no unresolved
  finding, mark tasks complete, run ready/receipt, guarded archive, and final
  exact-head verification.
- [ ] 3.4 Push signed DCO commits, open a ready issue-linked PR to `develop`,
  pass exact-head hosted Codex review and every required CI gate, merge through
  protection, close #123, update #7/#20, sync `develop`, and clean the branch
  and worktree.
