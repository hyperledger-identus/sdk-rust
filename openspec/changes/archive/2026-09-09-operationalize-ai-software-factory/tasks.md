## 1. Contract and policy

- [x] 1.1 Add machine-readable runtime, delivery, contribution, routing,
      metrics and capacity contracts adapted to sdk-rust.
- [x] 1.2 Add the repository handbook, role definitions, recovery runbook and
      ADR recording the guidance-based adoption.

## 2. Operational tooling

- [x] 2.1 Add the Nix-backed bootstrap and Pi audit/policy controls.
- [x] 2.2 Add OpenSpec-before-implementation preflight and dev-loop routing.
- [x] 2.3 Add contribution hooks and hosted policy parity.
- [x] 2.4 Add managed worktree lifecycle and immutable CI planning.
- [x] 2.5 Add private exact-head metrics validation/render/publication.

## 3. Evidence and integration

- [x] 3.1 Add happy-path and known-bad contract tests for every new control.
- [x] 3.2 Integrate the tests with the Nix factory contract and existing fast
      workflow without adding per-PR compiler lanes.
- [x] 3.3 Run focused and full local gates, record a distinct review and pass
      the guarded archive preflight.
- [x] 3.4 Prepare the issue-linked PR, exact-head CI/merge procedure and the
      bounded post-merge Pi canary handoff.
- [x] 3.5 Record the canary rule: a separate issue and OpenSpec change own the
      real SDK slice, and measured friction alone may start harness tuning.
