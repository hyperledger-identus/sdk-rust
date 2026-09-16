# Tasks

## 1. Contract and pre-implementation gate

- [x] 1.1 Record measured retrospective evidence in discussion #302 and issue
      #303.
- [x] 1.2 Define fast integration, slow promotion, review cutoff, push batching,
      slice guidance, SLOs, and rollback in OpenSpec and ADR 0127.
- [ ] 1.3 Pass research readiness, constraint readiness, strict OpenSpec, and
      immutable planning preflight before implementation.

## 2. Machine-readable factory contract

- [ ] 2.1 Add closed fast/slow lane semantics and budgets to factory policy.
- [ ] 2.2 Extend exact-diff target plans with separate integration and
      production-promotion decisions while preserving current fields.
- [ ] 2.3 Add focused mutation/unit tests for lane identity, budgets, review
      cutoff, slice guidance, promotion evidence, and fail-closed routing.

## 3. Agent and operator flow

- [ ] 3.1 Update factory operations, factory README, and agentic SDLC with the
      local-first and review-cutoff state transitions.
- [ ] 3.2 Document metrics interpretation and the rolling latency/attempt
      optimization trigger.
- [ ] 3.3 Confirm ordinary PR CI still exposes exactly one required `fast` Rust
      and factory gate and slow remains weekly/manual.

## 4. Verification and delivery

- [ ] 4.1 Run focused Node/shell policy tests, strict OpenSpec/factory checks,
      text/Nix/TOML lint, and fast-equivalent checks.
- [ ] 4.2 Complete a fresh local architecture/process review and route any new
      independent non-blocking finding to a follow-up issue after one
      remediation round.
- [ ] 4.3 Archive the change through the guarded facade, open an issue-linked
      PR, obtain exact-head green CI, and merge to protected `develop`.
