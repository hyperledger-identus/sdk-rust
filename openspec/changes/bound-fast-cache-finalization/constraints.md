# Constraints and limitations

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/260
Constraint blockers: none

## Existing entries affected

- `SDK-COMPAT-005`: the temporary Rust 1.98.1 fast/slow policy and 2026-12-08
  review date remain effective; this change makes its fast signal meaningfully
  bounded without changing the compiler or gate set.
- `SDK-AGENT-001`: autonomous merge still requires a successful exact-head
  `fast` check. Cache acceleration never grants merge authority.
- `SDK-LIM-006`: release, publication and `main` remain inactive.

## Introduced or changed constraints

The required `fast` job has read-only cache authority, explicit GitHub Actions
cache selection, no FlakeHub or diagnostic endpoint, non-fatal cache setup, and
a 20-minute job timeout. Its eight substantive Nix checks remain mandatory.

## Introduced or changed limitations

Read-only operation depends on existing cache entries and GitHub's eviction,
scope and rate-limit policy. A cache miss can increase substantive build time.
The 480-second median is an active-development throughput target, not a public
compatibility, release or service-level commitment. A 20-minute timeout bounds
the check but does not prove every upstream network failure resolves earlier.

## Consumer and product impact

No Rust API, wire contract, crate, dependency, feature, target, MSRV or
downstream repository changes. The change affects hosted CI latency and cache
write authority only.

## Activation and rollback

The policy activates after the issue #260 PR merges to `develop`. Rollback
reverts the workflow/policy validation change. If read-only Magic Nix Cache
still exceeds the finalization bound on the candidate, remove the action from
`fast` before merge while keeping the same Nix commands.

## Evidence

Offline tests will reject missing read-only authority, restored cache writes,
FlakeHub/diagnostic enablement, a missing timeout, fatal cache handling and any
change to the substantive check list. Hosted evidence will record exact step
durations and the action annotation state.
