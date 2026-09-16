# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/303
Constraint blockers: none

## Existing entries affected

- `SDK-RUST-001` and `SDK-COMPAT-005`: both lines continue to use the accepted
  Rust 1.98.1 active-development baseline; no lower MSRV is introduced.
- `SDK-LIM-004`: the fast/slow execution split remains temporary and must be
  reviewed by 2026-12-08 and before a release candidate.
- `SDK-SUPPLY-001`: Nix/Cargo locks and exact action pins remain authoritative;
  no new dependency or cache write authority is added.
- ADR 0111: pull-request Nix cache use remains read-only/best-effort.
- ADR 0120 and issue #276: native weekly scheduling and natural-run acceptance
  remain unchanged.

## Introduced or changed constraints

The only required active-development Rust/factory PR status SHALL remain
`fast`. Its initial observational SLO SHALL be p50 at most six minutes and p95
at most eight minutes; a measured p95 above ten minutes SHALL create or select
a focused optimization issue rather than silently removing evidence. One
automatic discovery review and one remediation round are the default maximum.
Round count SHALL never waive a P0/P1, security regression, introduced defect,
or failed acceptance criterion. Later independent non-blocking findings SHALL
be linked to follow-up issues.

Production promotion SHALL require a green slow receipt bound to the exact
candidate SHA, no subsequent code change, and no unresolved release blocker.
Slow debt SHALL block promotion, publication, and release preparation but SHALL
not retroactively invalidate an unrelated merged active-development slice.

## Introduced or changed limitations

File and changed-line thresholds are decomposition guidance, not an automatic
merge failure. GitHub-hosted duration varies with queue and cache conditions;
SLO calculations distinguish queue from execution and use a rolling sample.
The fast line does not claim macOS, mobile runtime, browser, full feature/target,
coverage, performance, fuzz, sanitizer, release archive, or publication proof.
The slow line is intentionally measured in tens of minutes.

## Consumer and product impact

No SDK consumer contract changes. Active-development integrations arrive with
faster bounded evidence; production-support and release claims remain blocked
until the broader exact-SHA proof succeeds.

## Activation and rollback

Activation requires policy tests, target-plan tests, factory/OpenSpec checks,
signed/DCO review, and merge into `develop`. Rollback restores the prior
process wording and machine fields without changing workflow jobs. Existing
slow receipts and issue #276 evidence remain valid historical records.

## Evidence

Evidence is discussion #302, issue #303, PR/Actions metadata for #274/#296/#300,
slow run 35006444994, canonical ADRs 0081/0111/0120, the factory operations and
metrics specifications, and focused policy/target-plan tests required by this
change. No constraint blocker remains.
