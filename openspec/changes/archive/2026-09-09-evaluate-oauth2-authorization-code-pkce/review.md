# Exact-diff protocol, architecture and security review

Review status: completed
Review date: 2026-09-09
Implementation head: 2acb814d6c3a2c44473595e39d9174f3fb573a06
Specification commit: db2d36b88edee2f8d02ddbc24d082586defd337c
Unresolved blockers: none

## Scope reviewed

The review inspected the complete
`develop@4c22c1ac059be5c5113301b205f1b7553c89e4b9...2acb814d` diff,
issue #160, ADR 0102, exact candidate lock, public-API consumer fixture,
isolation script, dependency ledger and local verification receipts.

## Findings

1. **Protocol authority — accepted.** OID4VCI 1.0 Final, RFC 6749, RFC 7636
   and RFC 9700 remain normative. Candidate results are explicitly oracle
   evidence and cannot redefine the SDK profile.
2. **Architecture and cohesion — accepted.** The fixture is an unpublished
   nested workspace. No oauth2, URL, HTTP, chrono or candidate error type
   enters `identus-oid4vci`, the root graph or a public Identus facade.
3. **Positive mechanic evidence — accepted.** Caller-owned state, verifier and
   authorization code produce the exact RFC challenge, authorization URL and
   token form without choosing network, browser, clock, RNG or runtime
   authority.
4. **PKCE safety mismatch — accepted as rejection evidence.** The candidate
   accepts invalid verifier characters and panics for a 42-byte verifier.
   Production use is prohibited until a bounded non-panicking facade or a
   narrower released implementation exists.
5. **Endpoint and extension mismatch — accepted as rejection evidence.** URL
   syntax accepts HTTP/userinfo/fragment input, scopes accept spaces, and extra
   parameters can duplicate managed keys. The fixture preserves these facts
   rather than normalizing them away.
6. **Response and secret boundary — accepted as rejection evidence.** Secret
   Debug is redacted, but parse failures retain the full raw body and unknown
   one-megabyte OID4VC fields are ignored. The response parser is explicitly
   `not-adopt`.
7. **Dependency and supply chain — accepted.** The exact no-default candidate
   remains a broad 68/77-line cone with unconditional clock/RNG/URL/HTTP/JSON
   families. Immutable source, checksum, license, MSRV, maintenance, deny and
   audit evidence are recorded.
8. **Isolation gate — resolved.** The initial script compared root manifests
   with `origin/develop`, which would become stale and depend on a remote ref.
   It now directly fails if the candidate enters the root manifest or lock and
   asserts the observed cone size.
9. **Formatting gate — resolved.** Full Nix validation found the new manifest
   was not Taplo-formatted. The manifest was formatted and all 31 compatible
   checks passed on the complete rerun.
10. **Delivery scope — accepted.** No production code, fast/slow workflow,
    support policy, donor or downstream repository is changed.

## Residual limitations

- Target results are compile-only; no browser or mobile runtime was exercised.
- The oracle does not prove authorization response correlation, PAR, DPoP,
  RAR, redirects, persistence or network interoperability.
- Reconsideration requires a named authorization engine with bounded owned
  inputs, errors and ports; passing one OAuth mechanic does not admit the
  complete client.

## Review decision

The research fixture is deterministic, isolated, reversible and sufficient to
decide oauth2 5.0.0 as an oracle rather than a production dependency. No
unresolved protocol, correctness, architecture, security, privacy, dependency,
licensing or delivery finding remains for hosted review.
