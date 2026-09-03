# Pre-implementation semantic and misuse-resistance review

- **Date:** 2026-09-03
- **Issue:** #47 (child of #5 / `IDR-006`)
- **Develop base:** `558677ef0359f9af241232ed13422a005d5cc5b0`
- **Reviewed contract:** OpenSpec `add-did-registration-lifecycle` and ADR 0015
- **Result:** no unresolved blocker

## Findings

1. **Draft stability:** DIF DID Registration is unratified and its job prose
   conflicts with examples. Pin the commit, prefer the coherent invariant, and
   avoid a direct wire promise.
2. **Secret safety:** the draft's raw `secret` maps and `returnSecrets` cannot
   enter the generic SDK. Opaque custody handles and public action data preserve
   implementability without moving private bytes.
3. **Destructive default:** internal generation with neither storage nor a
   returned handle can permanently lose control. Reject that combination.
4. **Method routing:** continuations need an explicit validated method outside
   the opaque job token so immutable registry dispatch never parses or guesses.
5. **Action replay:** action response ids must be bound into the job and checked
   before dispatch; otherwise stale or cross-job signatures can be injected.
6. **Idempotency:** the port can require immutable equality-comparable requests
   and keys but cannot provide durable replay storage. Enforcement is an
   adapter obligation and conflicts must be explicit.
7. **Cancellation:** cancelling observation is not cancelling ledger work. An
   explicit best-effort request preserves truthful partial/final outcomes.
8. **Patch semantics:** complete replace is generic; add/remove and
   method-specific payloads can be bounded and ordered but are interpreted only
   by the method adapter. The core must not resolve/diff/reorder them.
9. **Public open data:** method extensions are necessary, but all such maps need
   shared resource limits and recursive private-material rejection. Debug must
   redact the entire content rather than rely on field-name filtering.
10. **Cache boundary:** update/deactivate results expose the DID, but automatic
    invalidation remains outer so partial visibility and consistency policy are
    not hidden.
11. **Consumer proof:** PRISM and Midnight differ materially in operation
    encoding, custody, and finality. In-memory mocks can prove the generic seam;
    production downstream edits are neither required nor authorized.
12. **Scope:** redirect callback integrity, decryption plaintext, HTTP, DIF
    execute/resource operations, persistence, fees, finality policy, wallet
    consent, and compensation require independent contracts.

# Post-implementation semantic, security and API review

Pending implementation and exact-head verification.
