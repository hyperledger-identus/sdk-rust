## 1. Contract and provenance

- [x] 1.1 Create issue #113 under #7 / #20 / `IDR-023` with exact base,
  standards, public contract, bounds, threats, dependency cone, and non-scope.
- [x] 1.2 Record the immutable normative hash, consumer revisions/path digests,
  license posture, behavior-only use, and read-only checkout states.
- [x] 1.3 Specify state transition, required fields, extensions, grants shape,
  semantic limits, redaction, compatibility, non-scope, and rollback.
- [x] 1.4 Record ADR 0042 and complete pre-implementation architecture, API,
  standards, security, and resource review with no unresolved blocker.

## 2. Semantic implementation

- [x] 2.1 Add semantic limits and static redacted error/core-code variants.
- [x] 2.2 Implement selective required-member parsing from consumed validated
  transport without materializing unknown values or large numeric extensions.
- [x] 2.3 Validate the issuer identifier, configuration IDs, and grants shape;
  retain exact JSON and expose only explicit accessors/redacted diagnostics.

## 3. Evidence and architecture

- [x] 3.1 Add official and independent consumer-shaped positive tests.
- [x] 3.2 Add missing/type/duplicate/URI/grants/exact-bound/numeric-extension and
  diagnostic-canary negatives plus direct/invocation transition equivalence.
- [x] 3.3 Update the blueprint and canonical `IDR-023` ledger without claiming
  the full issuance engine is delivered.
- [x] 3.4 Verify consumer HEAD/status receipts exactly match preflight.

## 4. Verification and delivery

- [x] 4.1 Pass focused fmt, all/no-default tests, strict Clippy, and docs.
- [ ] 4.2 Pass factory, workspace, target/MSRV, supply-chain, and full Nix gates.
- [ ] 4.3 Complete a distinct post-implementation review and resolve all
  architecture/API/standards/security/resource findings.
- [ ] 4.4 Produce ready/receipt, synchronize and archive specs, then open the
  signed/DCO issue-linked PR for exact-head hosted CI/review and eligible merge.
