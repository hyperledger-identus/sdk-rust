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
catalogues with live counts 12, 21, 39, 34, 36, and 34. Their ordered ranges
SHALL be `InvalidLimits` through `UnsafeReferenceUri`, `InvalidSemanticLimits`
through `TransactionCodeDescriptionTooLarge`, `InvalidMetadataLimits` through
`TokenEndpointRequired`, `InvalidTransactionCodeInputLimits` through
`TokenErrorUriTooLarge`, `InvalidCredentialErrorResponseLimits` through
`CredentialRequestBodyTooLarge`, and
`InvalidDeferredCredentialRequestLimits` through
`DeferredCredentialTransactionMismatch`.

The five issuance-catalogue suffix records SHALL be the issue #345 fieldless
HTTP limits, status, media-type bound, media-type syntax and transaction
correlation errors. They SHALL preserve the immutable 171-variant v1 prefix and
receive independent exact code, kind, message, conversion and redaction tests.

#### Scenario: request-bound deferred HTTP diagnostics remain cohesive

- **WHEN** a reviewer inspects deferred issuance HTTP response validation
- **THEN** all five appended records are visible in the deferred/immediate
  issuance catalogue without reading unrelated protocol catalogues
- **AND** the complete live catalogue remains below the 39-record review ceiling

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

The compatibility oracle SHALL remain the planning golden captured from
`develop@6217384f85ff72003a7b94482bf7879f4587be02` with exactly 171 unique
ordered rows and SHA-256
`2c9e03381744b11902eb8b8bbc08934374781fad99d6561573cfcd62490bcd39`.
It SHALL pin baseline constant identity/visibility, code, kind, capability,
local/public/full display, discriminant order, and source. Stable and planning
copies SHALL remain byte-identical, fixed-hash, and receipt-blob bound.

The 171 baseline variants SHALL be the exact ordered prefix of the complete
live exhaustive inventory. Every live inventory entry SHALL be unique. A new
variant SHALL append after the existing inventory and SHALL NOT rewrite the v1
fixture. Ambiguity, baseline insertion/reorder/removal, schema/type drift,
coordinated fixture mutation, path escape, or symlinked components SHALL fail
closed.

#### Scenario: every baseline error remains exact after additive evolution

- **WHEN** the live catalogue is compared with the v1 golden
- **THEN** the first 171 entries remain ordered, public, exact-message,
  source-free, and distributed as 167 InvalidInput plus four Unsupported
- **AND** any live suffix contains only unique append-only variants

### Requirement: Public, wire, and protocol boundaries remain unchanged

The v1 OID4VCI error surface SHALL remain exact, including its enum prefix,
derives, non-exhaustive marker, `CAPABILITY`, all baseline public
constants/paths, `From`, `Display`, `Error`, and
`pub const fn to_identus_error`. A later issue-scoped feature MAY append a
fieldless variant, public stable code and exhaustive router record only when it
preserves every baseline discriminant and contract and independently tests the
new contract.

Existing typed protocol error responses and Serde/wire shapes SHALL remain
unchanged unless the feature's own specification explicitly governs a delta.
No feature may use this evolution rule to weaken redaction, stable-code,
resource-bound or compatibility requirements.

#### Scenario: additive error leaves the baseline exact

- **WHEN** a feature appends an independently tested fieldless error contract
- **THEN** all v1 public items, discriminants, codes, kinds, messages,
  conversions and wire behavior remain exact

### Requirement: Portable-target evidence remains bounded

The crate SHALL compile through the existing direct and authoritative Nix
WASM, Android AArch64, and iOS AArch64 gates. This evidence SHALL NOT create a
runtime, device, packaging, FFI, binding, React, or React Native support
promise.

#### Scenario: a target build passes

- **WHEN** an OID4VCI portable-target build succeeds
- **THEN** the result is reported as compile-only evidence with no broader support claim

### Requirement: Maintainability evidence is truthful

The historical refactor SHALL retain its 171 baseline behavioral decisions,
one mapping site and zero wildcard defaults. The complete live decision count
MAY grow only through append-only independently tested feature contracts. Six
responsibility catalogues SHALL remain discoverable, no catalogue SHALL exceed
the standing 39-record review ceiling, and every total-line or record-count
movement SHALL be disclosed without claiming decision deduplication.

#### Scenario: cohesion survives additive evolution

- **WHEN** a new error contract is appended
- **THEN** its responsibility catalogue remains independently reviewable, the
  router remains one explicit wildcard-free decision site, and all baseline
  plus live decisions remain explicit
