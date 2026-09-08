## ADDED Requirements

### Requirement: OID4VCI uses private RFC 3986 grammar beneath protocol policy

OID4VCI SHALL use exact `fluent-uri 0.4.1` privately for its absolute URI and
URI-reference grammar checks with default features disabled. Existing field
byte bounds and visible-character rules SHALL run before or compose with the
grammar engine. All Identus-owned wrappers, exact retained spelling, stable
errors and redaction behavior SHALL remain unchanged.

Absolute HTTPS identifiers and endpoints SHALL continue requiring an
ASCII-case-insensitive `https` scheme, present authority, non-empty host, no
userinfo, no fragment and the existing call-site-specific query rule. Token
Error `error_uri` and URI-shaped token types SHALL use URI-reference grammar
without importing normalization, resolution, IRI or trust behavior.

#### Scenario: HTTPS protocol policy survives grammar replacement

- **WHEN** an absolute URI has a non-HTTPS scheme, empty host, userinfo, forbidden query or fragment
- **THEN** the same existing OID4VCI error is returned without exposing the input or candidate diagnostic

#### Scenario: IPvFuture is parsed directly

- **WHEN** a standards-valid IPvFuture HTTPS host is presented at an allowed OID4VCI boundary
- **THEN** the original value is accepted directly without host substitution, normalization or a second parse

#### Scenario: URI references remain strict and exact

- **WHEN** a bounded Token Error `error_uri` or URI-shaped token type contains a valid relative or absolute URI reference
- **THEN** its exact spelling is retained while malformed percent escapes, forbidden characters and invalid field shapes fail closed

#### Scenario: Legacy runtime parser leaves the protocol cone

- **WHEN** the OID4VCI normal dependency graph is inspected
- **THEN** `uriparse`, `fnv` and `lazy_static` are absent unless another independently accepted runtime consumer is documented
