## 1. Research and specification

- [x] 1.1 Inventory every DID/OID4VCI URI and URI-reference parser, policy wrapper, current test and production/dev dependency edge.
- [x] 1.2 Verify current candidate version, exact published artifact/release provenance, features, MSRV, cone and unsafe/native surface.
- [x] 1.3 Refresh issue #157 and specify accepted/rejected parity, scope exclusions, rollback and stop conditions before implementation.

## 2. Dependency and implementation

- [ ] 2.1 Add exact `fluent-uri 0.4.1` once at workspace level with defaults disabled; record the exact resolved graph, licenses, advisories and target feature surface.
- [ ] 2.2 Replace only `identus-did::Uri` generic grammar mechanics while retaining the 4,096-byte precheck, exact owned string, stable public API and redaction-safe errors.
- [ ] 2.3 Replace all four OID4VCI `uriparse` seams with private absolute/reference candidate parsing while preserving HTTPS, authority, host, userinfo, query, fragment, visible-character, field-bound and error policy.
- [ ] 2.4 Remove the OID4VCI runtime `uriparse` edge and IPvFuture substitution workaround; retain `uriparse 0.6.4` only as DID development evidence.
- [ ] 2.5 Add ADR 0084 and correct living dependency research provenance, cone and disposition.

## 3. Verification and delivery

- [ ] 3.1 Add and pass deterministic accepted/rejected differentials for absolute URI, URI reference, IPvFuture, malformed percent, authority, userinfo, query/fragment, non-ASCII/IRI and exact spelling.
- [ ] 3.2 Pass format, factory, Rust 1.98, no-default-feature, WASM, Android, iOS, dependency/security and complete Nix gates; record exact counts and intentionally unrun checks.
- [ ] 3.3 Perform a distinct exact-diff core/DID/protocol/security review with no unresolved blocker, produce the immutable receipt and archive the change safely.
- [ ] 3.4 Publish an issue-linked PR to `develop`, require all hosted fast/policy/hygiene/DCO/review gates, merge only when green and post issue/parent receipts.
