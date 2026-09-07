# ADR 0062: use a rolling near-current MSRV

- **Status:** Accepted
- **Date:** 2026-09-07
- **Issue:** [#151](https://github.com/hyperledger-identus/sdk-rust/issues/151)
- **Implementation:** [#154](https://github.com/hyperledger-identus/sdk-rust/issues/154);
  the effective repository MSRV remains Rust 1.85 until that PR merges
- **Research:** [Rust library reuse research](../research/rust-library-reuse/report-source.md)

## Context

The repository currently declares Rust 1.85 as its Minimum Supported Rust
Version (MSRV). MSRV is the oldest Rust compiler the SDK promises can compile
each supported feature surface. It is a consumer compatibility contract, not
the maintainer development toolchain.

Rust publishes a stable release every six weeks and upstream supports only the
current stable release. On 2026-09-07, current stable is Rust 1.98.1. The
existing 1.85 floor is about thirteen release trains behind and was selected as
a static bootstrap value rather than from measured consumer constraints.

A literal “average crate MSRV” is not reliable. In the current SDK lock graph,
21 of 94 external packages do not declare `rust-version`; declared values also
describe historical compatibility rather than what modern releases need. The
assessment found a real example: `multibase 0.9.3` declares no floor, but its
current `base45 3.2.0` resolution uses `slice::as_chunks`, stabilized in Rust
1.88, and therefore fails under Rust 1.85.

The SDK wants broad access to maintained identity, cryptography and protocol
crates while retaining a useful upgrade window for native, mobile, WASM and
server consumers.

## Decision

Adopt a **stable minus three releases** policy:

1. At an MSRV review, the target floor is the Rust stable minor three release
   trains behind current stable. This is approximately an 18-week compatibility
   window.
2. Reviews occur at least quarterly, but the MSRV never changes automatically.
   A focused issue and PR must update Cargo, Nix, machine-readable support
   policy, documentation and all affected locks/gates together.
3. The initial transition target is Rust **1.95.0**, because current stable is
   1.98.1 at this decision date.
4. Every declared feature surface is compiled at the MSRV. The workspace is
   also tested on current stable or the pinned etalon toolchain; a newer pass
   never substitutes for the MSRV gate.
5. The floor may remain older for one review period when a named, supported
   consumer or platform toolchain has evidence of a hard constraint. The
   exception records an owner, expiry and security cost.
6. The floor may advance sooner when a critical security fix or required
   standards dependency cannot be consumed safely otherwise. That change still
   requires all target gates and explicit release notes.
7. An MSRV increase is a compatibility event. Before 1.0 it is announced in
   the next SDK candidate/minor notes; after 1.0 the release policy must define
   its SemVer treatment before publication.

“Near-current” is used instead of “slightly above average” because it is
measurable, reproducible and tied to Rust's release train. The dependency
research still applies: a crate is not accepted merely because the newer MSRV
can compile it.

## Consequences

- The SDK can consume most actively maintained crates without carrying a
  multi-year compiler constraint.
- Consumers receive a predictable minimum upgrade window rather than surprise
  dependency failures.
- Mobile and WASM compatibility remains evidence-based because the exact floor
  is compiled for supported targets/features where the toolchain supplies them.
- Quarterly MSRV work becomes routine maintenance and may require dependency
  lock adjustments.
- Organizations requiring longer compiler qualification can pin an older SDK
  release; indefinite old-compiler support is not a project promise.
- Rust 1.89 requirements such as Spruce SSI no longer fail solely on MSRV after
  the transition, but their dependency-cone and coupling objections remain.

## Alternatives rejected

### Keep Rust 1.85 indefinitely

This maximizes theoretical compatibility but has no validated consumer need,
blocks or complicates modern dependency resolution and increases maintenance
and security backport cost.

### Always require current stable

This maximizes crate access but gives downstreams no qualification window and
would force a compiler update every six weeks.

### Use the arithmetic mean or median of crate MSRVs

MSRV metadata is incomplete and biased toward historical minimums. An average
does not predict whether the chosen current dependency resolution compiles, as
the `base45` result demonstrates.

### Give every SDK crate a different MSRV

This could optimize individual cones but creates a confusing consumer contract
and fragmented CI. A future exceptional binding or adapter crate may request a
higher floor through a separate ADR; the generic workspace remains coherent by
default.

## Rollout and rollback

The implementation issue must prove the complete workspace and target matrix
at Rust 1.95.0, update `Cargo.toml`, `sdk-support-policy.toml`, Nix toolchains
and documentation atomically, and record downstream compatibility evidence.
Until that issue merges, Rust 1.85 remains the operational promise.

If a supported consumer cannot move after the PR is opened, keep the existing
floor, record the time-bounded exception and re-evaluate dependencies. Once a
release is published with a higher MSRV, rollback requires a new compatible
release rather than rewriting the published artifact.
