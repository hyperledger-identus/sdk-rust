## 1. Contract and provenance

- [x] 1.1 Create issue #71 under #6 / `IDR-007` and record the exact base.
- [x] 1.2 Record the Oxid source revision, path, digests, license, and `adapt`
  classification without mutating the consumer.
- [x] 1.3 Specify the open format grammar, artifact limits, error bridge,
  lifecycle boundary, compatibility, and deliberate non-scope.
- [x] 1.4 Record ADR 0023 and complete pre-implementation semantic/API/security
  review with no unresolved blocker.

## 2. Credential envelope implementation

- [x] 2.1 Replace the marker with focused format, artifact, envelope, and error
  modules while keeping the crate dependency cone minimal.
- [x] 2.2 Implement open `CredentialFormat` parsing and exact preservation.
- [x] 2.3 Implement the three bounded artifact types, private-material
  zeroization, and redacted formatting.
- [x] 2.4 Implement the non-validating `CredentialEnvelope` and stable error
  bridge.

## 3. Evidence and repository contracts

- [x] 3.1 Add boundary, negative, redaction, erasure, error, multi-format, and
  Oxid-shaped consumer tests.
- [x] 3.2 Reclassify `identus-credentials` in machine/human inventory and sync
  crate-ring expectations.
- [ ] 3.3 Verify Oxid and midnight-identity preflight/final HEAD and status are
  unchanged.

## 4. Verification and delivery

- [x] 4.1 Pass focused crate tests, no-default build/check, strict Clippy,
  formatting, and warning-denied docs.
- [ ] 4.2 Pass workspace tests, factory/conformance, target/MSRV, supply-chain,
  and full Nix gates.
- [ ] 4.3 Complete a distinct post-implementation API/security review and
  resolve every finding.
- [ ] 4.4 Produce ready/receipt, synchronize canonical specs, archive the
  change, and prepare the signed/DCO issue-linked PR; hosted CI, merge, effort
  receipt, parent updates, `develop` sync, and cleanup remain GitHub evidence.
