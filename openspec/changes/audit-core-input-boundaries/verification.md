# Verification receipt

Verification date: 2026-09-10

## Commands passed

- `./bootstrap.sh --audit-pi --json`: pinned Pi 0.84.2, Node 24.19.0,
  repository hooks and managed-worktree capacity passed.
- `scripts/factory preflight audit-core-input-boundaries --issue 246 --write`:
  planning receipt written for `de5c613` on base `b5d1a4c`.
- Pi worker: `cargo test -p identus-core`, 24 passed and 0 failed; formatting,
  research, constraints, receipt and diff checks passed.
- Supervisor: `./bootstrap.sh -- cargo test -p identus-core`, 24 passed and 0
  failed; doc tests passed.
- `./bootstrap.sh -- cargo clippy -p identus-core --all-targets --all-features -- -D warnings`:
  passed.
- `./bootstrap.sh -- cargo fmt --all -- --check`: passed.
- `scripts/check-constraints.py . --change audit-core-input-boundaries --require-ready`:
  passed.
- `./bootstrap.sh --check`: all factory/OpenSpec structure and 9 operational
  tests passed.
- `git diff --check`: passed.

## Evidence

- Pi was invoked through `./bootstrap.sh --pi` in no-session print mode with a
  bounded tool allowlist; measured duration was 471 seconds.
- The generated Pi package cache used 168,177,664 bytes across 19,386 files and
  1,797 directories and made the worktree dirty. It was moved intact to
  `/tmp/sdk-rust-pi-npm-issue-246`; remediation is issue #247.
- Root manifests and `Cargo.lock` are unchanged.
- `SDK-LIM-007` remains effective and explicitly preserves pre-validation and
  unaudited-crate limitations.

## Checks not run

The complete weekly Nix matrix, portable runtime targets, fuzzing and release
checks are not run locally for this evidence-only core test/docs slice. The
exact committed target plan and hosted PR gate remain required evidence. No
unrun command is represented as passing.

## Residual limitations

The repository-wide inherited-input audit remains incomplete. Transport,
decompression, JSON-token scanning and owned-string allocation can precede SDK
validation and remain caller-budgeted. The canary proves no downstream
adoption, publication, release, package upgrade or `main` activation.
