# Identus platform change ledger

This document is generated from
`docs/architecture/identus-platform-change-ledger.toml`. Do not edit it by
hand.

Registry version: `2.0.0`<br>
Status: `active`

## Release notes

### `crypto.secp256k1.rustcrypto-security-stack.v1`

State: `active`. Class: `security` / `security`. Target: `unreleased`. The same SDK wrappers, methods and byte behavior use advisory-clean k256 and p256 0.14; their exposed tuple fields consequently have the corresponding 0.14 upstream type identities.

### `did.peer.profiles.v1`

State: `active`. Class: `additive` / `added`. Target: `unreleased`. The additive identus-did-peer crate creates, parses and locally resolves bounded numalgo-4 current-profile and explicit numalgo-2 compatibility DIDs from caller-owned public material.

### `did.prism.portable-method.v1`

State: `active`. Class: `additive` / `added`. Target: `unreleased`. The additive identus-prism-did crate constructs and validates exact bounded PRISM operations, authenticates and reduces DID state, and projects generic DID documents under a separate native output budget without chain coupling.

### `did.prism.vdr-indexing.portable-core.v1`

State: `active`. Class: `additive` / `added`. Target: `unreleased`. The additive crates parse and authorize bounded PRISM VDR operations, resolve finite ancestry, plan complete ordered transactions, and expose replay-safe atomic commit ports without chain or persistence effects.

### `did.syntax.long-form-resource-policy.v1`

State: `active`. Class: `behavioral` / `changed`. Target: `unreleased`. Generic DID and URI values admit the measured finite long-form envelope while the independent method and cache limits remain bounded; a valid oversized cache identity bypasses retention observably.

### `oid4vp.direct-post-response.error.v1`

State: `active`. Class: `additive` / `added`. Target: `unreleased`. The additive transition consumes validated request lineage and explicit typed protocol-error values into a bounded plain direct-post error envelope.

### `oid4vp.direct-post-response.v1`

State: `active`. Class: `additive` / `added`. Target: `unreleased`. The additive transition consumes validated request lineage and bounded caller presentations into a deterministic plain direct-post success envelope.

## Migration actions

### `crypto.secp256k1.rustcrypto-security-stack.v1`

`0.1.0-rc.1` → `unreleased` for sdk-rust-adopters. Prefer identus-crypto from_slice, to_public_key and encoding methods; direct tuple-field consumers update their k256 or p256 dependency and imports to 0.14 when adopting the next candidate.

### `did.peer.profiles.v1`

`unreleased` → `unreleased` for sdk-rust-adopters, sdk-swift, sdk-ts. No migration action required.

### `did.prism.portable-method.v1`

`unreleased` → `unreleased` for neoprism, sdk-swift, sdk-ts. No migration action required.

### `did.prism.vdr-indexing.portable-core.v1`

`unreleased` → `unreleased` for neoprism, sdk-rust-adopters. No migration action required.

### `did.syntax.long-form-resource-policy.v1`

`0.1.0-rc.1` → `unreleased` for sdk-rust-adopters. Use the exported constants rather than copied literals, handle KeyBudgetBypass as an uncached successful resolution path, and keep external transport limits explicit.

### `oid4vp.direct-post-response.error.v1`

`unreleased` → `unreleased` for lace-id-portal, oxid, sdk-rust-adopters. No migration action required.

### `oid4vp.direct-post-response.v1`

`unreleased` → `unreleased` for lace-id-portal, oxid, sdk-rust-adopters. No migration action required.

## Compatibility windows

### `crypto.secp256k1.rustcrypto-security-stack.v1`

Compatibility: `source-break`. First compatible: `unreleased`. Last compatible: `0.1.0-rc.1`. The migration is declared on the release-candidate line before stable 0.1.0; the next candidate carries the advisory-clean type identities.

### `did.peer.profiles.v1`

Compatibility: `compatible`. First compatible: `unreleased`. Last compatible: `unreleased`. No compatibility window required.

### `did.prism.portable-method.v1`

Compatibility: `compatible`. First compatible: `unreleased`. Last compatible: `unreleased`. No compatibility window required.

### `did.prism.vdr-indexing.portable-core.v1`

Compatibility: `compatible`. First compatible: `unreleased`. Last compatible: `unreleased`. No compatibility window required.

### `did.syntax.long-form-resource-policy.v1`

Compatibility: `compatible`. First compatible: `unreleased`. Last compatible: `0.1.0-rc.1`. The release-candidate line documents the new limits before a stable 0.1.0 contract is declared.

### `oid4vp.direct-post-response.error.v1`

Compatibility: `compatible`. First compatible: `unreleased`. Last compatible: `unreleased`. No compatibility window required.

### `oid4vp.direct-post-response.v1`

