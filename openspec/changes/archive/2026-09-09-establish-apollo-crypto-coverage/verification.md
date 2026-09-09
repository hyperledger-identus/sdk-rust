# Verification receipt

Verification status: complete

## Scope

- Issue: https://github.com/hyperledger-identus/sdk-rust/issues/212
- Base: `origin/develop@0fa063cde16aadab91477cebbc82ec44df4e57c4`
- Branch: `codex/apollo-coverage-evidence`
- Evidence revision: `7fcdfacf778d75ae3ae7e9a9486c2319c1a97695`
- Donor and consumer repositories: unchanged

## Readiness

- `./scripts/factory doctor` passed before edits.
- `./scripts/factory research-ready establish-apollo-crypto-coverage` passed.
- `./scripts/factory constraints-ready establish-apollo-crypto-coverage`
  passed.
- Strict OpenSpec validation passed before implementation.

## Coverage evidence

- Nix-devshell `./scripts/coverage-crypto.sh` executed default, all-feature,
  no-default-feature and `kmp-compat` profiles successfully.
- Local and hosted results are identical: **1,481 / 1,722 executable lines,
  86.004646%**, exceeding the 74.82% Apollo threshold by 11.184646 percentage
  points.
- Hosted coverage job
  https://github.com/hyperledger-identus/sdk-rust/actions/runs/34295862000/job/102292244241
  passed on Rust 1.98.1 with cargo-llvm-cov 0.9.0.
- Artifact `crypto-coverage-7fcdfacf778d75ae3ae7e9a9486c2319c1a97695`
  contains deterministic JSON, Markdown and LCOV bound to the exact revision,
  compiler, tool, profiles, exclusions and counts.
- The downloaded LCOV contains exactly the same 16 executable crypto source
  files as JSON, all repository-relative; no dependency, test, example,
  generated, core or derive source record remains.
- Discussion receipt:
  https://github.com/hyperledger-identus/sdk-rust/discussions/178#discussioncomment-18359067

## Focused and factory evidence

- `python3 scripts/tests/crypto-coverage.py` -- 17 passed.
- `python3 scripts/tests/apollo-parity.py` -- 29 passed.
- `python3 scripts/tests/support-policy.py` -- 159 passed.
- `scripts/tests/factory-contract.sh` passed its complete nested suite.
- `./scripts/factory check` passed 52 OpenSpec specs/changes with zero failures.
- Nix-devshell ShellCheck and actionlint passed for the changed workflow and
  runner; `git diff --check` passed.
- Tracked TOML formatting passed with the repository-pinned Taplo.
- Local Nix `factory-contract`, `lint-toml`, `lint-nix` and `lint-text`
  derivations passed after the formatting repair.

## Hosted slow evidence

- Run https://github.com/hyperledger-identus/sdk-rust/actions/runs/34295862000
  is the exact evidence campaign for the revision above.
- Coverage and performance jobs passed. The Linux and macOS full Nix lanes are
  the hosted authority for the complete platform matrix and are required to be
  green before integration.
- Non-failing runner annotations concern FlakeHub authentication, repository
  retention limits and GitHub's Node runtime migration; none changes the
  evidence result.

## Review and limitations

The distinct review is recorded in `review.md`; both findings were repaired.
Line coverage remains non-semantic evidence and cannot replace published test
vectors, negative-path selectors, side-channel analysis, fuzzing or release
assurance. The tooling is slow-lane-only and adds no SDK runtime dependency.
