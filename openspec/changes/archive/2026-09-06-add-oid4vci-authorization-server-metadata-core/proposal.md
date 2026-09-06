# Add OID4VCI Authorization Server Metadata core

## Why

Issue #117 established exact agreement between a Credential Offer and unsigned
Credential Issuer Metadata. The wallet next needs a bounded view of each
advertised OAuth Authorization Server before a later slice can select a grant
or construct a token request.

The interoperable Oxid/Lace profile carries only the Authorization Server
fields needed by its Pre-Authorized Code flow and omits other fields required
for a complete RFC 8414 document. The SDK therefore needs an explicitly named
partial projection that preserves this evidence without overstating standards
conformance.

## What changes

- Add bounded, exact JSON parsing for an Authorization Server Metadata core
  with exact expected-issuer binding.
- Expose optional safe authorization/token endpoints, explicit or effective
  grant types, and the OID4VCI anonymous Pre-Authorized Code flag.
- Preserve all unknown metadata losslessly while rejecting duplicate names,
  type confusion, unsafe URLs, ambiguous arrays, and resource exhaustion.
- Make omission/default state explicit and document that this type does not
  validate a complete RFC 8414 document.

## Non-goals

This change does not construct discovery URLs, fetch metadata, validate HTTP
responses or signed metadata, establish trust, validate complete RFC 8414
metadata, parse response types/client authentication/JWK/scope/PKCE fields,
select a server or grant, invoke an endpoint, build authorization/token
messages, enforce replay policy, edit a consumer, publish, release, or promote
to `main`.

## Impact

- **Issue:** #119, child of #7 and #20 / `IDR-023`.
- **Owner:** existing unpublished `identus-oid4vci` crate.
- **Compatibility:** additive experimental API; no wire output or released
  compatibility commitment.
- **Dependencies:** unchanged normal dependency cone; no async, HTTP, crypto,
  DID, storage, chain, platform, or product dependency.
- **Rollback:** remove the additive core type, parser, limits, errors, tests,
  ADR, and roadmap entry while retaining the four earlier OID4VCI slices.
