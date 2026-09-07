## Why

ADR 0062 coupled the SDK's public compiler floor to the current Rust release
through a stable-minus-three formula. Repository measurements and sponsor
direction in issue #170 show that edition, primary validation compiler, MSRV
and forward-compatibility compiler are independent decisions.

## What Changes

- Keep Edition 2024 and the effective Rust 1.85.0 MSRV.
- Pin Rust 1.98.1 as the primary development and CI compiler.
- Retain NeoPRISM's pinned nightly as an independent etalon gate.
- Replace the automatic release-distance MSRV policy with evidence-driven
  selection and record Rust 1.89.0 as the next candidate, not a promise.
- Permit a higher crate-local MSRV only for an accepted boundary adapter with
  its own material decision and evidence.

## Capabilities

### Modified Capabilities

- `sdk-support-policy`: separates and enforces primary stable, MSRV and etalon
  toolchains.
- `dependency-research-readiness`: selects MSRV from measured consumer and
  dependency value rather than release arithmetic.

## Impact

The default Nix developer toolchain and full Rust quality/target gates move to
Rust 1.98.1. Rust 1.85.0 remains the public compiler floor. A new rust-overlay
input supplies the current stable compiler while the original NeoPRISM-aligned
overlay, nixpkgs revision and Nix version remain intact for the etalon lane.
No Rust API, wire format, dependency, supported target tier, FFI claim,
release state or downstream repository changes.
