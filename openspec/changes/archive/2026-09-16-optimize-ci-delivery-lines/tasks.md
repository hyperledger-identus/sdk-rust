# Tasks

## 1. Contract and pre-implementation gate

- [x] 1.1 Record measured retrospective evidence in discussion #302 and issue
      #303.
- [x] 1.2 Define fast integration, slow promotion, review cutoff, push batching,
      slice guidance, SLOs, and rollback in OpenSpec and ADR 0127.
- [x] 1.3 Pass research readiness, constraint readiness, strict OpenSpec, and
      immutable planning preflight before implementation.

## 2. Machine-readable factory contract

- [x] 2.1 Add closed fast/slow lane semantics and budgets to factory policy.
- [x] 2.2 Extend exact-diff target plans with separate integration and
      production-promotion decisions while preserving current fields.
- [x] 2.3 Add focused mutation/unit tests for lane identity, budgets, review
      cutoff, slice guidance, promotion evidence, and fail-closed routing.

## 3. Agent and operator flow

- [x] 3.1 Update factory operations, factory README, and agentic SDLC with the
      local-first and review-cutoff state transitions.
- [x] 3.2 Document metrics interpretation and the rolling latency/attempt
      optimization trigger.
- [x] 3.3 Confirm ordinary PR CI still exposes exactly one required `fast` Rust
      and factory gate and slow remains weekly/manual.

## 4. Verification and delivery

- [x] 4.1 Run focused Node/shell policy tests, strict OpenSpec/factory checks,
      text/Nix/TOML lint, and fast-equivalent checks.
- [x] 4.2 Complete a fresh local architecture/process review and route any new
      independent non-blocking finding to a follow-up issue after one
      remediation round.
- [x] 4.3 Prepare the guarded archive and signed issue-linked PR evidence for
      protected `develop`; hosted exact-head CI and merge follow the archive.
