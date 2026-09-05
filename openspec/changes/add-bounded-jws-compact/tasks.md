## 1. Contract and provenance

- [x] 1.1 Create issue #95 under #8 / #20 / `IDR-004` and record the exact
  base, standards, dependency cone, threats, target matrix and acceptance.
- [x] 1.2 Record immutable donor revisions, file digests, license posture,
  behavior-only classification and read-only checkout states.
- [x] 1.3 Specify limits, canonicality, closed protected headers, explicit
  unverified state, error redaction and non-scope.
- [x] 1.4 Record ADR 0034 and complete the pre-implementation architecture,
  API, standards, security and performance review with no unresolved blocker.

## 2. Compact codec implementation

- [ ] 2.1 Add `identus-jose` with validated limits and protected-header values.
- [ ] 2.2 Implement allocation-bounded canonical base64url segment decoding and
  strict protected-header deserialization.
- [ ] 2.3 Implement staged signing-input encoding, signature attachment and
  exact, explicitly unverified parse accessors.
- [ ] 2.4 Add static redaction-safe errors, Debug output and core error bridge.

## 3. Evidence and performance

- [ ] 3.1 Add RFC 7515 and independently reconstructed Oxid/Portal-shaped
  positive conformance cases.
- [ ] 3.2 Add negative cases for every structural, canonical, header and size
  boundary, including valid empty payload handling.
- [ ] 3.3 Add deterministic bounded round-trip matrices and run the ignored
  release parse-throughput diagnostic.
- [ ] 3.4 Update architecture rulebook, tests, human/machine inventory,
  blueprint status and the canonical `IDR-004` ledger.
- [ ] 3.5 Verify both donor checkout states still match their preflight state.

## 4. Verification and delivery

- [ ] 4.1 Pass focused formatting, tests, no-default check, strict Clippy and
  warning-denied docs.
- [ ] 4.2 Pass workspace tests, factory/conformance, target/MSRV,
  supply-chain and full Nix gates.
- [ ] 4.3 Complete a distinct post-implementation architecture/API/standards/
  security/performance review and resolve every finding.
- [ ] 4.4 Produce ready/receipt, synchronize canonical specs, archive the
  change, and prepare the signed/DCO issue-linked PR; hosted review/CI, merge,
  effort receipt, parent updates, develop sync and cleanup remain GitHub
  evidence.
