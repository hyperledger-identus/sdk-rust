# ADR 0045: project OID4VCI Authorization Server Metadata explicitly

- **Status:** Accepted for `develop`
- **Date:** 2026-09-06
- **Related:** issues #7, #20, and #119; ADRs 0041–0044

## Context

OpenID4VCI uses OAuth Authorization Server Metadata to let a wallet locate
authorization/token endpoints and understand advertised grants. The
interoperable Oxid/Lace Pre-Authorized Code profile carries the required
`issuer`, token endpoint, grant list, and OID4VCI anonymous-access flag, but it
does not include every member required for a complete RFC 8414 metadata
document.

Treating that consumer shape as fully RFC 8414-valid would weaken the standard
and create an unsafe public claim. Rejecting the shape entirely would also lose
useful, already interoperable protocol evidence before the full validator and
execution engine exist.

## Decision

Add `AuthorizationServerMetadataCore` as an explicitly partial, bounded,
lossless projection in `identus-oid4vci`.

The core validates a required RFC 8414 HTTPS issuer against a caller-supplied
expected identifier using simple string comparison. It optionally validates
HTTPS authorization/token endpoints, a non-empty unique grant list, and the
OID4VCI anonymous Pre-Authorized Code boolean. It preserves omission and
exposes the RFC 8414 grant defaults plus the OID4VCI false default without
inventing advertised values.

Success proves only those field shapes and the exact issuer binding. It does
not prove complete RFC 8414 conformance, retrieval authenticity, trust,
endpoint capability, grant selection, or authorization. Exact JSON and unknown
members are retained; all inputs are bounded, zeroized, and redacted.

## Consequences

- Existing consumer shapes are expressible without becoming a standards
  conformance claim.
- Later grant-selection and token-message slices can consume stable typed
  prerequisites while full RFC 8414 validation remains independently scoped.
- The `Core` suffix and documentation are security boundaries and must remain
  until a complete validator owns every required and conditional rule.
- The dependency cone and supported target matrix are unchanged.

## Alternatives rejected

- **Call the projection complete RFC 8414 metadata:** rejected because required
  response-type and conditional relationships are not validated.
- **Require the full RFC document in this slice:** rejected as a larger task
  that would conflate OID4VCI prerequisites with general OAuth conformance.
- **Copy the consumer parser:** rejected because it permits loopback HTTP and
  embeds application endpoint policy inappropriate for the generic SDK.
