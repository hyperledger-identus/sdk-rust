# Verification evidence

Verified on 2026-09-03 from `codex/r0-boundary-support-policy`, based on
`origin/develop` at `a3eed8717035ab7d18eadd78977bb4e60d20b61a`.

## Contract evidence

| Evidence | Result |
| --- | --- |
| GitHub coordination | Focused issue #22 was created before implementation |
| Program scope | `IDR-002` and `IDR-003` only; `IDR-001` and later component rows remain open |
| Dependency boundary | Nine synthetic/real conformance tests cover aliases, package identity, all table/override kinds, sources, paths and closure |
| Compatibility source | One TOML contract plus bounded human explanation |
| Toolchains | Rust `1.85.0` is independent from NeoPRISM-etalon nightly `2026-03-18` |
| Compile-only targets | Browser WASM, Android ARM64 and iOS ARM64 build the four implemented portable packages |
| Deferred claims | FFI is unsupported; size and build time are measurement-only |

## Passing gates

| Gate | Result |
| --- | --- |
| `scripts/check-support-policy.py` | Passed the canonical Cargo/Nix/policy contract |
| `scripts/tests/support-policy.py` | Passed 17 positive and adversarial drift tests |
| `scripts/tests/ssi-upstream-backlog.py` | Passed 6 canonical backlog tests |
| `cargo test -p identus-conformance` | Passed all 20 conformance tests |
| `scripts/tests/factory-contract.sh` | Passed support, backlog, PR and factory contract suites |
| `scripts/factory validate enforce-sdk-boundary-support-policy` | Strict validation passed |
| `scripts/factory check` | Passed current specs and the complete active change |
| `git diff --check` | Passed |
| `nix flake check --print-build-logs` | Passed all 27 compatible `aarch64-darwin` checks |

The full Nix gate includes formatting; Nix/TOML/text hygiene; factory/OpenSpec;
Rust `1.85.0` across all seven declared feature surfaces; nightly clippy, docs
and dependency policy; the 113-test default run and 49-test KMP crypto run;
isolated feature surfaces; and real release build commands for
`wasm32-unknown-unknown`, `aarch64-linux-android` and `aarch64-apple-ios`.

## Environment diagnostics

- The local full gate omitted incompatible `x86_64-linux` outputs. Hosted CI
  remains the Linux host confirmation.
- The offline dependency-policy derivation emits non-blocking unmatched-license
  allowances already present in the repository configuration.
- Nix emitted known non-fatal macOS `audit-tmpdir.sh` segmentation warnings
  during some artifact fixup phases; all affected derivations completed.

## Read-only repository boundary

The change uses the immutable NeoPRISM etalon revision
`8becb225132efb1d9302b2c5f6ed4d87b84e8685` recorded in ADR 0002. Apollo,
NeoPRISM, midnight-identity, Lace ID Portal and Oxid remained evidence-only:
no donor or consumer file was switched, staged, edited, copied or built. Their
locally observed pre-existing/unrelated dirty and untracked states were left
untouched.
