# Local review

- **Review date:** 2026-09-16
- **Reviewed base:** `develop@1d5283105678a4e62253416454a60859111e635c`
- **Review angle:** factory architecture, policy integrity, target-plan parsing,
  CI topology, agent behavior, and regression safety
- **Verdict:** pass; no blocking or worth-fixing-now finding remains

## Findings

No P0/P1, security, compatibility, correctness, acceptance, or introduced
behavior defect was found. No independent P2/P3 follow-up was necessary from
this review.

## Review evidence

1. The physical workflow topology remains one PR/push `fast` workflow plus the
   existing weekly/manual `slow` workflow. No platform, compiler, binding,
   coverage, fuzz, sanitizer, performance, or release job moved into PR CI.
2. Policy validation fails closed if a second fast status appears, required
   fast evidence disappears, slow evidence placement changes, review becomes
   unbounded, the SLO ordering is invalid, or promotion stops requiring an
   unchanged exact SHA.
3. `--numstat --no-renames -z` gives an unambiguous bounded text-line count.
   The parser uses the first two tabs only, preserves tabs in path bytes,
   distinguishes binary records, rejects malformed counts, and protects the
   safe integer range.
4. Target-plan schema v2 preserves every v1 field while adding integration,
   promotion and iteration records. Unknown diff state still recommends the
   complete slow set.
5. Review cutoff never waives P0/P1, security regression, introduced defect or
   failed acceptance. It only moves a later independent non-blocking finding
   to a linked issue.
6. The existing `cache-mode: read` actionlint compatibility exception remains
   exactly as accepted by ADR 0111 and the Nix factory check; this change does
   not broaden it.

## Decomposition note

The final slice crosses the 12-file guidance because one behavioral contract
must remain synchronized across OpenSpec/ADR, root and Pi agent guidance,
machine policy, target-plan implementation, workflow summary, tests and
operator documentation. Splitting those paths would temporarily make agents
or CI interpret different lane semantics. The implementation code is confined
to one target-plan module and its existing test module; there is no SDK runtime
or dependency change. Changed text remains below the 1,000-line guidance.

## Residual risk

The SLO is observational and not yet computed automatically from a rolling
Actions sample. Existing version-2 metrics retain exact CI attempt and
execution data, and the handbook defines the aggregation/trigger. Automating a
dashboard is optional follow-up only if manual trend review proves unreliable;
it is not required to enforce either lane.
