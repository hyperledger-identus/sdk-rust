# Verification receipt

- **Develop base:** `707a5a22c3fad18724d5c5cac953e7387f7e49d8`
- **Planning head:** `0a7ded352110474150e409aabcb3a2c4edfed661`
- **Implementation head:** `f5c057e7b67a7b3c94560a0b03bca92a43110b6d`
- **Host:** aarch64-darwin
- **Result:** passed with no unresolved blocker

## Exact-head audit evidence

The checked baseline describes exact develop base `707a5a2`: 30,092 authored
nonblank production lines in 116 files and 2,341 production functions; 23,589
external-test lines in 71 files and 1,052 functions; and 1,158 inline-test
lines in 21 files and 95 functions. It contains 53 function and six module
attention signals plus 13 explicit hotspot dispositions.

The clean exact implementation audit at `f5c057e` reports 30,086 production
lines and 2,340 production functions: a delta of -6 lines and -1 function.
External characterization grows by 27 lines; inline-test counts, production
file count, and function/module attention-signal counts are unchanged. This
is supporting evidence only: the architectural improvement is one owner for
the identical standard-failure invariant, not the numeric reduction.

## Commands passed

```text
./bootstrap.sh -- python3 scripts/code-health-audit.py --output /tmp/sdk-rust-code-health-head-rebased.json
python3 scripts/code-health-audit.py --check-report docs/architecture/code-health-baseline.json
python3 scripts/tests/code-health-audit.py (6/6)
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test -p identus-did --test did_method_registry --test did_resolution_cache (16 passed, 2 ignored)
cargo test --workspace --all-features
cargo test --workspace --no-default-features
cargo nextest run --workspace --no-fail-fast --no-tests=pass (701/701, 22 skipped)
cargo build --locked --workspace --all-targets
bash scripts/tests/factory-contract.sh
./scripts/factory check
./scripts/factory preflight establish-code-health-quality-budget --issue 270 --validate-receipt
nix build .#checks.aarch64-darwin.factory-contract .#checks.aarch64-darwin.lint-toml .#checks.aarch64-darwin.lint-text .#checks.aarch64-darwin.rust-clippy-all-targets-all-features --print-build-logs
nix build .#checks.aarch64-darwin.rust-build .#checks.aarch64-darwin.rust-test --print-build-logs
```

The implementation was rebased without conflict after issue #269 merged.
The planning and implementation commits were re-signed, the preflight receipt
and immutable baseline were rebound to the new base, and the exact-head audit,
strict Clippy, DID/factory gates and proportionate Nix gates were rerun after
the rebase. The full Cargo feature/no-feature and Nix workspace build/test
results above were obtained before the clean rebase; the rebase prerequisite
changed crypto secret encapsulation rather than the DID/tooling slice.

The first TOML formatting check found the newly authored manifest unformatted;
`taplo format` corrected it and the standalone and rebased Nix TOML gates then
passed. No failed result is represented as green.

## Unrun evidence

No x86_64-linux Nix build was run locally because the host is aarch64-darwin;
hosted CI owns that platform. Weekly cross-platform slow gates are not expanded
by this issue.
