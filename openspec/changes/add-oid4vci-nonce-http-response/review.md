# Semantic contract review

## Scope reviewed

- Issue #137 and exact base
  `38c0fc4313ef735fe564a1cb5080af38f4d2bf14`.
- OpenID4VCI 1.0 Final section 7.2, RFC 9110 field/media grammar and RFC 9111
  Cache-Control/no-store semantics at their recorded immutable hashes.
- Existing request, response core, limits, static errors, dependency boundary,
  target policy and consumer isolation.

## Findings

1. **Resolved — the transport envelope is normative rather than policy.**
   Final requires 2xx, `application/json` and Cache-Control including
   `no-store`; checking those inputs does not select product trust or network
   behavior.
2. **Resolved — request binding does not overclaim provenance.** The method is
   available only on a validated request, but its contract states that the
   caller still proves actual execution, endpoint origin and network policy.
3. **Resolved — retry policy remains external.** Borrowing rather than consuming
   the request permits retry without claiming that replay or remediation is
   safe.
4. **Resolved — headers are bounded and structurally parsed.** Independent
   positive limits stop field amplification. Token, parameter, quoted-string
   and list parsing prevents substring, quoted-delimiter and injection
   confusion without adding an HTTP library.
5. **Resolved — JSON classification is interoperable but narrow.** Type and
   subtype comparison is case-insensitive as RFC 9110 requires. Valid
   parameters do not change `application/json` classification and are ignored;
   multiple or malformed values fail without sniffing.
6. **Resolved — uncacheability requires the exact directive.** Only a bare
   case-insensitive directive named `no-store` passes. Extension names,
   arguments and quoted occurrences cannot impersonate it.
7. **Resolved — validation order protects body diagnostics.** Limits, status,
   media and cache checks precede the existing body parser, so a transport
   failure neither interprets nor exposes a malicious body.
8. **Resolved — optional DPoP remains a separate capability.** Discarding or
   interpreting DPoP here would create proof/lifecycle policy; the adapter
   retains it until an issue-first DPoP contract exists.
9. **Resolved — compatibility and provenance stay bounded.** The API is
   additive and unpublished; no dependency, feature, manifest, target,
   consumer, donor source or fixture changes.

## Decision

The proposal, design, capability requirements, program replacement and task
map are semantically complete, objectively testable, reversible and within the
standing mandate. No correctness, security, privacy, compatibility,
provenance, target or product-scope blocker remains before implementation.
