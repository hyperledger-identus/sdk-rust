# Exact-diff architecture and compatibility review

Review status: completed
Review date: 2026-09-25
Base: develop@e2112c00f98436bacfacff795f0333eb522fb220
Implementation head: 49d3264113612188ae21eceee3275a89fbaf7f38
Reviewed head: 49d3264113612188ae21eceee3275a89fbaf7f38
Specification commit: a7b92d73ca07ea230fc381bcedf60c1be7cb1fad
Preimplementation receipt commit: 7dcc86588ea0ce35047c9b42602e8c91e94b9c96
Unresolved blockers: none

## Scope reviewed

The review inspected the complete base-to-head diff, issue #362 and successor
#364, ADR 0146, OpenID4VCI 1.0 Final sections 3.3.4, 5.1.1 and 6.2, RFC 9396
sections 6 and 7, the immutable preimplementation receipt, parser reuse,
correlation precedence, secret ownership, append-only diagnostics, public API
compatibility and focused/workspace/portable verification.

## Findings

1. **Authority correlation — accepted.** Only the request-bound success owns
   the consuming transition. Every recognized configuration is compared with
   the exact request selection; any mismatch wins before cardinality, so
   response order cannot change the result. Exactly one matching detail
   advances its unique source-ordered identifiers.
2. **Extension behavior — accepted.** The existing response-local parser
   bounds and counts unknown types and fields. Correlation ignores them and
   exposes only the count, so extension data cannot become credential
   authority.
3. **Ownership and privacy — accepted.** The Token Response core and dataset
   identifiers move through zeroizing owners without secret clones. Failure
   consumes and drops the bound success. The correlated Debug view contains
   counts and already-redacted lineage only; both new errors are fieldless and
   static.
4. **Reuse and cohesion — accepted.** The implementation adds one 106-line
   state-transition module and two crate-private move helpers. It reuses the
   existing parser, limits, lineage, token core and public accessors without a
   second representation or dependency.
5. **Compatibility — accepted.** The API is additive and unpublished. Two
   fieldless non-exhaustive variants and stable codes append after every prior
   live row in a focused private catalogue. No existing parser, discriminant,
   code, message, serialized shape, dependency, feature, lockfile, unsafe or
   stored-data contract changes.
6. **Architecture — accepted.** The state proves bounded structural
   correlation only. HTTP provenance, token/dataset/issuer trust, request
   construction, proof generation, storage, format and product policy remain
   outside the slice.
7. **Delivery integrity — accepted after correction.** Exact-diff review found
   that the first implementation commit object had a bad OpenPGP signature.
   It was amended before push; immutable reviewed head `49d3264` and both
   planning commits verify with good signatures and DCO sign-offs.

## Decomposition decision

The exact implementation plan spans 25 paths and 1,031 added text lines, above
the preferred 12-file and 1,000-line guidance. The executable change remains a
single 106-line transition plus 44 lines of private error/decomposition support;
238 lines are focused tests, and the remainder is mandatory OpenSpec research,
specification, ADR, machine inventory and roadmap evidence. Splitting these
artifacts would either expose an undocumented authority transition or advance
the canonical backlog without its implementation. The feature itself already
hands request construction to independent issue #364, so no further functional
decomposition is warranted.

## Residual limitations

- The access token and dataset identifiers remain untrusted opaque inputs.
- Credential Request construction, proof generation and Credential Endpoint
  HTTP handling remain issue #364 or later focused work.
- The caller still owns HTTP origin, TLS, redirects, decompression, timeout,
  cancellation, retry, token validation and secure storage.
- Publication and downstream adoption remain separate release/consumer work.

## Review decision

The implementation is bounded, deterministic, redaction-safe, additive and
reversible. No unresolved correctness, security, privacy, compatibility,
architecture, dependency or delivery finding remains.
