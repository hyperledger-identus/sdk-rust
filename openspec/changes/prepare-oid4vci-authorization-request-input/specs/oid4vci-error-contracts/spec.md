# oid4vci-error-contracts Specification

## MODIFIED Requirements

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
