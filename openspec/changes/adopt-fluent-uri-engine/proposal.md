## Why

The SDK currently maintains one generic RFC 3986 parser in `identus-did` and
uses `uriparse 0.6.4` at four OID4VCI validation seams. The latter rejects the
RFC-defined IPvFuture host production, requiring a security-sensitive local
substitution workaround. `fluent-uri 0.4.1` provides strict borrowed URI and
URI-reference parsing, including IPvFuture, and can replace meaningful local
mechanics without changing public types or normalization policy.

The preliminary portfolio cited a post-release repository revision. This
change corrects provenance to the exact published artifact and release commit,
then requires an accepted/rejected parity matrix before adoption.

## What changes

- Add exact `fluent-uri 0.4.1` as a private workspace dependency with default
  features disabled and no optional features unless implementation evidence
  proves one is necessary.
- Back `identus-did::Uri` syntax validation with borrowed `fluent_uri::Uri`
  parsing after the existing 4,096-byte bound.
- Replace OID4VCI `uriparse` URI and URI-reference calls with the candidate,
  while retaining each wrapper's HTTPS, host, userinfo, query, fragment,
  visible-character, byte-limit and redaction policy.
- Remove the IPvFuture substitution workaround and the OID4VCI runtime
  `uriparse` dependency. Keep `uriparse 0.6.4` only as the existing DID
  development oracle.
- Add ADR 0084, exact release provenance, differential negative regressions,
  dependency/unsafe evidence and rollback documentation.

## Capabilities

### Modified capabilities

- `did-core`: replace only generic `Uri` grammar mechanics; keep the public
  value, resource policy, exact spelling and DID/DID-URL parsers unchanged.

### New capabilities

- `oid4vci-uri-validation`: define private grammar-engine use beneath current
  protocol-specific absolute URI and URI-reference policies.

## Non-goals

- No change to `identus-core::Url`, whose narrower `scheme://authority`
  acceptance contract has no current external consumer.
- No IRI, normalization, equivalence, resolution, DNS, SSRF, HTTP-client or
  trust commitment.
- No replacement of the DID or DID URL method-specific lexical parsers.
- No public third-party types, downstream repository changes or release claim.

## Delivery

Issue #157 owns this change under parent #151. Specification and research land
in a signed commit before Cargo or Rust implementation. A distinct exact-diff
architecture/security review, full local gates, hosted Linux `fast`, DCO,
policy and review are required before merge to `develop`.
