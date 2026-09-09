# Verification

## Passed

- `node --test scripts/tests/factory-operations.mjs`: 9 tests, including
  invalid metadata/signature/head, unknown routing, symlinked Pi policy,
  privacy, preflight and unmanaged-worktree cases.
- `scripts/tests/factory-contract.sh`: existing and new fixture contract passed.
- `scripts/factory check`: 60 OpenSpec items and all structural contracts passed.
- `scripts/factory preflight operationalize-ai-software-factory
  --validate-receipt`: planning receipt remains valid at `91d7619`.
- `./bootstrap.sh --configure-git`: existing identity/signing material was
  preserved; repository hooks were activated.
- `./bootstrap.sh --audit-pi`: Nix Node 24, Pi 0.84.2, sccache, policy, hooks and
  capacity audit passed.
- `nix build --print-build-logs .#checks.aarch64-darwin.factory-contract`:
  passed after making the worktree containment fixture independent of a Git
  checkout.
- `nix flake check --print-build-logs`: all 28 compatible aarch64-darwin checks
  passed, including 690 workspace tests, Rust 1.98.1/MSRV feature variants,
  Clippy, formatting, docs, dependency policy, WASM, Android and iOS builds.
- `nix build --print-build-logs .#checks.aarch64-darwin.lint-nix
  .#checks.aarch64-darwin.lint-text .#checks.aarch64-darwin.lint-toml`: passed.
- `node scripts/git-hooks/local-policy.mjs pre-commit`: passed.
- exact planning-commit DCO, OpenPGP envelope and local cryptographic
  verification: passed.
- `git diff --cached --check` and shell/Node syntax checks: passed.

## Intentionally deferred

- Pi end-to-end SDK work: must use the merged `develop` factory in a separate
  issue and OpenSpec change.
- x86_64 Linux fast gate: authoritative hosted CI after push.
- Weekly/manual slow targets: recommended by the immutable plan and unchanged
  from ADR 0081; not promoted to per-PR requirements.
- Rust/API/coverage gates: no Rust source, Cargo dependency, feature, target or
  public API changed; hosted `fast` still runs its complete Rust gate.
