# Exact-diff review

Review date: 2026-09-10
Reviewer: supervising LLM agent, distinct from the Pi implementation worker
Review base: `b5d1a4c52d31f3d37564eee9c6edc890fc7b1295`
Review status: accepted

## Scope and contract

The reviewed change is limited to issue #246: one `identus-core` serde
regression test, a complete public-surface inventory, the associated
`SDK-LIM-007` evidence wording, canary evidence and the active OpenSpec change.
The pre-implementation receipt binds planning head `de5c613` before any of
those implementation paths changed.

## Correctness review

- `UnixTimestampMillis` and `DurationMillis` are generated over `u64`; the new
  test exercises the actual public serde implementation at `u64::MAX` and at
  negative, fractional and one-above-maximum inputs.
- The inventory was checked against every public item and re-export in
  `crates/core/src/{lib,time,url}.rs` and against generated constructors in
  `crates/derive/src/{num,str}.rs`.
- `MonotonicTimestampMillis` remains deliberately free of serde derives.
- URL evidence continues to cite exact/over byte tests and the pre-allocation
  caveat; no existing limit or validation order changed.
- No public API, wire behavior, manifest, lockfile, feature or target changed.

## Security and limitation review

The test contains fixed public literals and produces no dynamic diagnostics.
The inventory does not confuse fixed retained state with parsing/allocation
performed before typed deserialization. `SDK-LIM-007` stays effective for all
unaudited SDK crates and outer hostile-input budgets. No secret, PII, auth,
prompt, transcript, session or provider data is tracked.

## Factory observations

The worker changed only authorized paths. Worktree-local Pi package pollution
was removed intact from the worktree and recorded in issue #247; no harness
change or version upgrade is bundled. Silent print-mode latency, one corrected
preflight invocation and unapproved npm-script warnings remain observations,
not implied authorizations.

## Findings

No blocking implementation finding remains. The initial factory-only canary
framing contradicted the canonical issue-backed SDK-slice requirement and was
corrected before the planning commit and preflight receipt.
