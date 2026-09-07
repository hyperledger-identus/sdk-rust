# ADR 0002: align the development toolchain with NeoPRISM

> **Compiler selection superseded:** ADR 0081 retains NeoPRISM's Nix baseline
> and historical provenance but uses stable Rust 1.98.1 for ordinary SDK work.
> The recorded nightly remains only a sanitizer-tooling exception during the
> temporary active-development phase.

- **Status:** Accepted by project-sponsor direction
- **Date:** 2026-09-02
- **Decision authority:** explicit project-sponsor direction; future pin changes
  remain subject to normal Identus review
- **Related work:** B01 selected-baseline stabilization

## Context

The SDK seed declared Rust 1.85 as its MSRV but selected an unversioned
`stable.latest` toolchain and a separately moving `nixos-unstable` input. That
made compiler diagnostics and developer tooling drift independently from the
established Identus Rust repository.

NeoPRISM is the toolchain etalon for Identus Rust repositories. At inspected
revision `8becb225132efb1d9302b2c5f6ed4d87b84e8685`, NeoPRISM uses:

- Rust nightly `2026-03-18` from `oxalica/rust-overlay`;
- rust-overlay revision `f17186f52e82ec5cf40920b58eac63b78692ac7c`;
- nixpkgs revision `c27cdad491a991b11ed731760aa2ef8db0cb0410`;
- Nix package version `2.34.8` from that nixpkgs revision.

The nixpkgs revision includes NeoPRISM's fix for crates.io HTTP 403 failures.

## Decision

The SDK flake SHALL use the same Rust nightly, rust-overlay revision and
nixpkgs revision as the inspected NeoPRISM baseline. The SDK keeps its own
flake structure, checks and unrelated inputs; NeoPRISM is a version reference,
not a runtime or source dependency.

The workspace `rust-version = "1.85.0"` remains the declared MSRV. Using a
nightly toolchain for reproducible development and CI does not authorize
nightly-only Rust features or raise the MSRV. A separate reviewed change is
required to alter compatibility policy.

Pins are copied from an immutable NeoPRISM revision. They do not automatically
follow NeoPRISM `main`; future synchronization is an explicit dependency
update with both repositories' revisions recorded.

## Consequences

- `nix develop` and crane checks use Rust nightly `2026-03-18` deterministically.
- The devshell's Nix package resolves to version `2.34.8` on supported systems.
- Compiler-diagnostic fixtures are evaluated against the same Rust baseline as
  NeoPRISM instead of an unbounded `stable.latest` channel.
- Plain-Cargo MSRV verification remains separate stabilization work.
- Existing SDK-specific CI hardening and flake modules remain unchanged.
