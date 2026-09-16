# ADR 0128: retain local and publish bounded delivery metrics

- **Status:** Accepted for implementation
- **Date:** 2026-09-17
- **Decision authority:** project-sponsor direction recorded in issue #306
- **Related:** ADRs 0108 and 0127; issue #259

## Context

The factory stores closed exact-head metrics below the Git common directory and
can render a privacy-bounded issue comment. Publication is optional and tied to
the caller's current checkout, so normal post-merge evidence and most retained
historical records are not visible to collaborators.

## Decision

1. The canonical metric remains an owner-only local record keyed by schema,
   issue, and exact head.
2. Every completed production-ready work item attempts one public bounded
   receipt after the local record is retained or confirmed.
3. Publication targets the recorded PR by default, falls back to the issue
   when no PR exists, and permits explicit issue targeting for historical or
   issue-centric reporting.
4. Historical PR-backed publication verifies the exact hosted PR head and
   repository/issue identity rather than requiring the current checkout head.
5. Public comments contain only the existing allowlisted aggregate and bounded
   versioned payload. Raw Pi or agent content never leaves the private run.
6. Comment mutation receives one bounded retry. Persistent failure is visible
   telemetry debt but does not invalidate independent product evidence.

## Consequences

- Reviewers can compare delivery time, CI churn, runtime usage, and resource
  evidence where the work happened.
- Post-merge publication and historical backfill become safe and idempotent.
- GitHub comments are derivatives, not backups; local retention and privacy
  boundaries remain authoritative.
- Telemetry availability cannot become an accidental product-code merge veto.

## Rejected alternatives

- Public raw session artifacts violate the privacy contract.
- Issue-only comments separate normal metrics from the exact PR review surface.
- Current-checkout-only verification makes post-merge publication impractical.
- Remote-only metrics lose the private authoritative record and retention
  control.

## Rollback

Restore optional issue-only publication. Keep local records and already-public
versioned comments as valid historical evidence.
