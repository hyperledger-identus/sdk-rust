# Exact-diff architecture and security review

Review status: completed
Review date: 2026-09-28
Base: develop@3512f31a660d536cdee7a585ae95e2c0a6129790
Implementation head: 51254ef27f9bb81292a96f5e776f4efb31a84eee
Reviewed head: 51254ef27f9bb81292a96f5e776f4efb31a84eee
Specification commit: e2479dbfc481e442d71108a85b26bfdd7a75f3ea
Unresolved blockers: none

## Scope reviewed

The review inspected the complete base-to-head diff, issue #396, ADR 0158,
OpenID4VP 1.0 Final Request URI/JAR requirements, RFC 9101, the immutable
preimplementation receipt, the public state transitions, resource and error
precedence, dependency direction, portability, and all focused/workspace
evidence.

## Findings

1. **Protocol envelope — accepted.** Omitted and explicit `get` plus explicit
   `post` produce exact runtime-neutral request data. A response must be 2xx,
   exactly typed, non-empty, bounded, and compact before it can become an
   explicitly unverified Request Object.
2. **Cryptographic state — accepted.** Protected `typ` is exact, the caller
   supplies an algorithm-bound key and suite registry, and `identus-jose`
   verifies the signature before payload claims are parsed or exposed. The
   result proves signature and correlation only, not key authorization or
   verifier trust.
3. **Correlation and ambiguity — accepted.** The complete signed payload is
   scanned under depth, node, member, and decoded-string limits. Duplicate
   object names fail closed; signed `client_id` and a sent `wallet_nonce` must
   match their outer values byte-for-byte.
4. **Security and privacy — accepted.** HTTP execution, DNS, TLS, redirects,
   decompression, retry, timeout, and SSRF policy stay with the adapter. Retained
   endpoint, compact object, payload, identifiers, and nonce values are
   zeroizing and require explicit sensitive accessors. Debug and error output
   remains static and redacted.
5. **Architecture and dependencies — accepted.** The crate reuses the existing
   JOSE owner and adds no third-party package. The exact internal cone is
   `identus-core` plus `identus-jose`; no runtime or transport dependency is
   introduced.
6. **Maintainability — accepted after correction.** Initial exact-head analysis
   found new complexity signals in the private JSON scanner and expanded error
   projection, plus an inherited signal in the touched ingress parser. The
   review decomposed all three without changing behavior. The final code-health
   report has no OID4VP function or module above the repository thresholds and
   production paths contain no panic/unwrap fallback.
7. **Compatibility — accepted.** The crate remains unpublished `0.0.0`; changes
   are additive except the standards-correct acceptance of explicit lowercase
   `request_uri_method=get`. Existing error fixture entries retain their order,
   and new stable variants append after them.

## Residual limitations

- Compact JWE and JWS JSON serialization are not supported.
- Client-prefix/key authorization, DID/X.509/federation/attestation trust,
  audience, time, replay, complete OAuth request semantics, DCQL, consent,
  response construction, and credential processing remain later transitions.
- The transport adapter must enforce its own network security policy before
  executing the exposed HTTPS request.

## Review decision

The implementation is a cohesive, bounded and runtime-neutral transition from
a validated reference to a cryptographically verified JAR envelope. No
unresolved correctness, security, privacy, compatibility, architecture,
dependency, portability, maintainability, or delivery finding remains.
