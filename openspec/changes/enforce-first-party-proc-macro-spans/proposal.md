## Why

The workspace unsafe forbid protects authored first-party targets, but Rust
1.98.1 may skip procedural-macro output whose expansion span allows internal
unsafe. `identus-derive` can close the direct first-party portion of that gap
with stable caller-origin spans and no additional tool or CI lane.

## What Changes

- Make every direct `Newtype` expansion template use the derived type's caller
  span through `quote_spanned!`.
- Add stable, dependency-free compile-fail evidence that caller-spanned unsafe
  expansion output is rejected by the inherited workspace forbid.
- Record ADR 0088 and narrow `SDK-LIM-008` to external, nested and otherwise
  ungoverned procedural-macro expansion output.
- Preserve all existing macro semantics and the exact Rust 1.98.1 etalon.

## Capabilities

### Modified Capabilities

- `unsafe-code-policy`: Adds bounded compiler evidence for direct first-party
  procedural-macro output and states the residual expansion boundary.

## Impact

This is a material security-assurance improvement with no public API, wire,
runtime, dependency, feature, target or MSRV change. It changes build-time
token spans in `identus-derive`, conformance tests, the constraint record and
governance evidence. Issue #189 is the decision authority. Activation requires
the existing derive suites, complete compatible Nix checks and hosted Linux
gates to pass.
