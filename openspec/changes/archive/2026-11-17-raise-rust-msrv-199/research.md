# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-11-17
Source retrieval date: 2026-11-17
Research blockers: none

## Problem and existing implementation

The current implementation has the workspace and release candidate declaring
Rust 1.89.0 while primary and
etalon validation use Rust 1.98.1. ADR 0133 freezes that split for `0.1.x` and
requires a later pre-1.0 minor line, focused ADR, migration guidance, and
complete profile/target evidence before any higher floor is effective.

Natural scheduled run `43792452831` at protected `develop`
`f7e777b8556ba4c82bdc272faf2551e7bfb9d8f8` independently failed the macOS
UniFFI generator audit. The isolated lock contains `rustix 1.1.4`, which is
affected by RUSTSEC-2026-0486; RustSec identifies `>=1.1.6` as patched.

## Normative sources

- Sponsor direction:
  https://github.com/hyperledger-identus/sdk-rust/issues/596#issuecomment-6952649191
- Rust 1.99.1 release:
  https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/
- RustSec advisory:
  https://rustsec.org/advisories/RUSTSEC-2026-0486.html
- Candidate overlay revision:
  `oxalica/rust-overlay@ec07af44be2e0ce4e1a17e00a97d44232607c1ec`
- NeoPRISM consumer revision:
  `hyperledger-identus/neoprism@755fba1b1625acc795340d8514a7af1f9490d1e4`
- SDK base:
  `hyperledger-identus/sdk-rust@f7e777b8556ba4c82bdc272faf2551e7bfb9d8f8`

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Rust 1.99.1 for MSRV, primary, and etalon | `adopt` | Exact sponsor direction, smallest corrected 1.99 point release, one compiler identity, and current downstream compiler compatibility. | A reproducible compiler defect or required consumer proves incompatibility before integration. |
| Rust 1.99.0 | `not-adopt` | Superseded by the 1.99.1 point release and adds no compatibility value over the accepted 1.99 line. | A supported target cannot obtain 1.99.1 artifacts but can obtain 1.99.0 safely. |
| Rust 1.100.0 | `defer` | Current stable is newer but was released only five days before this decision and exceeds the requested floor. | A focused later toolchain issue measures a concrete benefit. |
| Keep Rust 1.89.0 MSRV | `not-adopt` | Conflicts with sponsor direction and retains a second compiler/compatibility burden. | A named consumer requires the old line and the project explicitly reopens compatibility policy. |
| Patch `rustix` to `>=1.1.6` | `adopt` | This is the RustSec patched range and changes only the isolated generator tool lock. | UniFFI resolves a different non-affected dependency cone. |

## Compatibility and dependency evidence

Raising MSRV is a deliberate source-compatibility break for Rust 1.89 through
1.98 consumers. ADR 0200 therefore moves the next unpublished candidate to the
`0.2.x` line and supplies migration guidance; it does not rewrite historical
0.1 receipts.

NeoPRISM `main` at the recorded revision consumes the prior immutable SDK
candidate and pins nightly `2026-09-02`, locally identified as
`rustc 1.100.0-nightly`. It is compiler-compatible with Rust 1.99.1 manifests,
although a later SDK revision refresh remains downstream-owned. Apollo does not
consume the Rust SDK and has no Rust compiler-floor impact. SDK-TS consumes
generated WASM/package artifacts and does not define the Rust compiler floor.
No downstream repository was mutated.

The exact candidate overlay commit contains stable Rust 1.99.0 and 1.99.1
artifacts under the existing pinned nixpkgs baseline. It replaces only the
separately named stable overlay; NeoPRISM provenance and sanitizer-nightly pins
remain independent history/tooling evidence.

The exact compiler version is Rust 1.99.1 with the existing workspace feature
matrix unchanged. The direct dependency set is unchanged; the resolved
dependency cone changes only in the isolated UniFFI generator lock where
`rustix` advances from 1.1.4 to a patched `>=1.1.6` release. Existing Cargo
license and provenance policy remains authoritative. The change introduces no
unsafe or native-code surface. Public API and wire compatibility are unchanged;
the compatibility break is limited to the declared compiler floor. The facade
boundary remains the existing SDK-owned Rust and generated adapter APIs.

## Security, privacy and maintenance evidence

The RustSec issue is safe-code-reachable undefined behavior in `rustix` xattr
buffer handling. The UniFFI generator is a build tool rather than SDK runtime
code, but supply-chain policy correctly rejects the affected lock. Updating the
lock to `>=1.1.6` removes the advisory without an ignore or waiver.

The compiler change adds no runtime crate, unsafe block, secret flow, wire
format, network authority, or personal data. Maintenance becomes one exact
ordinary compiler identity plus the separate sanitizer nightly. Nix and Cargo
locks retain immutable source hashes.

Before integration, rollback is a normal signed revert of the policy,
toolchain, and lock changes. After a later 0.2 publication, rollback requires a
new corrective version because published package metadata is immutable. The
maintenance, release and security posture is re-reviewed before that separate
activation. Protocol or draft currency is not applicable because this slice
changes toolchain and package compatibility policy, not an SSI protocol or
draft implementation.

## Rejected or deferred candidates

Rust 1.99.0 is rejected because 1.99.1 is the corrected point release. Rust
1.100.0, automatic latest-stable tracking, a dual 1.89/1.99 support matrix, and
release publication are deferred. A future issue may reconsider a newer
compiler only with measured target and consumer evidence; this slice does not
publish, tag, promote `main`, or mutate downstream source.

## Evidence commands

- Focused support-policy, constraint, release-candidate, source-distribution,
  crypto-candidate, and supply-chain mutation suites.
- Exact Rust 1.99.1 workspace build/test/Clippy/doc and declared feature/target
  gates through Nix.
- `scripts/check-uniffi-generator-supply-chain.py`.
- `scripts/factory check`, readiness, receipt, archive, and full
  `nix flake check` on the available host.
- Hosted Linux fast CI and exact-head stack review after push.

Unrun or host-incompatible evidence will be named explicitly. A green newer
compiler does not relabel historical evidence.

## Open questions and blockers

None. The sponsor direction resolves the material compatibility choice. A
failing target artifact or named consumer probe would become an implementation
blocker rather than a reason to weaken the declared floor silently.
