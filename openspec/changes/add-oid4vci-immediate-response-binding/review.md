# Semantic review

- **Reviewer:** Codex fresh contract pass
- **Date:** 2026-09-07
- **Scope:** issue #143, OpenID4VCI Final section 8.3, RFC 9110 media
  grammar, proposal, design, capability delta and complete IDR-023 replacement
- **Result:** no unresolved blocker before implementation

## Findings

1. **Cleared — proof count is not necessarily distinct key count.** The state
   enforces only the necessary response-count upper bound available from the
   current opaque proofs and explicitly disclaims unique-key or credential-key
   correlation.
2. **Cleared — non-normative examples must not create Cache-Control policy.**
   Final requires JSON media for unencrypted responses but does not require a
   cache field in section 8.3. The API omits it and leaves stricter product
   policy downstream.
3. **Cleared — immediate and deferred status must remain distinguishable.**
   Exact 200 is accepted, 202 returns the existing unsupported-deferred error
   before body parsing, and other status values fail with a static HTTP error.
4. **Cleared — private grammar reuse could regress Nonce validation.** The
   reviewed helper moves without semantic edits and both the existing Nonce
   suite and new Credential suite are required gates.
5. **Cleared — a request method could overclaim transport provenance.** The
   method accepts caller-supplied effective values, retains none of them, and
   documents that endpoint origin, execution, TLS, DPoP, replay and single-use
   policy remain external.
6. **Cleared — invalid envelopes should not inspect credential content.**
   Status and bounded media validation precede body parsing; errors remain
   fieldless and canary tests cover validation order and diagnostics.
7. **Cleared — legacy Portal wire shape conflicts with Final.** It remains
   incompatible evidence and is neither copied nor accepted. Final normative
   text and the Oxid array shape control this slice.

## Gate decision

The change is additive, reversible, issue-linked, chain-neutral and within the
standing IDR-023 mandate. No protected product, governance, release, licensing,
security-disclosure, repository-administration or downstream-write boundary is
crossed. Implementation may proceed.
