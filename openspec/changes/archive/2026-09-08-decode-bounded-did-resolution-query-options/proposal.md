# Why

The first DID Resolution HTTP slice deliberately rejects every non-empty query.
The W3C GET binding instead carries every resolution option except `accept` as
a query parameter. Without a bounded adapter, hosts must either omit common
version/cache controls or recreate security-sensitive parsing.

Issue #203 defines the next small child of #10: map one strict HTTP query into
the existing typed `ResolutionOptions` without changing DID Core or adding a
permissive form-codec dependency.

# What changes

- Replace unconditional non-empty-query rejection with a private bounded URI
  query decoder.
- Decode the common `expandRelativeUrls`, `noCache`, `versionId` and
  `versionTime` options; keep method options as string-valued extensions.
- Preserve the negotiated document media type in `ResolutionOptions.accept`
  while keeping full-result envelope selection out of that field.
- Reject malformed, ambiguous, conflicting or excessive input as the existing
  redacted W3C `invalidOptions` response before resolver invocation.
- Record ADR 0091, conformance tests, exact review and rollback evidence.

# Capabilities

## Modified capabilities

- `did-resolution-http`: add bounded GET resolution-option transport and
  remove the temporary blanket rejection of non-empty query strings.

# Non-goals

- No DID URL dereferencing route or DID-parameter interpretation.
- No POST binding, OpenAPI, server runtime, caching implementation, resolver
  algorithm, authentication, authorization, rate limiting or publication.
- No public query parser or candidate dependency types.
- No generic coercion of extension values into JSON booleans or numbers.
- No portable-target, certification or downstream-adoption claim.

# Delivery

Issue #203 owns this bounded child of #10. Specification, research, constraints
and ADR land in a signed/DCO commit before Rust implementation. Focused and
workspace tests, factory/dependency gates, exact-diff review and hosted CI are
required before merge to `develop`.
