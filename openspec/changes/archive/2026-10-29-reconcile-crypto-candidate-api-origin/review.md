# Exact-diff release-integrity review

- **Review date:** 2026-10-29
- **Issues:** #586 and #587
- **Base:** `develop@6ebd4633c987539689ae180fbccbb629572ca45a`
- **Reviewed head:** `77372b0d`
- **Reviewer:** Codex CLI
- **Result:** passed after three medium findings were resolved
- **Unresolved blockers:** none

## Resolved findings

1. The first implementation rejected API removal before its bounded diff could
   survive atomic staging. The repair renders and bounds evidence first, then
   atomically retains a diagnostics-only output on incompatibility. Packages,
   publication workspace, and success receipt remain unavailable.
2. The retained removal report initially used the additive/equal classifier.
   Removals now take precedence and produce `incompatible`, including duplicate
   occurrence loss. Focused tests and a disposable complete negative candidate
   proved the exact output shape.
3. The exact archived-head review found that GitHub skipped the candidate
   artifact upload when qualification failed. The slow workflow now evaluates
   the upload under `always()`, so a retained diagnostics-only API diff remains
   available for triage. The release-candidate mutation suite protects the
   failure-path upload contract.

## Architecture and release decision

- The signed published `v0.1.0-rc.1` API origin remains byte-identical and is
  pinned by its exact SHA-256.
- Current generated API is compared as a line multiset, retaining duplicate
  re-export evidence. Additions pass; every removed/changed occurrence fails.
- Published-origin initialization fails before mutation.
- Evidence is deterministic and bounded. Failure output cannot be mistaken for
  a release-eligible candidate.
- No Rust source/API behavior, dependency, feature, version, target,
  publication state, tag, downstream repository, or release authority changed.

The exact archived-head review found no further actionable correctness,
security, architecture, documentation, test, or release-history defect after
the failure-path upload repair.
