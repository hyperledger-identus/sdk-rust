# ADR 0046: bind a Pre-Authorized Code server before request construction

- **Status:** Accepted for `develop`
- **Date:** 2026-09-07
- **Related:** issues #7, #20, #121; ADRs 0041–0045

## Context

The OID4VCI crate separately validates an offered Pre-Authorized Code grant,
Credential Issuer Metadata, and a partial Authorization Server Metadata core.
A wallet still needs to establish that one chosen server is permitted by the
issuer and offer, advertises the offered grant, and supplies the endpoint that
a later Token Request would target.

Leaving this agreement to each HTTP adapter would duplicate security-sensitive
mix-up and capability checks. Letting the SDK choose a server would instead
embed discovery, ranking, trust, and fallback policy that belongs to the
application.

## Decision

Add a consuming `CredentialOfferWithPreAuthorizedServer` stage in
`identus-oid4vci`. The caller presents one already validated Authorization
Server Metadata core. The transition succeeds only when:

1. the offer contains the Final Pre-Authorized Code grant;
2. the selected issuer exactly matches one effective Authorization Server from
   Credential Issuer Metadata, including its single-server issuer default;
3. any Pre-Authorized Code `authorization_server` hint exactly matches it;
4. its effective grant set contains the exact Pre-Authorized Code grant
   identifier; and
5. a validated Token Endpoint is present.

RFC 8414's omitted grant-list default contains only `authorization_code` and
`implicit`, so omitted metadata does not satisfy the fourth invariant. The
anonymous Pre-Authorized Code flag remains accessible but does not change the
binding result. The success state owns and exposes both predecessor states.

Success proves only these cross-document relationships. It does not prove
retrieval provenance, trust, endpoint reachability, client eligibility,
Transaction Code satisfaction, request readiness, replay safety, or issuance.
The transition performs no parsing or allocation and adds no dependency.

## Consequences

- A later Token Request builder can require one typed prerequisite instead of
  reimplementing server mix-up, grant-advertisement, and endpoint checks.
- Server choice stays explicit and caller-owned; no ranking, substitution, or
  fallback policy enters the generic SDK.
- Each failed invariant has a stable, static, redaction-safe error code.
- Existing offer, issuer-metadata, and Authorization Server Metadata APIs and
  their target/dependency contracts remain unchanged.

## Alternatives rejected

- **Build the Token Request immediately:** rejected because it would conflate
  server agreement with secret-bearing message and client-authentication
  policy.
- **Select the first advertised server automatically:** rejected because list
  order is not a trust or preference policy and a grant hint may constrain the
  exact server.
- **Treat omitted grants as permissive:** rejected because RFC 8414 defines a
  concrete default that excludes the Pre-Authorized Code grant.
- **Require anonymous access:** rejected because client identification and
  authentication are later policy/protocol concerns, not server eligibility.
