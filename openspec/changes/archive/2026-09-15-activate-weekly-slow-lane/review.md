# Review

## Scope and identities

- Planning head: `a777b0f91882dba72df85d568468a1b5d19817cc`
- First implementation head: `4d4ff3dea2bf42e570d2746af04bb35c62632360`
- Pull request: [#288](https://github.com/hyperledger-identus/sdk-rust/pull/288)
- Review lenses: scheduling architecture, least privilege, failure recovery,
  evidence retention, exact-revision binding and developer-facing truth

## Findings

1. **Resolved P2 — freshness exceeded retention.** The first implementation
   accepted a scheduled run for 192 hours, while GitHub retains its logs and
   artifacts for only seven days. A missed-run audit could therefore pass after
   the evidence receipt disappeared. The policy now fails after 160 hours,
   leaving eight hours before retention expiry, with a 161-hour regression.
2. **Resolved — target planner fixture remained external-only.** The machine
   plan changed to native weekly/manual operation but one Node regression still
   expected the superseded state. The assertion now binds the accepted value.
3. **Resolved — receipt summary triggered shell diagnostics.** Deterministic
   Python rendering replaced shell quoting and repeated redirects.
4. **Resolved — host Python compatibility.** The audit uses `timezone.utc`
   rather than the Python 3.11-only `datetime.UTC` name.
5. **No blocker — manual versus natural evidence.** The ADR, capability and
   issue retain the distinction; the activation PR cannot close #276.
6. **Resolved P1 — rerun retained the schedule event.** The live query now binds
   GitHub's attempt field and rejects attempt greater than one as natural
   schedule evidence, with a dedicated negative regression.
7. **Resolved P2 — three artifacts omitted attempt identity.** Candidate,
   coverage and benchmark artifacts now suffix both SHA and run attempt,
   matching the final receipt and avoiding immutable-upload collisions on
   rerun.

## Result

No unresolved architecture, security, privacy, compatibility or operations
finding remains. Repository-administration and the canary are intentionally
post-merge activities with explicit rollback and live receipts.
