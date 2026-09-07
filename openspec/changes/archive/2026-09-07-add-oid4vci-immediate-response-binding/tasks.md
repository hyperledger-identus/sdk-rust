## 1. Contract and provenance

- [x] 1.1 Create issue #143 before implementation and record exact base,
  normative hashes, consumer revisions/paths/licenses, public/wire/error
  contract, non-scope, bounds, targets and rollback.
- [x] 1.2 Define and semantically review the additive capability and complete
  IDR-023 replacement with zero unresolved blockers.

## 2. Implementation

- [x] 2.1 Add positive immediate HTTP limits and extract the existing private
  HTTP field grammar without public Nonce behavior drift.
- [x] 2.2 Add request-bound immediate response validation for exact status,
  media, bounded body and the necessary proof-count upper bound.
- [x] 2.3 Add static fieldless errors and update ADR, blueprint, inventory and
  canonical IDR-023 ledger pointer without dependency or target changes.

## 3. Verification and integration

- [x] 3.1 Add status, media, boundary, validation-order, equal/fewer/excess
  cardinality, state, consumer-shaped and diagnostic-canary tests plus Nonce
  HTTP regression evidence.
- [x] 3.2 Run focused feature modes, strict Clippy/docs, workspace/factory,
  target/MSRV/supply-chain and full Nix gates; record exact evidence and
  consumer-isolation receipts.
- [x] 3.3 Complete and record a distinct exact-diff local review with no
  unresolved finding, mark tasks complete, run ready/receipt, guarded archive
  and final exact-head verification.
- [x] 3.4 Prepare the signed/DCO, issue-linked PR delivery packet for
  `develop`, then require exact-head hosted review, all CI gates, protected
  merge, issue/parent updates, `develop` sync and cleanup.
