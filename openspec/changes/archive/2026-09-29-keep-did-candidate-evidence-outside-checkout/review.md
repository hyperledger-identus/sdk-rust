# Exact-diff architecture and workflow review

Review status: completed
Review date: 2026-09-29
Base: develop@a320bc5186ce6b92f80559714928999799c21906
Implementation head: 35132ba6e19e31140249dc9d6b2d9c0a4d651340
Specification commit: 9aec4cddf76a2193eb0a73e87e12e55c6b6fd4b0
Unresolved blockers: none

## Findings

1. **Root cause — confirmed.** Primary qualification wrote untracked evidence
   into the shared checkout, so MSRV correctly rejected a dirty source. The
   candidate implementation and compiler matrix did not fail.
2. **Boundary — accepted.** `RUNNER_TEMP` is job-scoped, outside the checkout,
   available to both sequential compiler lanes, and accepted directly by the
   pinned artifact action. Primary and MSRV retain separate new subdirectories.
3. **Fail-closed behavior — accepted.** A dedicated porcelain-status step runs
   before MSRV and rejects tracked, staged, or untracked mutation. The builder's
   independent clean-source validation is unchanged.
4. **Evidence identity — accepted.** Artifact name, SHA/run-attempt binding,
   lane receipt schema, aggregator input, missing-file behavior, and seven-day
   retention are unchanged; only the producing job's local path moves.
5. **Regression coverage — accepted.** Offline policy requires both external
   outputs, the explicit clean boundary, and external upload path. Mutations
   demonstrate that each material regression is rejected.
6. **Compatibility/security — accepted.** No dependency, feature, source API,
   wire shape, compiler, target, permission, credential, trigger, publication,
   or support claim changes. Generated evidence can no longer become implicit
   source input.

## Residual limitation

Local and PR gates cannot prove native scheduled runner execution. The next
natural or authorized slow run supplies that evidence to #388; no rerun or
dispatch is authorized by this change.

## Post-green discovery review

An independent exact-head review found two workflow-observability gaps before
merge: structural policy did not prove that the clean-source step was ordered
between compiler lanes, and the original one-line assertion hid porcelain
diagnostics. The implementation now enforces the primary/boundary/MSRV order
with a reorder mutation and emits a bounded Git status diagnostic on failure.
Both findings are resolved.

## Decision

The repair is the narrowest cohesive fix: it removes evidence/source coupling
while retaining every release-safety invariant. No unresolved correctness,
security, compatibility, architecture, or workflow finding remains.
