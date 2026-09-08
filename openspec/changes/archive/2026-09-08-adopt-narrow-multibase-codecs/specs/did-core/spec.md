## MODIFIED Requirements

### Requirement: Open public verification method boundary

A `VerificationMethod` SHALL contain a typed `Uri` id, a bounded non-empty
open type string, a typed controller `Did` and bounded suite-defined
properties. DID-shaped ids MAY be parsed separately as `DidUrl` by
downstream method adapters. The capability SHALL recognize and expose
`publicKeyJwk` maps and `publicKeyMultibase` strings while preserving other
properties without cryptosuite interpretation.

Recognized `publicKeyMultibase` SHALL be no larger than 4 KiB before
decoding, use only the `z` base58-btc or `u` unpadded-base64url prefix,
decode to non-empty bytes and be the canonical re-encoding of those bytes.
Exact `bs58 0.5.1` with defaults disabled and alloc only plus the existing
`base64 0.22` engine SHALL implement base conversion behind an
Identus-private dispatcher. Upstream types/errors SHALL NOT enter public
signatures or diagnostics. Successful validation SHALL NOT infer multicodec,
key type, curve, key length or cryptographic validity.

The capability SHALL reject simultaneous recognized material forms and all
registered private JWK members. The five core relationship properties SHALL
contain one or more embedded verification methods and/or typed `Uri`
references.

#### Scenario: embedded and referenced relationships coexist

- **WHEN** a document assigns embedded and referenced methods across authentication, assertion, key agreement, capability invocation and capability delegation
- **THEN** the exact relationship variants and order SHALL remain available without imposing curve, controller or chain policy

#### Scenario: secret or ambiguous known material is rejected

- **WHEN** a JWK contains private material or a verification method contains both `publicKeyJwk` and `publicKeyMultibase`
- **THEN** every construction path SHALL fail before the document is exposed

#### Scenario: interoperable canonical multibase carriers are retained

- **WHEN** recognized verification material contains a canonical non-empty `z` base58-btc or `u` unpadded-base64url value within the 4 KiB encoded limit
- **THEN** native and JSON construction SHALL retain its exact string and expose it through the existing accessor

#### Scenario: malformed or unsupported multibase fails closed

- **WHEN** recognized verification material has an empty payload, invalid alphabet, padding, non-canonical spelling, unknown prefix or exceeds the encoded limit
- **THEN** native and JSON construction SHALL return the existing redacted invalid-document error without exposing upstream diagnostics

#### Scenario: carrier validity does not claim key validity

- **WHEN** a canonical allowed multibase value decodes successfully
- **THEN** the DID Core boundary SHALL NOT claim that its bytes contain a supported multicodec, curve, key length or cryptographically valid public key
