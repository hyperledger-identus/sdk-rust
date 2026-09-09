# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-09
Source retrieval date: 2026-09-09
Research blockers: none

## Problem and existing implementation

Apollo `main@ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` reports
74.81865284974093% line coverage in Coveralls build 79551970. SDK-Rust has no
coverage runner. A local Rust 1.98.1 probe of `identus-crypto --all-features`
with `cargo-llvm-cov` 0.9.0 measured 1,481 covered of 1,722 first-party source
lines, or 86.00464576074333%, before this change. Test counts are deliberately
not compared because Kotlin and Rust test functions are not equal denominators.

## Normative sources

- Rust's official [instrumentation coverage guide](https://github.com/rust-lang/rust/blob/48a229cea63b66a1e788736a951007c2cbfb332a/src/doc/rustc/src/instrument-coverage.md)
  defines stable `-C instrument-coverage`, unique raw profiles and the matching
  `llvm-profdata`/`llvm-cov` flow.
- [`cargo-llvm-cov` 0.9.0](https://github.com/taiki-e/cargo-llvm-cov/tree/be59056988acd54c7f984b7c85643daea3711b29)
  is Apache-2.0 OR MIT, declares Rust 1.87, drives the compiler-matched LLVM
  tools and exports JSON, LCOV and HTML. The locked Nix input already packages
  exactly 0.9.0.
- [`cargo-tarpaulin` 0.37.2](https://github.com/xd009642/tarpaulin/tree/8a92ab675457ec41a71ee4ef860204dafde53b77)
  is Apache-2.0 OR MIT. Its default backend is Linux x86_64 ptrace, LLVM is a
  separate engine, and its default feature cone includes Coveralls, git2 and
  WASM support.
- [`grcov` 0.10.7](https://github.com/mozilla/grcov/tree/6efbc1d7604a22fae6ba145d1c3637f0bec7b1e6)
  is MPL-2.0 and aggregates raw LLVM profiles into many report formats, but it
  requires the repository to orchestrate instrumentation, binary discovery and
  LLVM version matching separately.

Version, license, provenance, declared MSRV and direct-dependency evidence was retrieved
from `cargo info --verbose`, official repositories and the locked Nix package
set on 2026-09-09.

## Candidate decisions

| Candidate | Decision | Evidence and fit | Reconsideration trigger |
| --- | --- | --- | --- |
| `cargo-llvm-cov` 0.9.0 | `adopt` | Rust 1.87 MSRV is below the 1.98.1 etalon; Apache-2.0/MIT; compiler-native coverage; multi-run profile merge; JSON/LCOV output; no production graph edge. | The tool cannot consume profiles emitted by the pinned compiler or loses maintained Nix packaging. |
| Raw rustc + LLVM tools | `not-adopt` | Zero additional binary, but the repository would own Cargo target discovery, proc-macro filtering, profile merge and LLVM path/version orchestration already handled by the selected wrapper. | The wrapper becomes unmaintained or materially misreports the Rust denominator. |
| Tarpaulin 0.37.2 | `not-adopt` | Provides reports, but ptrace/LLVM engine differences and its broader default cone add host and maintenance coupling without better evidence for this stable compiler campaign. | A required coverage mode is unavailable through compiler-native LLVM coverage. |
| grcov 0.10.7 | `not-adopt` | Mature aggregator and cross-platform formats, but adds a second orchestration layer and MPL-2.0 tooling obligation while cargo-llvm-cov already produces the needed formats. | Coverage from several languages or non-Cargo binaries must be merged. |
| Hosted Codecov/Coveralls service | `not-adopt` | Trend UI is useful, but introduces tokens/service policy and is unnecessary for downloadable Actions artifacts. | Maintainers authorize a hosted service and retention/privacy policy. |
| Nightly branch coverage | `not-adopt` | Apollo exposes only a line denominator and ADR 0081 makes nightly sanitizer-only. | Both projects publish comparable reproducible branch denominators on a selected stable toolchain. |

## Compatibility and dependency evidence

The current implementation has no coverage dependency. The selected tool is
Nix-only maintainer/CI tooling. It does not enter `Cargo.toml`,
`Cargo.lock`, crate metadata, public features, target artifacts or consumer
dependency cones. Its direct dependency cone contains 19 Rust crates and the
normal resolved host-tool graph contains 48 packages including the root. That
resolved dependency cone is owned by the pinned Nix derivation rather than the
SDK lockfile. Rust 1.98.1 gains the matching `llvm-tools-preview`
component. The four feature commands write to one clean profile directory;
the final report counts only regular files below `crates/crypto/src`.

Public and wire compatibility is unchanged, and the facade boundary remains
`identus-crypto`: no cargo-llvm-cov type, report schema or LLVM type appears in
the crate API. The Nix package's supply-chain revision and hash remain locked
by `flake.lock`/nixpkgs; no install-at-latest action or downloaded CI binary is
used. Its host target executes only in the Ubuntu slow job, while the SDK
feature and portable-target policy remains unchanged.

## Security, privacy and maintenance evidence

Coverage instruments tests and first-party source only. Reports contain source
paths, line numbers and hit counts, never runtime arguments or raw secrets.
The normalizer rejects files outside the declared crate source root, missing
metadata, tool/version drift, duplicate files and a denominator below 74.82%.
No first-party error, unsafe or secret-handling source is excluded.

The tool's own dependency cone may contain unsafe or native host code as any
Nix executable can; it is not linked into SDK artifacts and executes only as a
CI/maintainer process. Maintenance and release posture is bounded to the locked
0.9.0 package and explicit updates. Security review treats report paths as
untrusted inputs to the normalizer. Protocol or draft currency is not
applicable to compiler coverage; the only comparison is Apollo's pinned line
denominator, not a changing SSI protocol draft.

## Rejected or deferred candidates

Raw orchestration, Tarpaulin, grcov, hosted services and nightly branch
coverage are not adopted for the reasons and objective reconsideration
triggers in the candidate table. Rollback removes the tooling, LLVM component,
runner, workflow job and manifest contract together; it does not migrate a
consumer or library dependency.

## Open questions and blockers

None. Hosted artifacts are retained as CI evidence; a later threshold reduction
requires a superseding ADR rather than an ad hoc workflow edit.

## Evidence commands

- `cargo info cargo-llvm-cov@0.9.0 --verbose`
- `cargo info cargo-tarpaulin@0.37.2 --verbose`
- `cargo info grcov@0.10.7 --verbose`
- `nix eval` against the locked flake input for `cargo-llvm-cov.version`
- Four-profile local LLVM coverage probe on Rust 1.98.1
- Unrun before implementation: final Nix flake check, hosted Ubuntu slow job,
  artifact upload and post-implementation mutation suite.
