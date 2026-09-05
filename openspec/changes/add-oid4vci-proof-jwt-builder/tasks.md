## 1. Contract and standards

- [x] 1.1 Confirm #99, exact `develop` base, JOSE/DID readiness, repository
  rules and consumer isolation.
- [x] 1.2 Pin OpenID4VCI 1.0 Final Appendix F.1/F.4 plus RFC 7515, RFC 7519,
  RFC 8725 and RFC 9864 behavior used by the builder.
- [x] 1.3 Specify compatibility, key-reference/claim bounds, staged API,
  errors, dependency cone and verifier split.
- [x] 1.4 Record ADR 0036 and complete pre-implementation architecture, API,
  standards, security and performance review with no unresolved blocker.

## 2. Header and builder implementation

- [x] 2.1 Add the bounded exclusive JOSE key-reference model while preserving
  the existing header constructor and wire behavior.
- [x] 2.2 Add the OID4VCI client/claims/limits inputs and static error surface.
- [x] 2.3 Add staged proof preparation, exact signer delegation and typed signed
  proof output without ambient clock, custody or verification.

## 3. Evidence and integration

- [ ] 3.1 Add Final-spec key-reference and identified/anonymous positive cases.
- [ ] 3.2 Add negative tests for ambiguous/private/malformed/unbounded inputs,
  algorithm confusion, signer isolation and redaction.
- [ ] 3.3 Add independently reconstructed Oxid behavior and Lace deferred-gap
  cases plus release-only builder throughput evidence.
- [ ] 3.4 Update canonical architecture/specs/docs and verify consumer HEAD/
  status receipts remain unchanged.

## 4. Verification and delivery

- [ ] 4.1 Pass focused formatting, tests, minimal features, strict Clippy and
  warning-denied docs.
- [ ] 4.2 Pass workspace, factory/conformance, target/MSRV, supply-chain and
  full Nix gates.
- [ ] 4.3 Complete a distinct post-implementation architecture/API/standards/
  security/performance review and resolve every finding.
- [ ] 4.4 Produce ready/receipt, sync canonical specs, archive the change and
  prepare the signed/DCO issue-linked PR; hosted review, green CI, merge,
  roadmap receipts and cleanup remain GitHub evidence.
