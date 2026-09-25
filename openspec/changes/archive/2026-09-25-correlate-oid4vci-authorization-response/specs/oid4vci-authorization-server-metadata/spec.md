# oid4vci-authorization-server-metadata Specification

## ADDED Requirements

### Requirement: RFC 9207 support is retained with its exact default

The bounded partial Authorization Server Metadata core SHALL recognize
`authorization_response_iss_parameter_supported` only as a boolean, retain
whether it was advertised, and expose an effective value of false when
omitted. Existing unknown-field, duplicate, aggregate-resource and exact issuer
validation SHALL remain unchanged.

#### Scenario: omission defaults false

- **WHEN** metadata omits the RFC 9207 support member
- **THEN** advertised support is absent and effective support is false

#### Scenario: type confusion fails closed

- **WHEN** the support member is null, string, number, array or object
- **THEN** metadata parsing returns a static invalid-metadata failure
