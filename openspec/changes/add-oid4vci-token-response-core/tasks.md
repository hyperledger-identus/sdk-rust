## 1. Contract and provenance

- [x] 1.1 Create issue #127 before implementation and record exact base,
  normative hashes, consumer revisions/status, source paths/hashes, license,
  public/wire/error contract, non-scope, threats, bounds, targets, and rollback.
- [x] 1.2 Define and semantically review the additive capability and complete
  IDR-023 replacement with zero unresolved blockers.

## 2. Implementation

- [ ] 2.1 Add positive response limits and bounded strict core parsing with
  RFC token, token-type, expiry, and scope validation.
- [ ] 2.2 Add zeroizing partial response/token types, stable fieldless errors,
  explicit sensitive access, crate exports, and redacted diagnostics.
- [ ] 2.3 Update ADR, blueprint, and canonical IDR-023 ledger pointer without
  changing dependencies, manifests, features, or consumers.

## 3. Verification and integration

- [ ] 3.1 Add positive, negative, grammar, bound, extension, and diagnostic
  canary tests for every contract branch.
- [ ] 3.2 Run focused all-feature/no-default tests, strict Clippy/docs,
  workspace/factory/preservation gates, and the full target/supply-chain Nix
  matrix; record exact evidence and consumer-isolation receipts.
- [ ] 3.3 Complete and record a distinct local review with no unresolved
  finding, mark tasks complete, run ready/receipt, guarded archive, and final
  exact-head verification.
- [ ] 3.4 Prepare the signed/DCO, issue-linked PR delivery packet for
  `develop`, including exact-head hosted Codex review and required CI gates,
  protected merge, issue/parent updates, `develop` sync, and cleanup as the
  post-archive integration procedure.
