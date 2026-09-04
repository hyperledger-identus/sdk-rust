## 1. Contract and provenance

- [x] 1.1 Create issue #79 under `IDR-008` / #20 and record the exact base.
- [x] 1.2 Record normative versions, donor revisions/paths/digests/licenses,
  conceptual-adaptation roles, consumer states, and isolation.
- [x] 1.3 Specify scalar, query, request, candidate, privacy, performance,
  stable-error, compatibility, and deliberate non-scope contracts.
- [x] 1.4 Record ADR 0027 and complete a pre-implementation semantic/API/
  privacy/performance review with no unresolved blocker.

## 2. Presentation contract implementation

- [ ] 2.1 Add bounded query ID, purpose, challenge, and credential-handle
  values with redacted diagnostics.
- [ ] 2.2 Add claim-intent/request and bounded credential-query contracts that
  reuse credential descriptors.
- [ ] 2.3 Add bounded presentation requests and candidate/candidate-set
  validation against request membership, format, and claim coverage.
- [ ] 2.4 Add static presentation error bridges, root exports, dependency edge,
  and implemented component metadata.

## 3. Evidence and performance

- [ ] 3.1 Add Oxid-, Midnight-, DCQL-, and unrelated-format positive tests.
- [ ] 3.2 Add exact scalar/collection boundaries, duplicates, mismatches,
  redaction, no-panic corpus, and complete error-contract negative tests.
- [ ] 3.3 Add and run a manual release construction diagnostic and record the
  toolchain/host/result without a timing threshold.
- [ ] 3.4 Update human/machine inventories and canonical governance evidence.
- [ ] 3.5 Verify donor/consumer final HEAD/branch/status receipts match
  preflight.

## 4. Verification and delivery

- [ ] 4.1 Pass focused formatting, tests, no-default check, strict Clippy, and
  warning-denied docs.
- [ ] 4.2 Pass workspace tests, factory/conformance, target/MSRV,
  supply-chain, and full Nix gates.
- [ ] 4.3 Complete a distinct post-implementation architecture/API/privacy/
  performance review and resolve every finding.
- [ ] 4.4 Produce ready/receipt, synchronize canonical specs, archive the
  change, and prepare the signed/DCO issue-linked PR; hosted CI/review, merge,
  effort receipt, parent updates, `develop` sync, and cleanup remain GitHub
  evidence.
