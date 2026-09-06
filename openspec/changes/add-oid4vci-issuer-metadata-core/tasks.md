## 1. Contract and provenance

- [x] 1.1 Create issue #117 under #7 / #20 / `IDR-023` with exact base,
  standards, public contract, bounds, threats, dependency cone, and non-scope.
- [x] 1.2 Record normative hash, consumer revisions/path digests, license
  posture, behavior-only use, and read-only checkout states.
- [x] 1.3 Specify unsigned metadata parsing, offer matching, exact/default
  Authorization Server behavior, limits, redaction, compatibility, and rollback.
- [x] 1.4 Complete pre-implementation architecture, API, standards, security,
  privacy, and resource review with no unresolved blocker.

## 2. Metadata implementation

- [x] 2.1 Add metadata limits and static redacted error/core-code variants.
- [x] 2.2 Extend the selective bounded scanner for required metadata members,
  advertised Authorization Servers, endpoints, and configuration summaries.
- [x] 2.3 Add unsigned metadata types, safe accessors, exact JSON retention, and
  the consuming `CredentialOfferWithMetadata` agreement transition.

## 3. Evidence and architecture

- [x] 3.1 Add official Final-shaped, Oxid-shaped, and Lace-shaped positives for
  explicit/default/multiple Authorization Servers and opaque extensions.
- [x] 3.2 Add malformed/type/duplicate/bound/endpoint/issuer/configuration/hint
  negatives, exact-bound cases, and diagnostic canaries.
- [x] 3.3 Record the ADR and advance the blueprint/canonical `IDR-023` issue
  without claiming discovery, grant selection, trust, or engine completion.
- [x] 3.4 Verify consumer HEAD/status receipts exactly match preflight.

## 4. Verification and delivery

- [x] 4.1 Pass focused fmt, all/no-default tests, strict Clippy, and docs.
- [x] 4.2 Pass factory, workspace, target/MSRV, supply-chain, and full Nix gates.
- [x] 4.3 Complete a distinct post-implementation review and resolve every
  architecture/API/standards/security/resource finding.
- [ ] 4.4 Produce ready/receipt, synchronize and archive specs, then open the
  signed/DCO issue-linked PR for exact-head hosted CI/review and eligible merge.
