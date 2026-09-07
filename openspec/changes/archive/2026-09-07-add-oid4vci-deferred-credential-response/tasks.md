## 1. Contract and provenance

- [x] 1.1 Create issue #149 before implementation and record exact base,
  normative hash, consumer revisions, public/wire/error contract, non-scope,
  bounds, targets, isolation, and rollback.
- [x] 1.2 Define and semantically review the additive deferred body capability
  and complete IDR-023 replacement with zero unresolved blockers.

## 2. Implementation

- [x] 2.1 Add positive deferred response limits and a complete duplicate-safe
  parser for required transaction and interval members.
- [x] 2.2 Preserve the sensitive transaction ID and exact positive JSON-number
  interval without float conversion; reject branch ambiguity and redact all
  diagnostics.
- [x] 2.3 Update ADR, blueprint, inventory, and canonical IDR-023 ledger pointer
  without dependency, feature, manifest, lockfile, target, or consumer changes.

## 3. Verification and integration

- [x] 3.1 Add positive, numeric-form, negative, ambiguity, duplicate, boundary,
  extension, redaction, bridged-error, and consumer-shaped tests in both
  feature modes.
- [x] 3.2 Run focused feature modes, strict Clippy/docs, workspace/factory,
  target/MSRV/supply-chain, and full Nix gates; record exact evidence and
  consumer-isolation receipts.
- [x] 3.3 Complete and record a distinct exact-diff local review with no
  unresolved finding, mark tasks complete, run ready/receipt, guarded archive,
  and final exact-head verification.
- [x] 3.4 Prepare the signed/DCO issue-linked PR delivery packet for `develop`,
  then require exact-head hosted review, all CI gates, protected merge,
  issue/parent updates, `develop` sync, and cleanup.
