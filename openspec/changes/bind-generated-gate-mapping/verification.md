# Verification evidence

- **Issue:** #61, follow-up to #24 / PR #60
- **Develop base:** `69d38874e3d3f669d60e129ed6882e835aedafe6`
- **Local platform:** macOS 26.2, aarch64-darwin, Python 3.14.3
- **Result:** every applicable local gate passed

## Contract and mutation evidence

- `python3 scripts/tests/support-policy.py` passed 46/46 tests. The three new
  mutations cover constant-name gate collapse, a value detached from
  `makeGate gate`, and a nested `generatedChecks` shadow between mapping and
  returned publication.
- `./scripts/check-support-policy.py` passed the canonical 23-gate manifest and
  generator.
- Ruff lint and format checks passed for the validator and mutation suite.
- `python3 scripts/benchmark-support-policy.py --samples 20` passed with warm
  p50/p95 of 6.876/7.430 ms and process-cold p50/p95 of 44.150/46.093 ms.
- `./scripts/factory check` passed all 18 active-change/canonical items.
- `nix flake check --print-build-logs` passed every compatible
  aarch64-darwin check; x86_64-linux execution is intentionally supplied by
  hosted CI.

## Review conclusion

The late P1 is confirmed and fixed. The implementation also closes the
adjacent lexical-shadow route found during the distinct local review. The
accepted source shape is intentionally fail-closed and may reject a future
semantically equivalent Nix refactor until its contract changes with review.
No unresolved blocker remains.

## Compatibility and isolation

- No Nix generator, manifest, derivation, Rust API, wire behavior, dependency,
  support surface, release policy, or security boundary changed.
- No donor source or fixture was used.
- No downstream repository was read or modified for this follow-up.
- Reserved `main` remains untouched at
  `2c267d65af5c6b6dc9c8fd6826266c8ad0c3256a`.
