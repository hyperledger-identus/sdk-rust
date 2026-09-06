## 1. Contract and provenance

- [x] 1.1 Create issue #135 before implementation and record exact base,
  normative hash, consumer revisions/status, source paths/hashes, license
  posture, public/wire/error contract, non-scope, bounds, targets and rollback.
- [x] 1.2 Define and semantically review the additive capability and complete
  IDR-023 replacement with zero unresolved blockers.

## 2. Implementation

- [ ] 2.1 Add the owned request value and metadata-bound constructor without a
  raw-string or automatic-transport surface.
- [ ] 2.2 Expose exact POST, empty-body and no-access-token guidance plus the
  static missing-endpoint error.
- [ ] 2.3 Update ADR, blueprint and canonical IDR-023 ledger pointer without
  changing dependencies, manifests, lockfile, features or consumers.

## 3. Verification and integration

- [ ] 3.1 Add positive, missing-endpoint, ownership, consumer-shaped and
  diagnostic-canary tests for every contract branch.
- [ ] 3.2 Run focused all-feature/no-default tests, strict Clippy/docs,
  workspace/factory/preservation gates and the full target/supply-chain Nix
  matrix; record exact evidence and consumer-isolation receipts.
- [ ] 3.3 Complete and record a distinct exact-diff local review with no
  unresolved finding, mark tasks complete, run ready/receipt, guarded archive
  and final exact-head verification.
- [ ] 3.4 Prepare the signed/DCO, issue-linked PR delivery packet for
  `develop`, then require exact-head hosted review, all CI gates, protected
  merge, issue/parent updates, `develop` sync and cleanup.
