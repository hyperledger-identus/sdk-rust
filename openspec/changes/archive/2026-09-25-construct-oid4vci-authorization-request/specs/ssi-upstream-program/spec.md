## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and reference open
focused child #350 after #348 delivers bounded unencrypted Deferred Credential
payload-error handling. Issue #250 delivered bounded request construction,
issue #345 delivered bounded issued/pending success responses with exact
pending transaction correlation, and issue #348 composes the existing
Credential Error parser with deferred-specific classification and lifecycle
guidance. Issue #350 owns the next authorization-code server-binding seam.

The active slice SHALL NOT claim the full OID4VCI engine, HTTP execution, RFC
6750 challenge parsing, token ownership/validation, TLS, interval scheduling,
retry/invalidation effects, request/response encryption, credential
verification/storage, proof-count correlation, format/chain extensions,
consumer adoption, publication or release.

#### Scenario: Payload-error child hands off to authorization-code binding

- **WHEN** issue #348 is delivered and focused successor #350 is open
- **THEN** `IDR-023` references #350 with `delivery_status=in_progress`
- **AND** #7 and #20 remain open component and program parents

#### Scenario: payload-error validation is not engine completion

- **WHEN** a Deferred Credential Request validates and classifies bounded
  payload errors without transport, authorization, timing or lifecycle effects
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered

Before #350 integrates, the row SHALL reference one open focused successor for
the next least-authority Authorization Code request-input seam. This handoff
does not claim request serialization, PKCE/PAR, callback mix-up protection,
HTTP execution, token exchange, credential verification/storage,
format/chain extensions, consumer adoption, publication or release.

#### Scenario: server binding hands off without claiming engine completion

- **WHEN** issue #350 is delivered and its focused successor is open
- **THEN** `IDR-023` references that successor with
  `delivery_status=in_progress`
- **AND** #7 and #20 remain open component and program parents

After #350 delivers, the row SHALL reference focused child #352. Issue #352
owns bounded offered-configuration selection, OAuth caller-input validation
and SDK-derived PKCE S256 for a later Authorization Request serializer. It
SHALL NOT claim URL serialization, scope or `authorization_details` selection,
PAR, browser/callback behavior, Authorization Response correlation, code
exchange, HTTP execution or authorization completion.

#### Scenario: server binding hands off to request-input preparation

- **WHEN** issue #350 is delivered and #352 is open
- **THEN** `IDR-023` references #352 with `delivery_status=in_progress`
- **AND** #7 and #20 remain open component and program parents

Before #352 integrates, the row SHALL reference one open focused successor for
the next least-authority Authorization Request seam.

#### Scenario: prepared inputs do not complete the engine

- **WHEN** bounded inputs and PKCE S256 are prepared without wire serialization
  or response correlation
- **THEN** the backlog remains in progress under the focused successor

After #352 delivers, the row SHALL reference focused child #354. Issue #354
owns one deterministic, bounded Authorization-Details-based request URI with
strict endpoint query retention and exact offered `issuer_state`. It SHALL NOT
claim PAR/HTTP/browser execution, Authorization Response correlation, callback
or redirect safety, code exchange, authorization completion, consumer adoption,
publication or release.

#### Scenario: request inputs hand off to deterministic construction

- **WHEN** issue #352 is delivered and #354 is open
- **THEN** `IDR-023` references #354 with `delivery_status=in_progress`
- **AND** #7 and #20 remain open component and program parents

Before #354 integrates, the row SHALL reference one open focused successor for
the next least-authority Authorization Response correlation seam.

#### Scenario: request construction does not complete authorization

- **WHEN** a bounded deterministic request URI exists without response parsing
  or correlation
- **THEN** the backlog remains in progress under the focused successor
