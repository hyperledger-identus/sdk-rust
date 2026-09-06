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

## Post-implementation exact-diff review

- Reviewed implementation commit
  `c2185bbf53b87aa48395edb0db86e64d66d0d492` and the complete two-commit
  change against exact base
  `82ecd8c5947f064e6bed49e5c3731b6f989659bc`.
- Aggregate size is rejected before parsing or copying. The shared scanner
  accounts for the top-level object and every known, unknown, and nested value
  under caller-selected depth/node budgets, with decoded duplicate names
  rejected at every object boundary.
- Required and optional members enforce JSON type, non-empty value, independent
  decoded-byte limit, and the RFC character grammar. URI references receive a
  second strict syntax check, including malformed percent encoding, without
  resolution or dereference.
- The exact open-registry error code remains available while the separate
  exhaustive enum classifies only the six exact case-sensitive RFC values and
  `Extension`; it adds no retry, blame, remediation, or truth decision.
- Every retained remote string uses zeroizing storage. The response retains
  only its byte count and known fields. Response/code/URI Debug surfaces are
  content-free, and no Clone, Display, Serde, raw JSON, FFI, or network API was
  added.
- Unknown members are fully structurally checked then discarded. The partial
  state cannot imply HTTP validity, request correlation, server identity,
  provenance, trust, safe display, or safe navigation.
- The additive diff is confined to `identus-oid4vci`, tests, and delivery
  evidence. Manifests, lockfiles, features, dependency cones, consumers,
  chains, and product repositories are unchanged.

The review resolved the classification exhaustiveness and malformed-percent
URI coverage before the implementation commit was sealed. No correctness,
security, compatibility, portability, diagnostic, or scope finding remains.
