# Local review

## Scope and identity

- Issue: `#374`
- Exact base: `b7348510ce0db180a320bdf557c3976be56c67ec`
- Reviewed implementation head: `8c265134f773060ce719c7eee5a0d9c197ecce3b`
- Scope: PR metadata shell checker, focused operational tests and the
  `factory-operations` contract only.

## Findings

1. **Resolved — here-string portability stall.** The first implementation
   removed the producer pipeline with here-strings. macOS Bash 3.2 passed, but
   the pinned Linux Bash 5.3 stalled on the 60 KiB regression case inside the
   cleaned Nix source. The final implementation uses one unpredictable
   mode-0600 temporary file with exit-trap cleanup and passes under both Bash
   versions.
2. **Resolved — cleanup needed executable evidence.** The regression now runs
   long inputs with an isolated `TMPDIR` and fails if the checker leaves any
   body material behind.
3. **No unresolved policy finding.** All existing regexes, case behavior,
   issue extraction, diagnostics and negative cases remain unchanged. No
   contribution requirement or input bound is relaxed.
4. **No unresolved security/privacy finding.** PR text is never evaluated or
   echoed. The temporary name is unpredictable, permissions are tightened to
   0600, normal exits remove it, and diagnostics do not disclose its path.
5. **No unresolved architecture finding.** The change stays within the
   existing shell policy owner and test owners; no duplicate validator, public
   SDK API, dependency, workflow or product behavior is introduced.

## Granularity

The correction changes one four-match checker and its existing shell/Node
tests. Splitting it would either leave the false-negative bug unfixed or omit
the cross-platform/file-backed regression. The slice is independently
reversible and below the factory decomposition threshold.

## Residual limitations

- `SIGKILL` cannot run shell traps; a platform may retain the mode-0600
  temporary file until ordinary temporary-directory cleanup. This is not new
  secret material—the PR body is destined for GitHub—but it is recorded
  explicitly.
- A full local cleaned-source factory derivation enters the repository's known
  nested factory self-test after the relevant Nix `pr-policy tests: passed`
  receipt. It was stopped rather than allowed to recurse indefinitely. Hosted
  exact-head `fast` remains the authoritative complete integration gate.

## Verdict

Approved locally with no unresolved blocking finding. Exact-head hosted CI is
required before protected merge.
