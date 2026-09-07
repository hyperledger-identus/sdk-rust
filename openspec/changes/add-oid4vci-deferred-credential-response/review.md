# Semantic review

- **Reviewer:** Codex fresh contract pass
- **Date:** 2026-09-07
- **Scope:** issue #149, OpenID4VCI Final section 8.3, proposal, design,
  capability delta, and complete IDR-023 replacement
- **Result:** no unresolved blocker before implementation

## Findings

1. **Cleared — `number` could be narrowed silently to an integer or float.**
   The contract accepts all mathematically positive JSON-number forms and
   retains the bounded exact lexeme without conversion, rounding, or overflow.
2. **Cleared — syntax could acquire scheduling policy.** The core exposes an
   exact interval value only and makes no duration, cap, clock, retry, backoff,
   or wake-up decision.
3. **Cleared — the transaction handle could be treated as public metadata.**
   It is zeroizing, Debug-redacted, and available only through an explicitly
   sensitive accessor; provenance, freshness, and authorization are disclaimed.
4. **Cleared — immediate and deferred members could coexist ambiguously.** Both
   deferred members are mandatory while `credentials` and `notification_id`
   are explicit branch violations; the existing immediate parser is unchanged.
5. **Cleared — extension traversal could bypass resource limits.** Complete
   bytes, depth, aggregate nodes, top-level members, retained identifier, and
   interval lexeme each have positive independent bounds; duplicate decoded
   names fail before projection.
6. **Cleared — a body type could overclaim HTTP or transaction truth.** The API
   carries no status, media, request, endpoint, transport, or correlation state.
7. **Cleared — downstream behavior could become normative.** The pinned Final
   controls semantics; Oxid and Portal remain read-only and no code or fixture
   is copied.

## Gate decision

The change is additive, reversible, issue-linked, chain-neutral, and within the
standing IDR-023 mandate. No unresolved correctness, privacy, compatibility,
provenance, or product-scope blocker remains. Implementation may proceed.
