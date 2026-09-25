# ADR 0146: correlate OID4VCI Authorization Code Token details

- **Status:** Accepted
- **Date:** 2026-09-25
- **Issue:** [#362](https://github.com/hyperledger-identus/sdk-rust/issues/362)
- **Decision authority:** ADR 0105, ADR 0142, ADR 0145 and issue #362

## Context

Issue #360 binds a bounded successful Token Endpoint response to the exact
Credential Issuer, Authorization Server and Credential Configuration selected
for one Authorization Request. The existing response-local Authorization
Details parser validates bounded `openid_credential` entries and Credential
Dataset identifiers, but deliberately has no request lineage. Passing that
detached state onward would permit a response to introduce a configuration
that the wallet did not request or make a caller choose among repeated entries.

The Authorization Request produced by this SDK contains exactly one
`openid_credential` Authorization Details object. OpenID4VCI 1.0 Final requires
the Token Response to return authorized dataset identifiers for the referenced
configuration, while RFC 9396 permits unrelated extension detail types.

## Decision

1. Add a consuming transition only to the request-bound Token Response success
   type; the OAuth error branch cannot enter correlation.
2. Reuse the existing bounded response-local parser and its positive
   `TokenAuthorizationDetailsLimits` rather than adding another JSON or limit
   vocabulary.
3. Reject any recognized entry whose Credential Configuration ID differs from
   the exact request selection. Evaluate this mismatch before cardinality so
   the result is independent of source order.
4. Require exactly one recognized matching entry. Treat repeated matching
   entries as ambiguous; multiple dataset identifiers belong in that entry's
   `credential_identifiers` array.
5. Move the matched source-ordered, unique, zeroizing dataset identifiers into
   a correlated state with the existing Token Response core and exact lineage.
6. Count and ignore bounded unknown authorization-detail types. They confer no
   typed credential authority.
7. Expose borrowed lineage, token core and identifier iteration only. Provide
   no public constructor, raw-parts conversion, Clone, Display or Serde surface.
8. Keep Debug count-only apart from the already-redacted public lineage, and
   append two fieldless static diagnostics for mismatch and ambiguity.
9. Add no dependency, feature, transport, trust, storage, proof, format, chain,
   consumer, release or product behavior.
10. Defer request-bound authorized-dataset Credential Request construction to
    focused issue [#364](https://github.com/hyperledger-identus/sdk-rust/issues/364).

## Consequences

Downstream code can receive dataset authority only when it is cryptographically
untrusted but structurally exact to the SDK's originating request. Failure
consumes and drops the secret-bearing response state, and extensions cannot
expand authority. This transition still does not prove token, issuer, server,
dataset or Credential authenticity, freshness or authorization.

## Reconsideration and rollback

Reconsider the one-entry rule only if a pinned interoperable profile requires
multiple same-configuration objects and defines an unambiguous merge rule.
Rollback removes the additive correlation module, diagnostics, tests and
private move helpers; request-bound HTTP classification and response-local
Authorization Details parsing remain unchanged.
