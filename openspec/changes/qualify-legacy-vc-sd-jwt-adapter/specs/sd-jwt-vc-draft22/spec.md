## ADDED Requirements

### Requirement: legacy draft-03 migration is explicit and non-canonical

The SDK SHALL accept `vc+sd-jwt` only through a versioned, input-only adapter
bound to `draft-ietf-oauth-sd-jwt-vc-03` and the named SDK-TS 8.1.4 consumer.
The adapter SHALL require an issuer-verified and disclosure-processed RFC 9901
state, exact legacy envelope and protected identity, and the bounded semantic
subset proven common with draft-22. It SHALL reject draft-specific ambiguity,
including legacy status behavior, `aka_vcts`, `vct#integrity`, Type Metadata,
and millisecond NumericDate heuristics. A successful legacy state SHALL remain
type-distinct from `Draft22SdJwtVc`, SHALL report that canonical reissuance is
required, and SHALL expose no legacy issuance, automatic fallback, protected
header rewriting, or canonical conversion under the original signature.

#### Scenario: caller explicitly validates a compatible legacy credential

- **WHEN** the caller selects the draft-03 adapter for an exact `vc+sd-jwt` envelope whose issuer signature, disclosures, common claims, placement, dates, and resource bounds are valid
- **THEN** the SDK returns a redaction-safe legacy migration state that requires canonical reissuance and cannot be represented as draft-22

#### Scenario: canonical path receives legacy identity

- **WHEN** an exact `vc+sd-jwt` credential is passed to the canonical draft-22 envelope parser or profile validator
- **THEN** it remains rejected without invoking the migration adapter

#### Scenario: legacy input contains draft-specific semantics

- **WHEN** legacy input contains status, additional credential types, integrity metadata, Type Metadata, or a millisecond-only date representation
- **THEN** the adapter fails with a stable value-free draft-mismatch category before representing the credential as compatible

#### Scenario: canonical output is produced after adapter activation

- **WHEN** any credential is issued through the supported SD-JWT VC issuance API
- **THEN** its protected type remains exactly `dc+sd-jwt`, its envelope/media identity remains canonical, and no API emits `vc+sd-jwt`

#### Scenario: the migration window ends

- **WHEN** the named released consumer and supported stored credential populations have measured migration evidence and no supported deployment requires legacy input for the agreed window
- **THEN** a separate consumer-visible change removes the adapter without changing RFC 9901 or canonical draft-22 semantics
