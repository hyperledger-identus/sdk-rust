# Semantic review

- **Reviewer:** Codex fresh contract pass
- **Date:** 2026-09-07
- **Scope:** issue #141, OpenID4VCI Final section 8.3, proposal, design,
  capability delta and complete IDR-023 replacement
- **Result:** no unresolved blocker before implementation

## Findings

1. **Cleared — immediate/deferred validity was initially easy to conflate.**
   The public type is explicitly immediate-only and a `transaction_id` returns
   an unsupported-deferred error. The contract does not claim to parse every
   successful Final response.
2. **Cleared — opaque credentials need usefulness without format ownership.**
   Exact JSON is retained for strings and objects, representation kind is
   explicit, and string values additionally expose the decoded semantic value.
   No base64url or credential-format rule enters this crate.
3. **Cleared — extension tolerance can create resource and smuggling risk.**
   Unknown values are accepted only under full JSON byte/depth/node limits;
   explicit object-member caps bound name tracking, and duplicate names fail.
4. **Cleared — parsing could be mistaken for trust or transport evidence.**
   The type is a body core only, documentation names the missing HTTP,
   provenance, verification, correlation, notification and storage layers, and
   diagnostics retain no input.
5. **Cleared — legacy Portal wire shape conflicts with Final.**
   It is recorded as incompatible evidence and is neither copied nor accepted.
   Final normative text and Oxid's array shape control this slice.

## Gate decision

The change is additive, reversible, issue-linked, chain-neutral, and within the
standing IDR-023 mandate. No protected product, governance, release, licensing,
security-disclosure, repository-administration or downstream-write boundary is
crossed. Implementation may proceed.