Compatibility: `compatible`. First compatible: `unreleased`. Last compatible: `unreleased`. No compatibility window required.

## Rollback and removal

### `crypto.secp256k1.rustcrypto-security-stack.v1`

Fallback: Delay consumer adoption while migrating to SDK byte boundaries; do not publish a new security-current candidate that restores the rejected 0.13 closure. Rollback: Move forward to another advisory-clean curve family or SDK-owned opaque boundary; never restore the advisory-affected dependency closure as a release state. Removal gate: Not applicable; this is a security remediation and candidate source migration, not a deprecated compatibility facade.

### `did.peer.profiles.v1`

Fallback: Consumers retain their existing peer DID implementation until an explicit adapter or canary adopts the additive crate. Rollback: Remove the additive package dependency before publication; generic SDK-Rust DID behavior remains unchanged. Removal gate: Not applicable.

### `did.prism.portable-method.v1`

Fallback: Consumers remain on their existing PRISM implementation until this experimental crate and their own canary are accepted. Rollback: Remove the additive dependency or revert the consuming adapter; existing SDK-Rust crates retain their previous behavior. Removal gate: Not applicable.

### `did.prism.vdr-indexing.portable-core.v1`

Fallback: Consumers remain on their existing PRISM storage and indexing implementation until an explicit downstream canary accepts these additive crates. Rollback: Remove the additive crate dependencies before publication; existing SDK-Rust crates and downstream implementations retain their prior behavior. Removal gate: Not applicable.

### `did.syntax.long-form-resource-policy.v1`

Fallback: Bypass cache retention for valid oversized identities; do not truncate keys or fall back to a second parser. Rollback: Restore the prior lexical constants and cache outcome before publication while retaining issue and measurement evidence. Removal gate: Not applicable; this is an expansion of a bounded public envelope rather than a deprecated facade.

### `oid4vp.direct-post-response.error.v1`

Fallback: Consumers continue constructing error forms in their existing adapter until they explicitly adopt the experimental SDK transition. Rollback: Remove the additive error-response types and constructor before publication; existing request validation and success construction remain available. Removal gate: Not applicable.

### `oid4vp.direct-post-response.v1`

Fallback: Consumers continue assembling responses in their existing adapter until they explicitly adopt the experimental SDK transition. Rollback: Remove the additive response transition and private retained metadata before publication; existing request validation remains available. Removal gate: Not applicable.

## Limitations

### `crypto.secp256k1.rustcrypto-security-stack.v1`

Visibility: public-api, security. Wire values and SDK-owned methods remain compatible, but direct construction or access through the four public tuple fields is source-incompatible until the consumer adopts RustCrypto 0.14 or the SDK byte boundary.

### `did.peer.profiles.v1`

Visibility: behavior, public-api, security, target-support, wire. No custody, private keys, key generation, networking, persistence, DIDComm engine, agent runtime, language adapter, consumer mutation, publication or historical numalgo 0, 1 and 3 support is included. The 300-second Linux fuzz receipt remains issue-owned slow-lane evidence until a successful immutable run is attached.

### `did.prism.portable-method.v1`

Visibility: behavior, public-api, security, target-support, wire. The 32 MiB native projection budget does not widen generic hostile JSON ingress. Non-terminal retired identifiers are capped at 4096 per kind; exceeding that profile is a distinct resolution-failing resource outcome, while terminal deactivation discards obsolete tombstones. No Cardano indexing, VDR payload interpretation, persistence, HTTP, CLI, node composition, custody, product policy or completed downstream adoption is claimed.

### `did.prism.vdr-indexing.portable-core.v1`

Visibility: behavior, public-api, security, target-support, wire. No Cardano source, SQL repository, HTTP or CLI surface, runtime, retry loop, deployment, device execution, FFI, crates.io publication, or completed NeoPRISM adoption is included. The 300-second Linux fuzz receipt remains issue-owned slow-lane evidence until a successful immutable run is attached.

### `did.syntax.long-form-resource-policy.v1`

Visibility: behavior, public-api. The lexical envelope does not guarantee transportability through every host, cache retention, or acceptance by a method-specific semantic profile.

### `oid4vp.direct-post-response.error.v1`

Visibility: behavior, public-api, security, target-support, wire. Only plain OpenID4VP 1.0 Final direct_post protocol-error construction is included; no automatic SDK error mapping, HTTP, consent/privacy policy, protected response, DC API, consumer mutation or publication is claimed.

### `oid4vp.direct-post-response.v1`

Visibility: behavior, public-api, security, target-support, wire. Only plain OpenID4VP 1.0 Final direct_post success construction is included; no HTTP execution, presentation generation or verification, consent, trust, replay defense, signing, encryption, direct_post.jwt, SIOPv2, DC API, HAIP, persistence, consumer mutation or publication is claimed.
