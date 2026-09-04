## 1. Contract and provenance

- [x] 1.1 Create issue #81 under `IDR-008` / #20 and record the exact base.
- [x] 1.2 Record normative versions, donor revisions/paths/digests/licenses,
  conceptual roles, consumer states and read-only isolation.
- [x] 1.3 Specify selected-claim, credential-selection, disclosure-plan,
  privacy, performance, error, compatibility and deliberate non-scope rules.
- [x] 1.4 Record ADR 0028 and complete a pre-implementation semantic/API/
  privacy/performance review with no unresolved blocker.

## 2. Presentation selection implementation

- [x] 2.1 Add bounded selected-claim and credential-selection values with
  redacted diagnostics.
- [x] 2.2 Reuse candidate-set request validation and add disclosure-plan
  validation for candidate membership, claims, query coverage and multiplicity.
- [x] 2.3 Add static error bridges and root exports without a new dependency.

## 3. Evidence and performance

- [x] 3.1 Add shared DCQL-, Midnight- and unrelated-format positive tests.
- [x] 3.2 Add maximum-bound, duplicate, cross-request, unknown candidate,
  multiplicity, claim mismatch, required coverage, redaction and complete
  error-contract negative tests.
- [x] 3.3 Extend and run the manual release diagnostic and record its exact
  toolchain, host and result without a timing threshold.
- [x] 3.4 Update human and machine inventories plus canonical governance
  evidence for the added experimental surface.
- [ ] 3.5 Verify donor and consumer final HEAD/branch/status receipts equal
  preflight.

## 4. Verification and delivery

- [x] 4.1 Pass focused formatting, tests, no-default check, strict Clippy and
  warning-denied docs.
- [ ] 4.2 Pass workspace tests, factory/conformance, target/MSRV,
  supply-chain and full Nix gates.
- [ ] 4.3 Complete a distinct post-implementation architecture/API/privacy/
  performance review and resolve every finding.
- [ ] 4.4 Produce ready/receipt, synchronize canonical specs, archive the
  change, and prepare the signed/DCO issue-linked PR; hosted CI/review, merge,
  effort receipt, parent updates, `develop` sync and cleanup remain GitHub
  evidence.
