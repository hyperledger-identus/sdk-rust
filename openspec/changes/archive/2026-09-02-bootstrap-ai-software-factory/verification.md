# Verification evidence

Verified on 2026-09-02 from `codex/ai-software-factory`, based on
`origin/develop` at `279aefa3bad42aacf79a022fc7d39fdc80f4ce7b`.

## Passing gates

| Gate | Result |
| --- | --- |
| `scripts/tests/factory-contract.sh` | Passed all positive and negative fixtures |
| `scripts/check-factory.sh .` | Passed repository structure and local-state policy |
| `scripts/factory check` | Passed OpenSpec doctor and 10 strict validations |
| ShellCheck, actionlint, yamllint and Markdown/link checks | Passed |
| `nix build .#checks.aarch64-darwin.factory-contract` | Passed the packaged factory contract |
| `nix develop --command cargo fmt --all -- --check` | Passed |
| `nix develop --command cargo clippy --workspace --all-targets --all-features -- -D warnings` | Passed |
| `nix develop --command cargo doc --workspace --no-deps` | Passed |
| `nix develop --command cargo test --workspace --all-features` | Passed all unit, integration, UI and doc tests |
| `nix flake check --print-build-logs` | Passed all 12 `aarch64-darwin` checks |

The flake run included two hermetic Nextest variants. Each ran 104 tests and
passed all 104. It also passed formatting, strict clippy, documentation, WASM,
dependency policy, audit, Nix/TOML/text lint and the factory contract. The local
flake command reported `x86_64-linux` as an incompatible omitted system; the
GitHub workflows exercise Linux.

## Environment-specific diagnostics

The bootstrap found and resolved one environment gap: the initial direct test
run in the macOS dev shell could not link `-liconv`, so the dev shell now
includes `libiconv` on Darwin. The final dev-shell test run is recorded above.

One baseline toolchain-specific diagnostic remains: host Rust 1.95
`cargo test --workspace --all-features` ran the workspace tests
  through the derive UI suite, where 2 of 10 trybuild cases differed only in
  compiler diagnostic wording: `associated function or constant` became
  `function or associated item`. The Rust source and snapshots were deliberately
  not changed in this factory-only pull request.

## Boundary evidence

- `main` and `origin/main` remain
  `2c267d65af5c6b6dc9c8fd6826266c8ad0c3256a`.
- `develop` and `origin/develop` remain
  `279aefa3bad42aacf79a022fc7d39fdc80f4ce7b` before pull-request delivery.
- All implementation changes are isolated in the SDK Rust worktree and feature
  branch; no Rust crate source or public API changed.
- The Oxid and Midnight Identity worktrees were inspected and left as-is. Their
  existing local status is outside this branch and is not included in its diff.

Independent maintainer review and repository-hosted CI remain required before
merge.

## Readiness receipt

Generated after the signed implementation commit passed `factory ready`:

```text
Change: bootstrap-ai-software-factory
Branch: codex/ai-software-factory
Head SHA: 0e78abb9e5974f976427a37d6ab3a475abb164a1
Develop merge base: 279aefa3bad42aacf79a022fc7d39fdc80f4ce7b
Factory contract: passed
Product-specific gates: not asserted; attach their outputs separately
```
