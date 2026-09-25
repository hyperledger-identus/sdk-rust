# Exact-diff architecture and compatibility review

Review status: completed
Review date: 2026-09-25
Base: develop@342d936107624aee95788f1788b8cee6d62641c8
Implementation head: da0c7ed9e64c6e3de5333dbb355ecb2449872d84
Reviewed head: da0c7ed9e64c6e3de5333dbb355ecb2449872d84
Specification commit: 2b70661b1dad631cc6a3b279ea4b307be03eb045
Preimplementation receipt commit: f2cee2de981fe6413a83c62b59d5041a02f48f5e
Unresolved blockers: none

## Scope reviewed

The review inspected the complete base-to-head diff, issues #368 and #370,
ADR 0149, OpenID4VCI 1.0 Final sections 9, 9.1 and 12.2.4, the immutable
preimplementation receipt, authority ownership, secret lifetime, serializer
reuse, error precedence, redaction, compatibility, and focused/workspace/
portable verification.

## Findings

1. **Minimal authority — accepted.** The request retains only the exact
   validated issuer, optional advertised Deferred Credential Endpoint, existing
   zeroizing Authorization and proof count. It does not retain full metadata,
   Token Response, refresh token, scope, offer, proof JWTs or authorization
   lineage.
2. **Branch-specific secret lifetime — accepted.** Status is selected first.
   HTTP 200, 400 and unsupported statuses drop the complete request before
   remote parsing. HTTP 202 moves only the minimal continuation capability and
   drops the proof body and obsolete Credential Endpoint before media/body
   parsing; every error then drops the retained capability.
3. **No substitution — accepted.** The consuming bound-response method accepts
   only `DeferredCredentialRequestLimits`. Its issuer, endpoint, Authorization,
   transaction and proof count all derive from owned predecessor state and
   cannot be replaced through the public API.
4. **Reuse and cohesion — accepted.** One private constructor and the existing
   bounded serializer serve both the legacy structural request and new bound
   request. JSON encoding, complete-body limits and static errors are not
   forked.
5. **Privacy — accepted.** Bearer and request body remain zeroizing; the new
   states are non-Clone and expose sensitive values only through deliberately
   named accessors. Debug and every tested error bridge omit issuer, endpoint,
   bearer, proof, transaction and response values.
6. **Compatibility — accepted.** The public API is additive and unpublished.
   Existing constructors, borrowed validators, request wire bytes, errors and
   immutable error golden remain unchanged. No manifest, lockfile, feature,
   dependency, unsafe/native or stored-data contract changed.
7. **Architecture — accepted.** The type proves structural authority continuity
   only. It performs no HTTP and claims no origin, TLS, token validity, issuer
   control, retry, polling, credential verification/storage, trust or product
   policy. Focused issue #370 owns consuming Deferred Endpoint response binding.
8. **Delivery integrity — accepted.** All three branch commits have valid
   OpenPGP signatures and DCO sign-offs. The canonical backlog points to open
   successor #370 and passes its live GitHub-state audit.

## Decomposition decision

The exact change spans 23 paths and 850 changed text lines, exceeding the
preferred 12-file guidance but remaining below the 1,000-line guidance. The
shipping implementation is 270 changed lines across five cohesive source files
and 118 changed test lines in one consumer-shaped test; 462 changed lines are
mandatory OpenSpec planning, ADR, predecessor amendment, inventory, roadmap,
review and verification evidence. Splitting the retained fields, consuming
transition or shared serializer would detach one secret-lifetime decision from
its type-system enforcement. Deferred response binding is already split to
#370, so no further functional decomposition is warranted.

## Residual limitations

- HTTP origin, TLS, redirects, decompression, timeout and cancellation remain
  outer transport obligations.
- The retained access token remains opaque and unvalidated; the issuer,
  endpoint and transaction remain structurally validated rather than trusted.
- Interval scheduling, retry, replay, refresh and terminal-use policy remain
  outside this construction slice.
- Encrypted responses and RFC 6750 authorization-error responses remain outside
  the current unencrypted profile.
- Publication and downstream adoption remain separate release/consumer work.

## Review decision

The implementation is bounded, cohesive, one-shot, least-authority,
redaction-safe, additive and reversible. No unresolved correctness, security,
privacy, compatibility, architecture, dependency or delivery finding remains.
