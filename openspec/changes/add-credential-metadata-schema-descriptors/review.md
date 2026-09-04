# Pre-implementation semantic, API, security, and performance review

- **Date:** 2026-09-05
- **Issue:** #75, child of #6 / `IDR-007` and #20
- **Develop base:** `9aa33fcf8d2a6755270e46e435beae25f9ca75cf`
- **Result:** contract is implementable with no unresolved blocker

## Findings

1. W3C VC Data Model 2.0 is a wire/profile model, not the SDK wire contract.
   Requiring every generic identifier to be a W3C URL would exclude Midnight
   family IDs and Lace schema IDs; format adapters must enforce narrower rules.
2. W3C permits multiple and unidentified subjects. The generic metadata model
   therefore uses a bounded zero-or-more subject list rather than one required
   DID copied from Oxid.
3. Display names, descriptions, logos, language, and direction are important
   but belong to a dedicated internationalized rendering/profile contract.
   A single Oxid `display_name` or Lace JSON display blob would prematurely
   freeze an English/product-oriented shape.
4. Claim values and commitment openings are private credential contents, not
   metadata. Claim descriptors carry only identifiers, paths, disclosure
   capability, required state, and an optional type hint.
5. Midnight claim paths expressed as segments are more format-neutral than
   Oxid's delimiter-bearing string. Keeping segments also removes JSON Pointer
   escape ambiguity from the generic core.
6. Schema version is optional and structurally bounded, not mandatory SemVer.
   W3C schema references are unversioned while Lace currently uses `1.0` and
   Midnight expects SemVer; adapters retain those profile validations.
7. Metadata must remain separate from the opaque envelope. Joining arbitrary
   bytes with caller-supplied descriptors would look like a parsed or verified
   association that the existing envelope explicitly does not establish.
8. Pairwise duplicate scans are appropriate only after hard collection bounds.
   With at most 64 claims and 16 entries elsewhere, they avoid temporary
   allocations while keeping worst-case work deterministic.
9. Issuer and subject identifiers can correlate people and organizations.
   Custom Debug plus static errors must prevent them entering logs through the
   generic domain surface.
10. Apollo and NeoPRISM supply no competing credential descriptor model at the
    inspected revisions. No artificial dependency or placeholder API should be
    added for them.
11. Donor repositories are evidence only. No source copy, build, branch switch,
    dependency repoint, generated artifact, or consumer mutation is necessary.

Verdict: READY to implement after strict structural validation.
