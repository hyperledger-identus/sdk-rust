# Semantic contract review

## Scope reviewed

- Issue #129 and exact base
  `82ecd8c5947f064e6bed49e5c3731b6f989659bc`.
- OpenID4VCI 1.0 Final Token Error Response clarifications.
- RFC 6749 Token Error Response fields, codes, extension rules, and grammars.
- Existing OID4VCI strict scanner, limits, errors, redaction, and target policy.
- Read-only Oxid and Lace evidence listed in issue #129.

## Findings

1. **Resolved — standard codes must not close the OAuth registry.** Exact
   bounded code strings are retained, while a separate closed classification
   distinguishes the six RFC values from extensions.
2. **Resolved — remote descriptions are not wallet UI copy.** The optional
   description is validated and zeroized but exposed only through an explicitly
   untrusted accessor with no localization or user-safety claim.
3. **Resolved — valid URI-reference does not imply safe navigation.** Relative
   references remain interoperable; the validated wrapper offers no network or
   resolution operation and documents SSRF/navigation as consumer policy.
4. **Resolved — unknown members cannot become a compatibility rejection.** All
   values remain structurally bounded and duplicate-checked, then unknown
   members are discarded without entering the semantic or diagnostic surface.
5. **Resolved — HTTP semantics stay outside JSON parsing.** The partial state
   does not claim status, content type, cache headers, authentication challenge,
   endpoint provenance, or request correlation.
6. **Resolved — attacker-controlled diagnostics remain data-free.** Retained
   strings zeroize, Debug exposes only classification/presence/length, and new
   errors are static and fieldless.

## Decision

The proposal, design, capability requirements, program replacement, and task
map are semantically complete, objectively testable, reversible, and within
the standing mandate. No unresolved blocker remains before implementation.
