# Bind generated gate mapping

## Why

Post-merge review of issue #24 and PR #60 proved that the support-policy
validator accepts an arbitrary `map` body between `manifest.gates` and
`generatedChecks`. A constant mapped name can collapse 23 gates into one, and
a detached mapped value can stop using `makeGate`, while structural validation
still reports success.

## What changes

- Require every generated attribute name to come from the current manifest
  gate's `name` field.
- Require every generated attribute value to come from `makeGate gate`.
- Require the manifest binding, mapping helpers, generated mapping, and
  published result to belong to the same immediate `perSystem` `let` scope.
- Add exact fail-closed regressions for constant-name collapse and detached
  mapped values, helper replacement, and nested input shadowing.
- Record the contract and verification evidence without changing the gate
  manifest, toolchains, support claims, Rust APIs, or downstream repositories.

## Capabilities

### Modified capabilities

- `sdk-support-policy`: bind the Nix generator's mapped names and values to
  each manifest entry before structural validation can pass.

## Impact

The change is limited to repository factory validation, its tests, and
contract evidence. The canonical Nix output does not change. No public or wire
compatibility, dependency, security, release, chain, or consumer behavior is
affected.
