# SD-JWT VC draft-22 stages

![SD-JWT VC stages](../diagrams/sd-jwt-vc-draft22.svg)

The architecture keeps independently meaningful evidence in distinct states.
RFC 9901 parsing does not imply a valid issuer signature; an issuer signature
does not imply a complete disclosure graph; a complete graph does not imply
draft-22 profile validity; and structurally correlated metadata or status does
not imply trust or acceptance.

Production dependencies point inward to generic credential and JOSE
semantics. OID4VCI and OID4VP participate only in executable integration
evidence, preventing protocol DTOs from becoming the credential model.
Inline and URI-discovered JWKS inputs both carry bounded resolved keys and
must contain the exact issuer-verification key; a URL alone advances no state.
Supplying a verification key alone proves a signature, not issuer-key
resolution, so the generic report keeps that stage `NotChecked` unless the
optional discovery evidence correlates successfully.
Type Metadata paths retain object-member, array-index and wildcard segments;
evidence failures remain attributed to issuer-key, schema or status stages
without overwriting a successful signature-proof stage.

The exact ownership, limits and migration trigger are maintained in the
[architecture knowledge base](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/architecture/sd-jwt-vc-draft22-profile.md).
