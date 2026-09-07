## 1. Contract and provenance

- [x] 1.1 Create issue #137 before implementation and record exact base,
  normative hashes, consumer revisions/status, public/wire/error contract,
  non-scope, bounds, targets and rollback.
- [x] 1.2 Define and semantically review the additive capability and complete
  IDR-023 replacement with zero unresolved blockers.

## 2. Implementation

- [x] 2.1 Add positive HTTP response limits and request-bound validation without
  an executor, generic headers or retained transport input.
- [x] 2.2 Implement bounded RFC-shaped media-type and Cache-Control parsing plus
  static fieldless error bridges.
- [x] 2.3 Update ADR, blueprint and canonical IDR-023 ledger pointer without
  changing dependencies, manifests, lockfile, features or consumers.

## 3. Verification and integration

- [x] 3.1 Add status, boundary, casing, parameter, quoted delimiter, malformed,
  injection, no-store, body-order, consumer-shaped and canary tests for every
  contract branch.
- [ ] 3.2 Run focused all-feature/no-default tests, strict Clippy/docs,
  workspace/factory/preservation gates and the full target/supply-chain Nix
  matrix; record exact evidence and consumer-isolation receipts.
- [ ] 3.3 Complete and record a distinct exact-diff local review with no
  unresolved finding, mark tasks complete, run ready/receipt, guarded archive
  and final exact-head verification.
- [ ] 3.4 Prepare the signed/DCO, issue-linked PR delivery packet for
  `develop`, then require exact-head hosted review, all CI gates, protected
  merge, issue/parent updates, `develop` sync and cleanup.
