# Verification

## Passed

- `scripts/factory preflight isolate-pi-package-cache --issue 247
  --validate-receipt`: exact planning receipt remains valid at `8c06a67`.
- `./bootstrap.sh --check`: factory structure, strict OpenSpec store and all 14
  operational tests passed.
- `node --test scripts/tests/factory-operations.mjs`: 14 tests passed, including
  exact identity and lock separation, two-worktree reuse, simulated concurrent
  convergence, operator data and symlink refusal, and clean Git status.
- Fresh `./bootstrap.sh --prepare-pi-cache`: `npm ci` installed 113 locked
  packages with lifecycle scripts disabled; the verified cache contains 19,386
  files, 1,797 directories and 164,248 KiB.
- Repeated `./bootstrap.sh --prepare-pi-cache`: reused the same digest in about
  1.5 seconds without an install or generated Git change.
- `./bootstrap.sh --pi list --approve`: the pinned Pi 0.84.2 launch path passed
  its policy audit and resolved all three project packages through the external
  cache symlink.
- `./bootstrap.sh --audit-pi --json`: Node 24.19.0, npm 11.17.0, Nix Pi 0.84.2,
  Git hooks, worktree capacity and cache state `ready` passed.
- `npm audit --omit=dev --package-lock-only --prefix .pi/package-runtime
  --audit-level=high`: zero vulnerabilities.
- `nix build --print-build-logs .#checks.aarch64-darwin.factory-contract
  .#checks.aarch64-darwin.lint-text .#checks.aarch64-darwin.lint-toml
  .#checks.aarch64-darwin.lint-nix`: isolated factory, Markdown/YAML/shell,
  TOML and Nix lint checks passed.
- `git diff --check`, Node syntax and shell syntax checks passed.

## Intentionally deferred

- Hosted x86_64 Linux `fast`: authoritative after push.
- Weekly/manual complete Nix, fuzz, security and portable-target slow evidence:
  the change does not alter Rust, Cargo, public API or target behavior, and the
  active-development CI policy keeps these off each PR.
- A full model-backed Pi task: issue #246 already supplied the required canary;
  this tuning smoke exercises package discovery without starting another
  product slice.
