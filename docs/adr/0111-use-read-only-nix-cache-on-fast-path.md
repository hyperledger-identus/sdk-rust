# ADR 0111: use read-only Nix cache on the fast path

- **Status:** Accepted under standing routine authority
- **Date:** 2026-09-11
- **Issue:** [#260](https://github.com/hyperledger-identus/sdk-rust/issues/260)
- **Retrospective:** [discussion #256](https://github.com/hyperledger-identus/sdk-rust/discussions/256)
- **Complements:** ADR 0081
- **Review no later than:** 2026-12-08 and before any release candidate

## Context

The one-job Rust 1.98.1 fast lane is the required active-development merge
signal. Across three comparable successful PR runs, its median job duration was
694 seconds, while the median substantive gate was 249 seconds and `Post Magic
Nix Cache` was 393 seconds. The action restored useful paths but then uploaded
hundreds of Nix paths sequentially and also probed unauthenticated FlakeHub.

Cache publication is not build, test or conformance evidence. Letting it
dominate the required status undermines ADR 0081's feedback objective.

## Decision

1. Keep the pinned Magic Nix Cache action in `fast` as a restore accelerator.
2. Set GitHub job cache authority to `read`; the job may restore but cannot save.
3. Select GitHub Actions cache explicitly and disable FlakeHub and the optional
   diagnostic endpoint.
4. Treat the cache step as best effort. Cache errors are visible but the
   unchanged Nix gates determine correctness.
5. Bound the whole required job to 20 minutes. A timeout fails closed and
   blocks merge.
6. Keep all eight existing Nix check attributes and Rust/Nix inputs unchanged.
7. If the exact-head canary's cache finalizer exceeds 30 seconds, remove the
   cache action from `fast` before merge; do not restore write authority.
8. Target a median complete job at or below 480 seconds over the first three
   successful comparable post-change PR runs. Review with ADR 0081 by
   2026-12-08.

## Consequences

Warm cache entries can still reduce build time, while pull requests stop
creating path-by-path cache uploads. Expired or missing entries may increase
the substantive phase, but they cannot change derivation identity or waive a
gate. No new service, action, secret, permission, cost or dependency is added.

Slow weekly/manual workflows retain their existing cache behavior and remain
the appropriate place to revisit trusted cache publication separately.

## Alternatives rejected

- **Implicit current behavior:** measured finalization dominates the lane.
- **Immediate cache removal:** safe fallback, but discards proven restore value
  before testing the native read-only control.
- **New restore-only cache action:** expands the supply chain and store/database
  semantics without evidence of a better outcome.
- **Authenticated FlakeHub:** adds protected service, credential/OIDC, ownership
  and possible cost decisions outside this routine issue.

## Verification and rollback

The offline support-policy checker binds read-only cache authority, explicit
backend selection, disabled FlakeHub/diagnostics, best-effort handling, timeout
and the unchanged Nix gate list. Hosted job timestamps supply canary evidence.
Rollback reverts this workflow/policy change; the cache-free fallback removes
only the action while preserving all substantive commands.
