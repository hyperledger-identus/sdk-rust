# Decompose OID4VCI resource-limit policy

## Why

`crates/oid4vci/src/limits.rs` contains 1,445 authored nonblank lines and 23
public limit types spanning Credential Offer, token, credential-response, and
metadata lifecycles. Discussion #399 and issue #404 identify the concentration
as security-sensitive: smaller ownership units are useful only if all numeric
policy remains centrally discoverable and exact behavior remains unchanged.

## What changes

- Keep a private `limits` facade and the existing crate-root public exports.
- Single-own every default numeric value and the configurable JSON-depth
  ceiling in one private policy module.
- Group public limit types and their validation/accessor mechanics into four
  private lifecycle modules: offer, token, credential, and metadata.
- Preserve every public path, type, constructor, accessor, trait
  implementation, default value, validation rule, error, feature, and target.
- Prove equivalence through existing exact/one-over tests, immutable error
  evidence, code-health evidence, strict Rust gates, and protected CI.

## Capability

### Added capability

- `oid4vci-resource-limit-policy`: records centralized numeric policy,
  lifecycle ownership, and compatibility requirements for OID4VCI limits.

## Non-goals

This change does not alter a limit, accepted input, error precedence, public
API, serialized form, transport obligation, or `SDK-LIM-007`; introduce a
generic limit framework or macro; add a dependency; or refactor consumers.

## Delivery

Issue #404 owns this slice. Planning and the exact-head factory preflight
precede implementation. A pre-move OID4VCI suite establishes the
characterization baseline; post-move tests, exact API inspection, code-health
regeneration, and protected CI prove the result.
