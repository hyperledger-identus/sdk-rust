## MODIFIED Requirements

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
