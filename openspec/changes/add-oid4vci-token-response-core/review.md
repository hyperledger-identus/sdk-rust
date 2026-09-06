# Semantic contract review

## Scope reviewed

- Issue #127 and exact base
  `39d8446aef19130e32a235c9f6be8734c40711ed`.
- OpenID4VCI 1.0 Final successful Token Response and token-protection rules.
- RFC 6749 successful response, unknown-member, syntax, and no-store rules.
- Existing OID4VCI strict scanner, limits, errors, redaction, and target policy.
- Read-only Oxid and Lace evidence listed in issue #127.

## Findings

1. **Resolved — do not silently lose recognized Authorization Details.** The
   exact response remains privately available for a later transition and the
   partial state exposes presence only; it cannot construct a Credential
   Request or claim Credential Dataset validation.
2. **Resolved — unknown members cannot become a compatibility rejection.** All
   values remain structurally bounded and duplicate-checked, but unknown names,
   including historical nonce members, are ignored semantically.
3. **Resolved — standards syntax must not become product policy.** The contract
   validates RFC grammars and preserves exact token type, while making no
   Bearer/DPoP, trust, refresh, or scope decision.
4. **Resolved — token sizes are undefined by OAuth.** Independent positive
   caller limits supplement the aggregate JSON bound and every oversize path is
   static and testable.
5. **Resolved — the raw response is itself sensitive.** The internal copy and
   extracted fields zeroize; only explicitly named sensitive accessors reveal
   tokens or scope, and Debug/errors remain data-free.
6. **Resolved — transport claims stay outside parsing.** Successful core syntax
   does not prove HTTP status, headers, endpoint provenance, server acceptance,
   expiration, replay safety, or cryptographic validity.

## Decision

The proposal, design, capability requirements, program replacement, and task
map are semantically complete, objectively testable, reversible, and within
the standing mandate. No unresolved blocker remains before implementation.
