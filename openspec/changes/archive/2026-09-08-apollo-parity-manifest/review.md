# Exact-diff and evidence-integrity review

Review date: 2026-09-09
Reviewer role: distinct post-implementation evidence review
Verdict: approved after findings were resolved

## Scope reviewed

- The 27 fixed capability IDs, 22 fixed vector/evidence IDs, 11 dependency
  decisions and five target records were compared with issue #211 and the
  pinned Apollo inventory in Discussion #178.
- Every Apollo source URI was checked for the declared Apollo revision and
  every vector origin URI for its exact repository, full revision and path.
- Every local SDK evidence path exists and every named selector occurs in the
  referenced source or integration-test file.
- Summary counts were independently recomputed as 14 `parity`, six
  `sdk-exceeds`, seven `accepted-difference` and zero `gap` rows.
- Factory wiring, deterministic rendering and mutation cases were reviewed for
  fail-open behavior, path traversal, hidden schema drift and coupling.

## Findings and resolutions

1. The initial checker fixed the capability inventory but allowed a vector row
   and its capability reference to disappear together. The checker now fixes
   the complete 22-ID vector inventory and a mutation test proves drift fails.
2. The initial safe-path rule rejected lexical traversal but did not reject a
   repository path resolving through a symlink to an external file. Resolution
   containment is now enforced and a symlink-escape mutation test proves it.
3. The first manifest draft had one SHA/vector origin mismatch and the first
   Nix run exposed missing staging plus TOML/editorconfig formatting. Each was
   corrected; the canonical checker, hermetic factory, TOML lint and text lint
   now pass.

## Residual limitations

- Hosted link availability is reviewed externally; the offline checker proves
  immutable shape and declared-revision binding, not GitHub uptime.
- Test selector presence is a stable evidence locator, not Rust syntax
  analysis. The Rust and hosted gates remain the executable proof.
- Performance, coverage, fresh closing-target receipts and production language
  bindings remain explicitly owned by #214, #212, #213 and #163/#215.
- Issue #179 remains monitor-only and does not block this parity ledger.

No unresolved correctness, evidence-integrity, security, portability,
dependency, ownership or scope finding remains in this change.
