## 1. Contract and provenance

- [x] 1.1 Create issue #119 under #7 / #20 / `IDR-023` with exact base,
  normative profile, partial-conformance boundary, bounds, threats, dependency
  cone, and non-scope.
- [x] 1.2 Record normative hashes, consumer revisions/path digests, behavior-only
  use, license posture, and read-only checkout states.
- [x] 1.3 Specify exact issuer binding, endpoint/grant/anonymous flag semantics,
  RFC defaults, limits, redaction, compatibility, and rollback.
- [x] 1.4 Complete pre-implementation architecture, standards, security,
  privacy, resource, compatibility, and provenance review without blockers.

## 2. Core implementation

- [x] 2.1 Add Authorization Server Metadata limits and static redacted errors.
- [x] 2.2 Extend the selective bounded scanner for issuer, optional endpoints,
  optional grant types, and the OID4VCI anonymous-access flag.
- [x] 2.3 Add the partial core type, safe accessors, exact JSON retention, and
  explicit/effective default semantics.

## 3. Evidence and architecture

- [x] 3.1 Add official- and consumer-shaped positives for endpoints, explicit
  and omitted grants, anonymous-access omission/default, and opaque extensions.
- [x] 3.2 Add malformed/type/duplicate/bound/issuer/endpoint/grant negatives,
  exact-bound cases, partial-conformance assertions, and diagnostic canaries.
- [x] 3.3 Record the ADR and advance blueprint/canonical `IDR-023` tracking
  without claiming discovery, full RFC 8414 conformance, or engine completion.
- [x] 3.4 Verify consumer HEAD/status/path receipts against preflight.

## 4. Verification and delivery

- [ ] 4.1 Pass focused fmt, all/no-default tests, strict Clippy, and docs.
- [ ] 4.2 Pass factory, workspace, target/MSRV, supply-chain, and full Nix gates.
- [ ] 4.3 Complete a distinct post-implementation review and resolve every
  architecture/API/standards/security/resource finding.
- [ ] 4.4 Produce ready/receipt, synchronize and archive specs, then open the
  signed/DCO issue-linked PR for exact-head hosted CI/review and eligible merge.
