# Design

## Instrumentation and feature isolation

`scripts/coverage-crypto.sh` starts with `cargo llvm-cov clean --workspace`,
then runs the default, all-feature, no-default-feature and `kmp-compat` test
surfaces with `--no-report`. In cargo-llvm-cov 0.9.0, `--no-report` preserves
profiles; the commands must not add the mutually exclusive `--no-clean` flag.
The final `report` command exports summary JSON and LCOV from the union.

## Denominator and artifact contract

`scripts/report-crypto-coverage.py` accepts cargo-llvm-cov summary JSON and
selects only regular `.rs` files whose resolved path is below
`crates/crypto/src`. It sums LLVM's line counts, applies the 74.82% threshold
using integer cross-multiplication, and emits deterministic `summary.json` and
`summary.md`. Metadata includes schema, Git revision, Rust version,
cargo-llvm-cov version, four exact feature profiles, exclusions, covered,
total, uncovered and decimal percentage.

The slow artifact also includes LCOV for line-level inspection. Raw JSON uses
host paths and remains an intermediate rather than the normalized evidence.

## Capability and vector mapping

The Apollo parity manifest gains a coverage table plus one mapping for every
instrumented production source file. Each mapping names at least one existing
capability and, where parity/shared-vector behavior is present, existing vector
IDs. The parity validator rejects unknown mappings, missing source files,
unmapped executable source files, disallowed exclusions, wrong tool/version,
or a threshold below Apollo's rounded 74.82% baseline.

Global coverage never substitutes for mapped vector selectors or explicit
negative tests already required by the manifest.

## CI placement and failure behavior

A dedicated Ubuntu job in the existing weekly/manual `slow` workflow runs the
script and uploads the three normalized/inspectable artifacts. Any test,
profile merge, schema, file mapping or threshold failure prevents publication.
The `fast` workflow is not edited.

## Rollback

The addition is repository-local and reversible. Removing it requires removal
of the complete coverage contract; silently lowering the threshold is rejected
by ADR 0096.
