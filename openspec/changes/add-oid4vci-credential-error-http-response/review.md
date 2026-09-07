# Semantic review

- **Reviewer:** Codex fresh contract pass
- **Date:** 2026-09-07
- **Scope:** issue #147, OpenID4VCI Final sections 8.3.1.1 and 8.3.1.2,
  proposal, design, capability delta, and complete IDR-023 replacement
- **Result:** no unresolved blocker before implementation

## Findings

1. **Cleared — payload and authorization errors could be conflated.** The new
   boundary accepts only the section 8.3.1.2 status/media/body contract,
   rejects its explicitly displaced generic `invalid_request`, and leaves RFC
   6750 responses and authentication challenges to a separate capability.
2. **Cleared — a non-normative example could become a false header mandate.**
   Cache-Control is neither accepted nor required because `no-store` appears
   only in the example, unlike the normative Credential Nonce response rule.
3. **Cleared — a method on request state could overclaim correlation.** The
   associated parser returns only the bounded body core and does not accept or
   retain request state, endpoint identity, status, or header data.
4. **Cleared — HTTP composition could weaken parser bounds.** Status is checked
   before remote fields, media length/grammar before body, and the existing
   complete body/depth/node/value limits remain authoritative.
5. **Cleared — strict known-code validation would break extensions.** Only the
   exact generic value forbidden by the Final payload branch is rejected; all
   other valid unknown codes remain exact `Extension` values without policy.
6. **Cleared — remote content could escape through diagnostics.** New errors
   are fieldless/static, envelope values are discarded, and the existing
   zeroizing/redacted body state is returned unchanged.
7. **Cleared — consumer precedent could become normative.** The pinned Final
   text controls behavior; Oxid and Portal remain read-only evidence and no
   source or fixture is copied.

## Gate decision

The change is additive, reversible, issue-linked, chain-neutral, and within the
standing IDR-023 mandate. No protected product, governance, release, licensing,
security-disclosure, repository-administration, or downstream-write boundary
is crossed. Implementation may proceed.
