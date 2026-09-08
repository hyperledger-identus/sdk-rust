# DID Resolution HTTP delta

## MODIFIED Requirements

### Requirement: Bounded composable DID resolution route

The SDK SHALL provide an unpublished outer-boundary Axum router with fixed
`GET /{did}` routing and all resolver state applied. Construction SHALL accept
an `Arc<dyn DidResolver>` and SHALL NOT accept a dynamic path, start a server
or expose private handler/state/dependency types. Consumers SHALL select an
external service prefix by nesting the returned router.

Invalid path extraction and invalid DID syntax SHALL return a standard W3C
`invalidDid` result with HTTP 400. Query options SHALL be decoded through the
bounded GET contract before resolver invocation. Caller-controlled input SHALL
NOT appear in errors.

#### Scenario: Consumer nests the state-closed router

- **WHEN** a consumer nests the router and requests a valid encoded DID
- **THEN** the injected resolver receives the validated DID without additional state wiring or dynamic route parsing

#### Scenario: Invalid path input fails as a W3C result

- **WHEN** path decoding or DID validation fails
- **THEN** the response is HTTP 400 with a bounded `application/did-resolution` error result and no echoed input

### Requirement: Resolution projection preserves W3C state

For `application/did-resolution`, the adapter SHALL serialize the complete
result and leave `ResolutionOptions.accept` absent. For a negotiated DID
document representation, it SHALL pass that media type in
`ResolutionOptions.accept` and serialize only the document after verifying
that result metadata contains the same content type. Every other decoded query
option SHALL be preserved in both projection modes. Missing or mismatched
success fields SHALL become a standard internal-error HTTP 500 result.

Standard result errors SHALL retain the pinned W3C status mapping. Deactivated
metadata SHALL map to 410. Error/deactivated bodies SHALL always be complete
results with `application/did-resolution`. Every response SHALL carry
`Vary: Accept`.

#### Scenario: Full result preserves non-accept options

- **WHEN** `application/did-resolution` and valid query options are supplied
- **THEN** the resolver receives no `accept`, receives every other typed option, and HTTP 200 returns the complete result

#### Scenario: Document representation merges options

- **WHEN** a document representation and valid query options are supplied
- **THEN** the resolver receives that exact `accept` plus every query option and HTTP 200 returns the matching document

#### Scenario: Invalid success projection fails closed

- **WHEN** a document response lacks a document or matching metadata content type, or bounded serialization unexpectedly fails
- **THEN** the adapter does not emit a mislabeled success and returns a static/redacted internal failure

## ADDED Requirements

### Requirement: Strict bounded GET resolution options

The adapter SHALL map every GET query parameter except `accept` into
`ResolutionOptions`. It SHALL recognize exact common names
`expandRelativeUrls`, `noCache`, `versionId` and `versionTime`; unknown names
SHALL be preserved as string-valued extensions. `accept` SHALL be controlled
only by the HTTP header.

Before or during parsing, the adapter SHALL enforce 8 KiB raw query, 32
parameters, 256 decoded-name bytes and 4,096 decoded-value bytes. It SHALL
split raw structure before exactly one strict percent-decoding pass, preserve
literal `+`, and reject missing `=`, empty/control-bearing names, control-
bearing values, malformed escapes, invalid UTF-8 and duplicate decoded names.

Booleans SHALL accept only `true` or `false`; versions SHALL use existing typed
validators; `versionId` and `versionTime` SHALL NOT occur together. Invalid,
ambiguous, conflicting and excessive queries SHALL return a redacted W3C
`invalidOptions` HTTP 400 result without invoking the resolver.

#### Scenario: Common and method options reach the resolver

- **WHEN** a bounded valid query contains common fields and an extension
- **THEN** the resolver receives exact typed common values and the exact decoded extension string once

#### Scenario: URI query semantics are preserved

- **WHEN** a value contains a literal plus or percent-encoded delimiters
- **THEN** plus remains plus and decoded delimiters remain inside the original scalar without creating parameters

#### Scenario: Ambiguous or malformed query fails closed

- **WHEN** input is duplicate, conflicting, invalidly encoded, invalidly typed, control-bearing or missing required structure
- **THEN** the resolver is not called and HTTP 400 contains only the static standard error result

#### Scenario: Resource ceilings fail at the boundary

- **WHEN** raw bytes, member count, decoded name bytes or decoded value bytes exceed the exact ceiling
- **THEN** parsing stops with the same redacted 400 response before resolver invocation
