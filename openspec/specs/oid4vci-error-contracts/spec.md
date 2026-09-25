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
request/response/errors, credential/nonce/HTTP, deferred/immediate issuance,
and authorization-code server-binding catalogues with live counts 12, 21, 39,
34, 36, 34, and 4. The first six ordered ranges SHALL remain
`InvalidLimits` through `UnsafeReferenceUri`, `InvalidSemanticLimits` through
`TransactionCodeDescriptionTooLarge`, `InvalidMetadataLimits` through
`TokenEndpointRequired`, `InvalidTransactionCodeInputLimits` through
`TokenErrorUriTooLarge`, `InvalidCredentialErrorResponseLimits` through
`CredentialRequestBodyTooLarge`, and
`InvalidDeferredCredentialRequestLimits` through
`DeferredCredentialTransactionMismatch`. The seventh SHALL range from
`AuthorizationCodeGrantMissing` through `AuthorizationEndpointRequired`.

The five issuance-catalogue suffix records SHALL remain the issue #345
fieldless HTTP limits, status, media-type bound, media-type syntax and
transaction correlation errors. The four authorization-code records SHALL be
the issue #350 fieldless grant absence, server-hint mismatch, unsupported
grant and endpoint absence diagnostics. All SHALL preserve the immutable
171-variant v1 prefix and receive independent exact code, kind, message,
conversion and redaction tests.

#### Scenario: authorization-code diagnostics remain cohesive

- **WHEN** a reviewer inspects Authorization Code server binding
- **THEN** all four appended records are visible in one focused catalogue
  without widening the full metadata catalogue
- **AND** no catalogue exceeds the standing 39-record review ceiling

Issue #352 SHALL extend the seventh focused catalogue with ten request-input
diagnostics, changing its live count to 14 and its terminal range to
`PkceCodeVerifierTooLarge` while keeping it below 39 records. The router SHALL
remain one explicit wildcard-free decision site.

#### Scenario: request preparation remains in the focused catalogue

- **WHEN** a reviewer inspects server binding and request preparation
- **THEN** their static errors are discoverable together without widening
  unrelated metadata, token or issuance catalogues

Issue #354 SHALL extend the same catalogue with seven request-construction
diagnostics, changing its live count to 21 and its terminal range to
`AuthorizationRequestUriTooLarge` while keeping it below 39 records.

#### Scenario: request construction remains in the focused catalogue

- **WHEN** a reviewer inspects the complete Authorization Code request seam
- **THEN** its 21 static errors remain in one cohesive catalogue and the router
  remains one explicit wildcard-free decision site

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

The authorization-code server-binding capability SHALL append static public
rows for an absent Authorization Code grant, an Authorization Code
server-hint mismatch, unsupported Authorization Code grant capability, and a
missing Authorization Endpoint. Existing ordered rows and every existing
row's variant, constant, code, category, message, Display text and help URL
SHALL remain exact.

#### Scenario: new server-binding diagnostics are append-only

- **WHEN** the authorization-code server-binding capability is delivered
- **THEN** its four diagnostics follow the previous canonical inventory rows
- **AND** the historical prefix remains byte-for-byte semantically unchanged

The Authorization Request input capability SHALL append static public rows for
invalid limits, an absent configuration index, invalid or oversized client
identifier, redirect URI and state, and invalid or oversized PKCE verifier.
Every prior live row SHALL remain exact, and no new diagnostic SHALL retain or
display remote input.

#### Scenario: request-input diagnostics append safely

- **WHEN** issue #352 extends the live error inventory
- **THEN** all request-input rows follow every pre-existing row
- **AND** the v1 golden and prior live rows remain semantically exact

The Authorization Request construction capability SHALL append static public
rows for invalid limits, oversized Authorization Details, excessive endpoint
query fields, oversized or invalid endpoint query components, duplicate or
reserved query names, and an oversized final request URI. Every prior live row
SHALL remain exact, and no new diagnostic SHALL retain or display request data.

#### Scenario: request-construction diagnostics append safely

- **WHEN** issue #354 extends the live error inventory
- **THEN** all seven construction rows follow every pre-existing row
- **AND** the v1 golden and prior live rows remain semantically exact

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
MAY grow only through append-only independently tested feature contracts.
Seven responsibility catalogues SHALL remain discoverable, no catalogue SHALL
exceed the standing 39-record review ceiling, and every total-line or
record-count movement SHALL be disclosed without claiming decision
deduplication.

#### Scenario: cohesion survives additive evolution

- **WHEN** a new error contract is appended
- **THEN** its responsibility catalogue remains independently reviewable, the
  router remains one explicit wildcard-free decision site, and all baseline
  plus live decisions remain explicit

### Requirement: Authorization Response diagnostics are static append-only contracts

The Authorization Response capability SHALL append unique public error codes
and static messages for invalid limits, encoded size/count/components,
malformed form/structure, duplicate parameters, state correlation, issuer
correlation, invalid branch and invalid/oversized code/error roles. A focused
private catalogue SHALL own these contracts and the central router SHALL make
every mapping explicit without a wildcard.

#### Scenario: remote values never enter diagnostics

- **WHEN** a response is rejected for any new reason
- **THEN** Debug, Display and bridged public errors contain no query, code,
  state, issuer, description, URI or extension value

#### Scenario: existing contracts remain immutable

- **WHEN** the new catalogue is appended
- **THEN** every baseline and prior live error keeps its exact order, code,
  kind, capability and message

### Requirement: Authorization Code Token Request diagnostics are append-only

The Authorization Code Token Request capability SHALL append unique fieldless
public diagnostics for invalid non-positive limits, an oversized request Token
Endpoint, and an oversized encoded request body. One focused private catalogue
SHALL own the records and the central wildcard-free router SHALL map every
variant explicitly.

Every prior baseline and live error SHALL preserve its exact order,
discriminant, code, kind, capability, message, Display output and help URL.
No diagnostic SHALL retain or display an endpoint, body, code, verifier,
redirect URI, client identifier or remote value.

#### Scenario: new errors remain static and redacted

- **WHEN** limits or request construction reject input
- **THEN** the returned error is an append-only static contract containing no
  rejected value

#### Scenario: the historical inventory stays exact

- **WHEN** the three request diagnostics are appended
- **THEN** every prior exhaustive inventory row remains semantically exact and
  the router has no wildcard fallback

### Requirement: Authorization Code Token HTTP diagnostics are append-only

The response-binding capability SHALL append unique fieldless public
diagnostics for invalid non-positive limits, unsupported HTTP status, a `401`
status/error mismatch,
oversized/invalid Content-Type, oversized/invalid Cache-Control, and
oversized/invalid Pragma. One focused private catalogue SHALL own the records
and the central wildcard-free router SHALL map every variant explicitly.

Every prior baseline and live error SHALL preserve its exact order,
discriminant, code, kind, capability, message, Display output and help URL. No
diagnostic SHALL retain or display status-associated content, a header, body,
token, request secret, endpoint, lineage or remote value.

#### Scenario: response validation stays static and redacted

- **WHEN** limits, status or headers reject a response
- **THEN** the error is an append-only static contract containing no rejected
  value

#### Scenario: historical inventory stays exact

- **WHEN** the nine response diagnostics are appended
- **THEN** every prior exhaustive inventory row remains semantically exact and
  the router has no wildcard fallback
