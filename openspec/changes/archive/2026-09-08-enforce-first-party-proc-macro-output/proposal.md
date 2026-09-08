## Why

The workspace unsafe forbid protects authored first-party targets, but Rust
1.98.1 may skip direct procedural-macro output whose expansion span allows
internal unsafe. `identus-derive` can close its direct first-party portion of
that gap before returning generated items, without a new tool or CI lane.

## What Changes

- Add a post-composition syntax-tree guard that rejects unsafe constructs and
  unsafe attributes in direct `Newtype` output.
- Add stable unit evidence for every rejected construct, nested syntax and safe
  output; retain the existing finite-shape runtime/trybuild regression suite.
- Record corrected ADR 0088 and narrow `SDK-LIM-008` to external and nested
  procedural-macro expansion output.
- Preserve existing generated spans and the exact Rust 1.98.1 etalon.

## Capabilities

### Modified Capabilities

- `unsafe-code-policy`: Adds bounded machine evidence for direct first-party
  procedural-macro output and states the residual expansion boundary.

## Impact

This is a material security-assurance improvement with no public API, wire,
runtime, package, target or MSRV change. It enables the existing stable `syn`
dependency's `visit` feature, changes build-time validation in
`identus-derive`, tests, the constraint record and governance evidence. Issue
#189 is the decision authority. Activation requires the existing derive suite,
complete compatible Nix checks and hosted Linux gates to pass.
