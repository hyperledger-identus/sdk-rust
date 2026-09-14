# Verification receipt

- **Develop base:** `707a5a22c3fad18724d5c5cac953e7387f7e49d8`
- **Planning head:** `0a7ded352110474150e409aabcb3a2c4edfed661`
- **Implementation head:** `f5c057e7b67a7b3c94560a0b03bca92a43110b6d`
- **First review-remediation head:** `34a197692af144cd53eeafed31175b8a9f8d2af4`
- **Second review-remediation head:** `88bcb1700170b184760e388634f5ebbf49aeb108`
- **Third review-remediation head:** `6096cab17a54882cf98e7044d8174610208ccdc4`
- **Fourth review-remediation head:** `2b948ebe47eb03fbf2207d6ba8b7ca2ca859c785`
- **Fifth review-remediation head:** `f1826db71de8f370bfb5cf3dd49f0725be2602bb`
- **Sixth review-remediation head:** `2acf13f4ccb18450b7ebc21294330ac095fd2c9e`
- **Seventh review-remediation head:** `3bafdbb7ebc3392e381316b28c3ec3afd627c32e`
- **Eighth review-remediation head:** `1521c242ffac901f37dea0766b1510ae2f5180dc`
- **Final review-remediation head:** `8ef23489d4cded5cb635b970de9c4e1da360d711`
- **Host:** aarch64-darwin
- **Result:** passed with no unresolved blocker

## Exact-head audit evidence

The checked baseline describes exact develop base `707a5a2`: 28,374 authored
nonblank production lines in 111 files and 2,194 production functions; 22,600
external-test lines in 69 files and 982 functions; and 3,865 inline-test lines
in 28 files and 312 functions. It contains 50 function and six module
attention signals plus 13 explicit hotspot dispositions.

The clean exact implementation audit at `8ef2348` reports 28,368 production
lines and 2,193 production functions: a delta of -6 lines and -1 function.
External characterization grows by 27 lines to 22,627; inline-test counts,
production/external file counts, and function/module attention-signal counts
are unchanged. This is supporting evidence only: the architectural improvement
is one owner for the identical standard-failure invariant, not the numeric
reduction.

## Commands passed

```text
./bootstrap.sh -- python3 scripts/code-health-audit.py --output /tmp/sdk-rust-code-health-head-rebased.json
python3 scripts/code-health-audit.py --check-report docs/architecture/code-health-baseline.json
./bootstrap.sh -- python3 scripts/code-health-audit.py --verify-baseline
python3 scripts/tests/code-health-audit.py (37/37)
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test -p identus-did --test did_method_registry --test did_resolution_cache (16 passed, 2 ignored)
cargo test --workspace --all-features
cargo test --workspace --no-default-features
./bootstrap.sh -- cargo nextest run --workspace --no-fail-fast --no-tests=pass (703/703, 22 skipped)
cargo build --locked --workspace --all-targets
bash scripts/tests/factory-contract.sh
./scripts/factory check
./scripts/factory preflight establish-code-health-quality-budget --issue 270 --validate-receipt
nix build .#checks.aarch64-darwin.factory-contract .#checks.aarch64-darwin.lint-nix .#checks.aarch64-darwin.lint-toml .#checks.aarch64-darwin.lint-text .#checks.aarch64-darwin.rust-clippy-all-targets-all-features .#checks.aarch64-darwin.rust-build .#checks.aarch64-darwin.rust-test --print-build-logs
```

The implementation was rebased without conflict after issue #269 merged.
The planning and implementation commits were re-signed, the preflight receipt
and immutable baseline were rebound to the new base, and the exact-head audit,
strict Clippy, DID/factory gates and proportionate Nix gates were rerun after
the rebase. The full Cargo feature/no-feature and Nix workspace build/test
results above were obtained before the clean rebase; the rebase prerequisite
changed crypto secret encapsulation rather than the DID/tooling slice.

After the initial PR review failed the evidence-integrity design, the corrected
implementation added Git-tree source binding, complete slow regeneration,
closed schemas, exact generated exclusions, balanced comma termination and
recursive test-only module inheritance. Follow-up review then removed implicit
trust from `src/tests.rs`, preserved nested inline-module resolution context,
isolated macro token trees, rejected `#[path]` overrides, included outer doc
attributes and closed schema primitive types. Final review added nested
test-attribute context and a production-wins fixed point for mixed module
reachability. Mutation and population fixtures cover all reported failure
modes. Hosted review added balanced brace-delimited item macros with and
without a trailing semicolon, then comment/raw-token parsing and recursive
three-valued `cfg_attr` application. Final review narrowed item subtraction to
proven terminators: ambiguous generic/comparison angles, comma-less members,
labels, block expressions, match arms and unrecognized macros remain
production; mixed source lines are production-wins. Stable cfg booleans,
Unicode/unknown predicates, inactive `cfg_attr` branches, raw module
identifiers and Unicode Rust lifetimes have exact regressions. Full syntax
classification is tracked separately in issue #275. Macro definition and
invocation token-tree contents remain production even when a macro consumes a
literal `#[cfg(test)]` token and emits the captured item without that attribute.
Inner `#![cfg(...)]` scopes are explicitly retained as production and delegated
to issue #275.
A direct `cargo nextest` invocation was
unavailable outside the pinned shell; the recorded bootstrap invocation is the
successful replacement. The Nix install fixup emitted a Darwin
`audit-tmpdir.sh` child segmentation diagnostic after tests, but the derivation
and complete multi-check command exited zero. No failed gate is represented as
green.

## Unrun evidence

No x86_64-linux Nix build was run locally because the host is aarch64-darwin;
hosted CI owns that platform. Weekly cross-platform slow gates are not expanded
by this issue.
