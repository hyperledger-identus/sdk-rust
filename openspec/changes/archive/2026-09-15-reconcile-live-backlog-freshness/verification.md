# Verification

## Focused evidence

- `scripts/tests/ssi-upstream-backlog-live.py`: seven success/failure families
  passed, including closed active ownership, closed evidence rows, missing and
  duplicate issues, malformed/exact-URL snapshots, rejected alternate ledgers
  and redacted GitHub failure.
- `scripts/factory backlog-live`: 30 rows and nine unique live issues passed.
- `scripts/check-ssi-upstream-backlog.py`: all 30 canonical rows passed.
- `scripts/check-openspec-archive.py . --change
  reconcile-live-backlog-freshness`: the complete modified requirement and
  exact archive intent passed.

## Repository evidence

- `scripts/tests/factory-contract.sh`: passed, including 173 support-policy
  mutations, 101 immutable public-error bindings and the new live-audit suite.
- `scripts/factory check`: passed with 70 OpenSpec items while the change was
  active.
- Full `nix flake check --print-build-logs`: the implementation snapshot
  passed on `aarch64-darwin` with the Rust 1.98.1 workspace lane at 721/721
  tests. The final evidence-only snapshot repeated 721/721 and exposed one
  non-canonical `archive-intent.toml` layout; repository Taplo formatting and
  `nix build .#checks.aarch64-darwin.lint-toml --print-build-logs` then passed.
  Nix otherwise reported only the repository's documented Darwin audit-helper
  warning.

## Coordination evidence

- IDR-004 active ownership moves from closed #95 to open completion-audit #286.
- IDR-023 active ownership moves from closed #250 to open component epic #7.
- Native binding issue #222 is closed with exact #227/#229/#231 merge receipts.
- Parent #163 records the remaining experimental/support boundary and blocked
  React Native child #223.

No SDK runtime, public API, consumer repository, repository setting, release,
publication or `main` state changed.
