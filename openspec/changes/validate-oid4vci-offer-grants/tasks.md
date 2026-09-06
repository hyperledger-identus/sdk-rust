## 1. Contract and provenance

- [x] 1.1 Create issue #115 under #7 / #20 / `IDR-023` with exact base,
  standards, public contract, bounds, threats, dependency cone, and non-scope.
- [x] 1.2 Record normative hashes, consumer revisions/path digests, license
  posture, behavior-only use, and read-only checkout states.
- [x] 1.3 Specify the consuming state transition, known/unknown grant handling,
  Transaction Code defaults, limits, redaction, compatibility, and rollback.
- [x] 1.4 Complete pre-implementation architecture, API, standards, security,
  privacy, and resource review with no unresolved blocker.

## 2. Grant implementation

- [x] 2.1 Add grant limits and static redacted error/core-code variants.
- [x] 2.2 Extend the selective bounded scanner for known grants, opaque
  extensions, strict Transaction Code fields, and exact integer/character
  limits.
- [x] 2.3 Add grant, Transaction Code, and Authorization Server value types plus
  the consuming `CredentialOfferWithGrants` transition and safe accessors.

## 3. Evidence and architecture

- [x] 3.1 Add official Authorization Code and Pre-Authorized Code examples,
  both-known-grant, empty-grants, and independently reconstructed consumer
  positives.
- [x] 3.2 Add type/empty/bound/mode/integer/legacy-null/unknown-extension and
  diagnostic-canary negatives plus direct/invocation transition equivalence.
- [x] 3.3 Record ADR 0043 and advance the blueprint/canonical `IDR-023` issue
  without claiming metadata, protocol state, or the full engine is delivered.
- [x] 3.4 Verify consumer HEAD/status receipts exactly match preflight.

## 4. Verification and delivery

- [ ] 4.1 Pass focused fmt, all/no-default tests, strict Clippy, and docs.
- [ ] 4.2 Pass factory, workspace, target/MSRV, supply-chain, and full Nix gates.
- [ ] 4.3 Complete a distinct post-implementation review and resolve every
  architecture/API/standards/security/resource finding.
- [ ] 4.4 Produce ready/receipt, synchronize and archive specs, then open the
  signed/DCO issue-linked PR for exact-head hosted CI/review and eligible merge.
