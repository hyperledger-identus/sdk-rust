## 1. Contract and policy

- [ ] 1.1 Add machine-readable runtime, delivery, contribution, routing,
      metrics and capacity contracts adapted to sdk-rust.
- [ ] 1.2 Add the repository handbook, role definitions, recovery runbook and
      ADR recording the guidance-based adoption.

## 2. Operational tooling

- [ ] 2.1 Add the Nix-backed bootstrap and Pi audit/policy controls.
- [ ] 2.2 Add OpenSpec-before-implementation preflight and dev-loop routing.
- [ ] 2.3 Add contribution hooks and hosted policy parity.
- [ ] 2.4 Add managed worktree lifecycle and immutable CI planning.
- [ ] 2.5 Add private exact-head metrics validation/render/publication.

## 3. Evidence and integration

- [ ] 3.1 Add happy-path and known-bad contract tests for every new control.
- [ ] 3.2 Integrate the tests with the Nix factory contract and existing fast
      workflow without adding per-PR compiler lanes.
- [ ] 3.3 Run focused and full local gates, record independent review and
      archive the completed OpenSpec change.
- [ ] 3.4 Open the issue-linked PR, monitor exact-head CI and merge only when
      every required gate and review is clean.
- [ ] 3.5 After merge, run one separate issue-backed Pi canary and tune the
      harness only through a follow-up change based on measured friction.
