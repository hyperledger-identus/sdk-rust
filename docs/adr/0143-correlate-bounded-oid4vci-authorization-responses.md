# ADR 0143: correlate bounded OID4VCI Authorization Responses

- **Status:** Accepted
- **Date:** 2026-09-25
- **Issue:** [#356](https://github.com/hyperledger-identus/sdk-rust/issues/356)
- **Decision authority:** ADR 0083, ADR 0102, ADR 0142 and issue #356

## Context

Issue #354 constructs a deterministic Authorization Request and retains the
state, selected Authorization Server and PKCE lineage needed after a browser or
native adapter returns control. The reusable crate must correlate the returned
protocol fields without owning callback routing, accepting a full URI, or
mistaking an OAuth error for a local parser failure.

RFC 9207 issuer identification is optional metadata. Treating an absent support
flag as verified mix-up protection would overstate evidence; accepting an
unexpected `iss` would silently change the selected-server policy. OAuth form
responses also admit encoded-name aliases, so duplicate checks must happen
after strict decoding.

## Decision

1. Consume `AuthorizationRequest` exactly once and accept only an
   already-extracted query component from the registered callback adapter.
2. Apply independent positive bounds to the encoded query, parameter count,
   decoded names, generic values, code, error, description and URI roles.
3. Strictly form-decode every unique field, reject empty names and duplicate
   decoded names, and validate then discard bounded unknown extensions.
4. Require a non-empty returned state equal byte-for-byte to the retained
   request state before constructing any usable outcome.
5. Project `authorization_response_iss_parameter_supported` from selected
   Authorization Server Metadata with the RFC 9207 false default. When true,
   require an exact selected-server `iss`; when false or omitted, require
   `iss` to be absent.
6. Preserve closed evidence as `VerifiedRfc9207` or `NotAdvertised`; the latter
   is explicitly not a mix-up-protection claim.
7. Return an exclusive success or protocol-error outcome. Success retains the
   complete request lineage and one bounded visible-ASCII code. Error retains
   a classified exact NQSCHAR code and optional independently bounded NQSCHAR
   description and valid URI-reference, but drops request secrets.
8. Store sensitive and remote values in zeroizing ownership, require explicit
   accessors, and keep aggregate Debug plus all public errors static and
   redacted.
9. Reuse the private strict form decoder and factor shared OAuth character
   checks privately. Add no dependency; `oauth2 5.0.0` remains an oracle under
   ADR 0102.
10. Defer bounded authorization-code token-request construction to focused
    successor [#358](https://github.com/hyperledger-identus/sdk-rust/issues/358).

## Consequences

Consumers receive a one-shot protocol transition that cannot expose a code or
remote error before exact transaction correlation. Delegated Authorization
Servers are compared against their own metadata issuer, not the Credential
Issuer. Adapters still own callback routing, redirect registration, transport
authenticity, timeout and input preallocation limits.

The API intentionally distinguishes a correlated OAuth error outcome from an
SDK validation error. It adds no browser, HTTP, fragment, form-post, JARM,
authorization, trust, persistence or code-exchange behavior.

## Reconsideration and rollback

Reconsider response modes only in separately specified slices with their own
transport and cryptographic evidence. Reconsider a shared OAuth parser only
when a released, feature-sliced candidate improves multiple protocol surfaces
without weakening exact decoding, resource bounds or redacted diagnostics.
Rollback removes the response types, metadata projection, private grammar
factoring, diagnostics and evidence while preserving #354 request behavior.
