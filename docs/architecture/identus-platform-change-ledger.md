# Identus platform change ledger

This document is generated from
`docs/architecture/identus-platform-change-ledger.toml`. Do not edit it by
hand.

Registry version: `2.0.0`<br>
Status: `active`

## Release notes

### `did.prism.portable-method.v1`

State: `proposed`. Class: `additive` / `added`. Target: `unreleased`. The additive identus-prism-did crate constructs and validates exact bounded PRISM operations, authenticates and reduces DID state, and projects generic DID documents under a separate native output budget without chain coupling.

### `did.syntax.long-form-resource-policy.v1`

State: `proposed`. Class: `behavioral` / `changed`. Target: `unreleased`. Generic DID and URI values admit the measured finite long-form envelope while the independent method and cache limits remain bounded; a valid oversized cache identity bypasses retention observably.

## Migration actions

### `did.prism.portable-method.v1`

`unreleased` → `unreleased` for neoprism, sdk-swift, sdk-ts. No migration action required.

### `did.syntax.long-form-resource-policy.v1`

`0.1.0-rc.1` → `unreleased` for sdk-rust-adopters. Use the exported constants rather than copied literals, handle KeyBudgetBypass as an uncached successful resolution path, and keep external transport limits explicit.

## Compatibility windows

### `did.prism.portable-method.v1`

Compatibility: `compatible`. First compatible: `unreleased`. Last compatible: `unreleased`. No compatibility window required.

### `did.syntax.long-form-resource-policy.v1`

Compatibility: `compatible`. First compatible: `unreleased`. Last compatible: `0.1.0-rc.1`. The release-candidate line documents the new limits before a stable 0.1.0 contract is declared.

## Rollback and removal

### `did.prism.portable-method.v1`

Fallback: Consumers remain on their existing PRISM implementation until this experimental crate and their own canary are accepted. Rollback: Remove the additive dependency or revert the consuming adapter; existing SDK-Rust crates retain their previous behavior. Removal gate: Not applicable.

### `did.syntax.long-form-resource-policy.v1`

Fallback: Bypass cache retention for valid oversized identities; do not truncate keys or fall back to a second parser. Rollback: Restore the prior lexical constants and cache outcome before publication while retaining issue and measurement evidence. Removal gate: Not applicable; this is an expansion of a bounded public envelope rather than a deprecated facade.

## Limitations

### `did.prism.portable-method.v1`

Visibility: behavior, public-api, security, target-support, wire. The 32 MiB native projection budget does not widen generic hostile JSON ingress. Non-terminal retired identifiers are capped at 4096 per kind; exceeding that profile is a distinct resolution-failing resource outcome, while terminal deactivation discards obsolete tombstones. No Cardano indexing, VDR payload interpretation, persistence, HTTP, CLI, node composition, custody, product policy or completed downstream adoption is claimed.

### `did.syntax.long-form-resource-policy.v1`

Visibility: behavior, public-api. The lexical envelope does not guarantee transportability through every host, cache retention, or acceptance by a method-specific semantic profile.
