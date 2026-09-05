## 1. Contract and provenance

- [x] 1.1 Create issue #89 under #20 / `IDR-010` and record the exact base.
- [x] 1.2 Record donor revisions, file digests, licenses, semantic-adapt
  classification, consumer states and repository isolation.
- [x] 1.3 Specify ownership, capability separation, associated types,
  object-safe async execution, concurrency, pagination, redaction and non-scope.
- [x] 1.4 Record ADR 0032 and complete the pre-implementation architecture,
  API, security and performance review with no unresolved blocker.

## 2. Storage contract implementation

- [x] 2.1 Add bounded opaque revision/cursor/page vocabulary and static errors.
- [x] 2.2 Add stored/write/delete receipts with value-redacted Debug behavior.
- [x] 2.3 Add five separate associated-type, object-safe async store traits.
- [x] 2.4 Activate and export the wallet surface with only core/derive edges.

## 3. Evidence and performance

- [x] 3.1 Add consumer-shaped exact-key, missing-read, conditional mutation,
  revision-rotation and deletion tests.
- [x] 3.2 Add pagination, redaction, object-safety, non-enumerable-secret and
  concurrent-dispatch tests.
- [x] 3.3 Add and run an ignored release trait-object dispatch diagnostic.
- [x] 3.4 Update human/machine inventories and the canonical IDR-010 ledger.
- [x] 3.5 Verify each donor/consumer postflight state matches preflight.

## 4. Verification and delivery

- [ ] 4.1 Pass focused formatting, tests, no-default check, strict Clippy and
  warning-denied docs.
- [ ] 4.2 Pass workspace tests, factory/conformance, target/MSRV,
  supply-chain and full Nix gates.
- [ ] 4.3 Complete a distinct post-implementation architecture/API/security/
  performance review and resolve every finding.
- [ ] 4.4 Produce ready/receipt, synchronize canonical specs, archive the
  change, and prepare the signed/DCO issue-linked PR; hosted review/CI, merge,
  effort receipt, parent update, develop sync and cleanup remain GitHub
  evidence.
