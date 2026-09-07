# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-07
Source retrieval date: 2026-09-07
Research blockers: none

## Problem and existing implementation

The current implementation uses Edition 2024, declares Rust 1.85.0, and runs its full Nix
gate on NeoPRISM's `nightly-2026-03-18`. ADR 0062 proposed Rust 1.95.0 solely
from a stable-minus-three formula. The machine contract has no independent
current-stable lane.

## Normative sources

Issue https://github.com/hyperledger-identus/sdk-rust/issues/170 records the
accepted sponsor direction. Rust's 1.98.1 release announcement records the
1.98.0 vtable-generation miscompilation and corrective point release. Cargo's
official documentation records that Edition 2024 was stabilized with Rust
1.85. Repository ADRs 0002, 0062 and 0063 govern the existing etalon, MSRV
proposal and material activation rules.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| Existing Nix/Crane gate architecture | `origin/develop@8ed0d2017e1411016f6660f0efe454c14ce6be60` | `retain-local` | It already supplies declarative, machine-checked providers and gates. | Replace only if it cannot express an independently pinned compiler class. |
| Current stable compiler | Rust 1.98.1 plus a pinned rust-overlay revision | `adopt` | Current stable validation catches stable-only compatibility and avoids the 1.98.0 miscompilation. | Advance only through a focused issue after a new stable patch is evaluated. |
| NeoPRISM etalon | `nightly-2026-03-18`, NeoPRISM `8becb225132efb1d9302b2c5f6ed4d87b84e8685` | `retain-local` | NeoPRISM still uses `#![feature(error_reporter)]`; this is independent integration evidence. | Reconsider after NeoPRISM removes the nightly requirement and updates its etalon. |
| Public MSRV | Rust 1.85.0 | `retain-local` | The full locked graph passes and no supported consumer requires a higher floor. | A required maintained dependency or named consumer produces measured value for an uplift. |
| Next MSRV candidate | Rust 1.89.0 | `conditional-adopt` | It is available in current pins and admits known 1.88/1.89 ecosystem candidates. | Activate only after supported target and downstream compatibility evidence passes. |
| rustup CI installation | Current channels | `not-adopt` | It bypasses the reproducible flake and exact overlay provenance. | Reconsider only if Nix cannot supply a supported target or signed Rust artifact. |

## Compatibility and dependency evidence

Against a clean archive of `origin/develop`, `cargo check --workspace
--all-targets --all-features --locked` passed with Rust 1.85.0, 1.89.0 through
the pinned Nix overlay, 1.90.0 and 1.95.0. The lock graph contains 94 external
packages: 73 declare `rust-version`, 21 omit it, and the maximum declared value
is 1.85.0. The current overlay resolves 1.85 and 1.89, but not 1.95 or 1.98.1;
its latest stable is 1.94. NeoPRISM at the recorded revision uses
`#![feature(error_reporter)]`, so its nightly remains a real separate lane.
The direct and resolved dependency cone is unchanged because the added flake
input is a build-tool source, not a Cargo dependency. Its exact revision,
license and provenance are locked from `oxalica/rust-overlay`; no dependency
type crosses the Identus public facade. Public and wire compatibility are
unchanged because no Rust API, serialized value or protocol behavior changes.

## Security, privacy and maintenance evidence

Rust 1.98.1 avoids the compiler miscompilation documented for 1.98.0. The
change adds no runtime crate, native library, unsafe code, secret processing or
personal data. Two overlay revisions add update work, bounded by explicit
machine policy, lockfile pins and drift tests. A newer compiler passing never
substitutes for the Rust 1.85 MSRV gates.
Supply-chain evidence consists of the exact flake lock revision, GitHub source
provenance and existing Nix hash verification. Maintenance and release remain
repository-controlled; the compiler change does not publish an SDK artifact.
Protocol/draft currency is not applicable because no protocol profile changes.

## Rejected or deferred candidates

- Rust 1.95 MSRV: rejected as an automatic target because the current graph
  and named consumers provide no evidence for the larger compatibility break.
- Rust 1.89 MSRV activation: deferred until cross-target and downstream probes
  show concrete dependency value; it remains a target.
- Current stable as MSRV: rejected because it gives consumers no qualification
  window.
- Dioxus-driven MSRV: rejected because Dioxus belongs to Oxid, not the generic
  SDK.
- Workspace-wide uplift for future UniFFI: rejected; an accepted boundary
  adapter may request a crate-local exception later.
- Replacing the NeoPRISM nightly: deferred because NeoPRISM still requires a
  nightly feature and downstream changes are out of scope.

Rollback removes the primary overlay/provider and restores ordinary gates to
the existing etalon provider; the effective Rust 1.85 contract is unchanged.
The objective reconsideration trigger for the two-overlay design is a
NeoPRISM etalon that supplies a supported current stable compiler without
nightly-only SDK requirements.

## Open questions and blockers

None for this policy and validation-lane change. Rust 1.89 activation, FFI,
iOS simulator and Android x86_64 support remain separate future decisions.

## Evidence commands

Commands run: `scripts/factory doctor`; `cargo +1.85.0 check --workspace
--all-targets --all-features --locked`; the same command with Rust 1.90.0 and
1.95.0; a Nix-provided Rust 1.89.0 exact check; focused support-policy,
constraint and factory suites; and `nix flake check --print-build-logs` with
all 30 compatible local checks passing. The full run exercised Rust 1.98.1,
Rust 1.85.0, the NeoPRISM nightly, WASM, Android and iOS compilation on the
aarch64-Darwin host. Unrun checks: Nix omitted x86_64-Linux locally as an
incompatible system; required hosted CI remains the independent Linux result
and is not represented as passing here.
