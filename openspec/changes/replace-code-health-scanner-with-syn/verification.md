# Verification

## Exact-head local evidence

The initial implementation and evidence head
`f2e5d4a5a3162a1155d5e71208f424aeb767feb0` passed:

- Nix `aarch64-darwin` checks: `factory-contract`, `lint-nix`, `lint-text`,
  `lint-toml`, `rust-fmt`, `rust-build`, `rust-clippy`, and `rust-test`;
- 735 workspace tests under Cargo Nextest, including six focused classifier
  tests; 22 intentionally skipped tests were reported by the existing suite;
- strict conformance Clippy with warnings denied and all-target conformance
  tests;
- eight Python protocol/report mutation tests;
- exact v2 baseline regeneration under the pinned Nix shell;
- factory structure and mutation suites, including 72 strict OpenSpec items;
- Markdown, YAML, shell, EditorConfig, TOML and Nix linting; and
- `git diff --check`.

The warm local fast source-evidence command completed in 5.99 seconds with a
45,645,824-byte maximum resident set size on macOS ARM64. This measurement
includes entering the cached Nix development shell and invoking Cargo. The
heavy `rust-code-analysis-cli` metric engine was not run on that fast path.

Hosted-review remediation through exact classifier/evidence head
`1d10a646776f1709dbe39cad83a54c5c093c5fed` additionally passed:

- all 37 conformance targets/tests and strict all-target Clippy;
- all nine Python classifier/report mutation tests;
- exact durable-source baseline regeneration and canonical report validation;
- factory structure/mutation checks and strict change validation; and
- `cargo fmt --all -- --check` plus `git diff --check`.

The added regressions prove inherited test-only state through nested expression
scopes and fail-closed behavior for unresolved conditional module path
overrides.

## Migration evidence

The v1 classifier from planning commit `a27f31f` and the reviewed v2 classifier
classified durable source revision
`5dff6f38c861b858dd62dc8310246f7d485d3e92`: the same 135 production sources
and 73 external test files. Both produced 3,990 inline-test and 29,161
production authored nonblank lines, with no
per-file authored nonblank delta. The exhaustive comparison and the intentional
252 blank-only line representation change are recorded in
`docs/architecture/code-health-v2-migration.md`.

## Hosted evidence

The pull-request number, exact hosted head, required-check results and review
resolution are added before archival and merge. Protected `develop` remains
the authoritative integration gate.
