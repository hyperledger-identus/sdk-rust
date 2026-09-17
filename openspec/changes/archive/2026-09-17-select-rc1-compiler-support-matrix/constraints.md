# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/325
Constraint blockers: none

## Existing entries affected

`SDK-COMPAT-002` through `SDK-COMPAT-005`, `SDK-LIM-009`, ADR 0081, the
machine support policy, and the unpublished candidate descriptor currently
describe a temporary single-compiler phase that cannot authorize publication.
`SDK-ARCH-001`, `SDK-LIM-002`, `SDK-LIM-003`, `SDK-LIM-005`, and
`SDK-LIM-006` continue to keep chain/product behavior, FFI, runtime support,
and downstream adoption outside the generic SDK release.

## Introduced or changed constraints

The `0.1.0-rc.1` release train declares Rust 1.89.0 as its MSRV and Rust
1.98.1 as its pinned primary/etalon compiler. The fast PR lane remains one
Rust 1.98.1 Linux signal. MSRV, target, complete-feature, dependency, and
security evidence runs in weekly/manual slow and exact-candidate release
paths. The candidate cannot publish unless both compilers and every promised
profile/target pass on the frozen revision.

An MSRV increase is a compatibility change. It cannot occur within the
`0.1.x` release line and requires a focused decision, measured payoff,
migration guidance, and complete evidence for a later minor line.

## Introduced or changed limitations

Linux x86_64 and macOS ARM64 are source/host-tested. Browser WASM, Android
ARM64, and iOS ARM64 are compile-only. No FFI, runtime, device, packaging,
secure-storage, application-framework, performance, certification, Windows,
or WASI support is introduced.

## Consumer and product impact

Rust consumers can rely on a tested 1.89.0 compiler floor for the three
candidate crates. Consumers retain ownership of platform, chain, wallet,
custody, trust, and deployment decisions. No consumer repository or primitive
is imported into SDK source.

## Activation and rollback

Activation requires issue-bound preflight, primary and MSRV local evidence,
policy mutation tests, candidate packaging, signed/DCO PR, green fast CI, and
complete frozen-SHA slow evidence before publication. Rollback restores the
single Rust 1.98.1 release-ineligible policy and blocks publication.

## Evidence

Issue #325 and the sponsor-provided toolchain recommendation are the exact
material direction. #326 remains the protected release-approval record.

