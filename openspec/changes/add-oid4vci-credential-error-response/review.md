# Semantic review

- **Reviewer:** Codex fresh contract pass
- **Date:** 2026-09-07
- **Scope:** issue #145, OpenID4VCI Final section 8.3.1.2, proposal,
  design, capability delta and complete IDR-023 replacement
- **Result:** no unresolved blocker before implementation

## Findings

1. **Cleared — the Final's named codes are not a closed wire registry.** The
   specification says implementations SHOULD use the seven values, so the
   contract preserves every valid exact code and classifies unknown values as
   `Extension` rather than rejecting future interoperability.
2. **Cleared — Credential and Token Endpoint errors must not share a public
   type.** Their known registries and members differ. The implementation reuses
   only private bounded JSON machinery and exposes a distinct credential
   response surface without `error_uri`.
3. **Cleared — superseded draft nonce fields could regain authority.**
   `c_nonce`, `error_uri`, and other foreign fields are bounded unknown
   extensions, duplicate checked and discarded without semantic access.
4. **Cleared — remote descriptions are unsafe wallet copy.** The optional
   value is strict NQSCHAR-compatible input, stored in zeroizing memory, exposed
   only through an explicitly untrusted accessor, and absent from diagnostics.
5. **Cleared — body syntax could be mistaken for transport truth.** The type
   and requirements disclaim HTTP status/media/authentication, origin,
   correlation, issuer truth, retryability, blame, remediation, and UI policy.
6. **Cleared — hostile extensions can bypass known-field limits.** Complete
   bytes, depth, aggregate nodes, and duplicate decoded names remain bounded
   for every known and unknown value before unknown content is discarded.
7. **Cleared — consumer code is not normative.** Oxid and Portal supply only
   read-only architectural evidence; no source or fixture is copied. The Final
   text controls code grammar and semantics.

## Gate decision

The change is additive, reversible, issue-linked, chain-neutral and within the
standing IDR-023 mandate. No protected product, governance, release,
licensing, security-disclosure, repository-administration or downstream-write
boundary is crossed. Implementation may proceed.
