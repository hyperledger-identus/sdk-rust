# oid4vci-error-contracts Specification

## Purpose
TBD - created by archiving change decompose-oid4vci-error-contracts. Update Purpose after archive.
## Requirements
### Requirement: OID4VCI owns a private multi-kind error contract

`identus-oid4vci` SHALL own a crate-private compile-time record containing
exactly stable code, error kind, and one static message. The `oid4vci`
capability SHALL remain centralized. No dependency, feature, allocation,
runtime input, unsafe/native code, serialization, FFI, retryability, public
API, wire model, or protocol behavior SHALL be introduced.

#### Scenario: varying kind is explicit without repeating invariants

- **WHEN** any current `CredentialOfferError` is converted
- **THEN** its record supplies code/kind/message and shared conversion supplies capability

### Requirement: Catalogue ownership follows OID4VCI responsibilities

Records SHALL be grouped privately into offer transport/JSON, offer
semantics/grants, issuer/authorization-server metadata, token
request/response/errors, credential/nonce/HTTP, and deferred/immediate issuance
catalogues with counts 12, 21, 39, 34, 36, and 29. Their ordered ranges SHALL
be `InvalidLimits` through `UnsafeReferenceUri`, `InvalidSemanticLimits`
through `TransactionCodeDescriptionTooLarge`, `InvalidMetadataLimits` through
`TokenEndpointRequired`, `InvalidTransactionCodeInputLimits` through
`TokenErrorUriTooLarge`, `InvalidCredentialErrorResponseLimits` through
`CredentialRequestBodyTooLarge`, and
`InvalidDeferredCredentialRequestLimits` through
`CredentialResponseExceedsProofCount`.

#### Scenario: one protocol responsibility is reviewable alone

- **WHEN** a reviewer inspects credential, nonce, and HTTP diagnostics
- **THEN** all 36 records are visible without reading offer, metadata, token, or issuance records

### Requirement: Routing and inventory are compile-exhaustive

One private wildcard-free router SHALL map every `CredentialOfferError`
variant to exactly one record. The same private list SHALL produce only its
test inventory and SHALL NOT generate a public declaration, constant, wire
model, or Serde implementation. No code, kind, capability, or message literal
SHALL occur in the router.

#### Scenario: an unmapped variant fails

- **WHEN** a variant is added without a record/router entry
- **THEN** compilation fails rather than selecting an implicit contract

### Requirement: Exact pre-refactor behavior is immutable

The compatibility oracle SHALL be a planning golden captured from
`develop@6217384f85ff72003a7b94482bf7879f4587be02` with exactly 171 unique
ordered rows and SHA-256
`2c9e03381744b11902eb8b8bbc08934374781fad99d6561573cfcd62490bcd39`.
It SHALL pin constant identity/visibility, code, kind, capability,
local/public/full display, discriminant order, and source. Stable and planning
copies SHALL be byte-identical, fixed-hash, and receipt-blob bound. Ambiguity,
absence, schema/type drift, coordinated mutation, path escape, or symlinked
components SHALL fail closed.

#### Scenario: every current error remains exact

- **WHEN** the refactored catalogue is compared with the golden
- **THEN** all 171 rows remain ordered, public, exact-message, source-free, and distributed as 167 InvalidInput plus four Unsupported

### Requirement: Public, wire, and protocol boundaries remain unchanged

The public OID4VCI error surface SHALL remain exact, including enum order,
derives, non-exhaustive marker, `CAPABILITY`, all public constants/paths,
`From`, `Display`, `Error`, and `pub const fn to_identus_error`. Existing typed
protocol error responses, Serde/wire shapes, parsers, bounds, transports,
metadata, token, credential, nonce, and issuance behavior SHALL remain exact.
Issues #7 and #168 SHALL receive no behavioral delta.

#### Scenario: compatibility diff is empty

- **WHEN** base and head public/dependency/protocol inventories are compared
- **THEN** no public item, constness, wire behavior, manifest, feature, dependency, lockfile, canonical protocol specification, #7, or #168 delta exists

### Requirement: Portable-target evidence remains bounded

The crate SHALL compile through the existing direct and authoritative Nix
WASM, Android AArch64, and iOS AArch64 gates. This evidence SHALL NOT create a
runtime, device, packaging, FFI, binding, React, or React Native support
promise.

#### Scenario: a target build passes

- **WHEN** an OID4VCI portable-target build succeeds
- **THEN** the result is reported as compile-only evidence with no broader support claim

### Requirement: Maintainability evidence is truthful

The change SHALL report largest bridge/router/catalogue, mapping sites,
wildcard defaults, decision count, error-contract lines, and whole-crate
production lines. It SHALL retain 171 behavioral decisions, one mapping site,
and zero wildcard defaults, cap catalogues at 39 records, and disclose every
total-line movement without claiming decision deduplication or compression.

#### Scenario: cohesion improves without false compression

- **WHEN** before/after evidence is reviewed
- **THEN** six responsibility catalogues are independently discoverable, the public bridge is a small delegator, and all 171 decisions remain explicit
