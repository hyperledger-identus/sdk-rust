# Pre-implementation semantic, API, security, and performance review

- **Date:** 2026-09-05
- **Issue:** #73, child of #6 / `IDR-009` and #20
- **Develop base:** `4e4cf5a8b4fb66e289629d287b4b87c63d5fc2bf`
- **Result:** contract is implementable with no unresolved blocker

## Findings

1. Metadata/schema and verification are separate backlog capabilities. This
   slice should not reproduce Oxid's aggregate record or join IDR-007b to
   IDR-009a.
2. Oxid's structural, issuer, proof, temporal, status, and schema stages are
   reusable. Its trust stage is product policy and must be excluded.
3. A caller-supplied aggregate outcome creates a contradictory-state problem.
   Deriving the outcome removes the invariant instead of repeatedly checking
   it.
4. A six-element array is preferable to a vector/map/set: completeness is
   partly type-level, validation is bounded, lookup is direct, and no report
   collection allocation is required.
5. Fixed canonical ordering is an acceptable experimental API constraint. A
   new normative stage changes outcome semantics and deserves a compatibility
   decision.
6. Failed and not-checked states both need machine reasons. NotChecked without
   a reason would collapse unsupported, unavailable, skipped, and inapplicable
   evidence.
7. Bounded lower-ASCII reason codes permit stable namespaces without allowing
   dynamic error prose or secrets into portable records.
8. Operational verifier errors and mode-specific evidence payloads remain in
   adapters. The report records their contracted projection only.
9. A manual release diagnostic is appropriate for this small hot path; a wall-
   clock CI threshold would be noisy and is not acceptance evidence.
10. Donor and consumer repositories remain read-only. No build, branch switch,
    dependency repoint, generated artifact, or source deletion is needed.

Verdict: READY to implement after strict structural validation.
