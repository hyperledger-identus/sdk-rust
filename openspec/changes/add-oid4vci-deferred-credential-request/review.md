# Exact-diff architecture and security review

Review status: completed
Review date: 2026-09-09
Base: develop@49c5288713f9979747c56f0307cf6ed12c785422
Implementation head: d60ffc7a8ecadc526a639249ddcfe1057c2683ad
Reviewed head: 37c70e9935457eef0da6c31cd1b99ad75b44e06a
Specification commit: 6930bba9a32a6b4da85ad05f3d0166e580793013
Unresolved blockers: none

## Scope reviewed

The review inspected the complete base-to-reviewed-head diff, issue #250,
OpenID4VCI 1.0 Final section 9.1, ADR 0109, public API, zeroizing body storage,
bounded writer, error mapping, ten focused tests, inventories and verification.

## Findings

1. **Normative shape — accepted.** The request contains exactly one required
   `transaction_id`, reports POST and `application/json`, and uses the existing
   typed advertised endpoint.
2. **Limit composition — resolved.** The initial tests proved an exact custom
   body ceiling but did not explicitly prove that the 16 KiB default covers the
   worst-case escaped 2,048-byte default transaction or that a larger custom
   response policy remains independently constrained. Commit `37c70e9` adds
   both cases; they pass.
3. **Serialization — accepted.** `serde_json` owns scalar escaping while a
   private writer checks arithmetic and the complete body ceiling before every
   retained write. Failure drops and zeroizes partial output.
4. **Sensitive-state boundary — accepted.** The body is zeroizing bytes with
   one explicitly sensitive accessor. Request, endpoint, response, transaction
   and errors do not expose canary content through diagnostics.
5. **Authority separation — accepted.** The value reports that a token is
   required but owns no token and performs no HTTP, TLS, timing, replay,
   invalidation, encryption, correlation, storage or trust operation.
6. **Compatibility — accepted.** Existing response and metadata behavior is
   unchanged; the new request, limits, constants, method and errors are
   additive and unpublished.
7. **Architecture and supply chain — accepted.** The implementation reuses
   existing `serde_json` and `zeroize`; no manifest, lockfile, dependency,
   feature, unsafe, target, donor or downstream change exists.
8. **Roadmap evidence — accepted.** IDR-023 advances from completed child #241
   to active #250 while remaining `in_progress`; no engine-completion or
   product-support claim is made.

## Residual limitations

- Typed construction does not prove endpoint provenance, issuer control,
  reachability, TLS/redirect policy, token or transaction validity.
- Interval scheduling, retry/replay/invalidation, encryption, HTTP execution,
  error response handling and response correlation remain absent.
- Portable compilation is not runtime, consumer, release or certification
  evidence.

## Review decision

The slice is cohesive, additive, independently bounded, redaction-safe and
reversible. No unresolved correctness, security, privacy, compatibility,
architecture, dependency or delivery finding remains before hosted review.
