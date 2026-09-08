# Context

The HTTP adapter owns a raw URI query and an existing typed
`ResolutionOptions` facade. The boundary must preserve extensibility without
giving ambiguous or malformed input to a method resolver.

# Goals

- Implement the W3C GET mapping for non-`accept` resolution options.
- Bound all attacker-controlled parsing and keep error output static.
- Reuse the existing option types and preserve representation negotiation.
- Keep the Cargo graph and public parser surface unchanged.

# Decisions

## Decode privately at the HTTP boundary

Add a private `decode_resolution_options` helper in the adapter. It accepts the
optional raw query plus the negotiated representation and returns a validated
`ResolutionOptions`. The handler completes path/DID and content negotiation
before query decoding, then calls the resolver exactly once.

## Split before decoding

Check the raw 8 KiB ceiling, split on `&`, require one `=`, and enforce the
32-member ceiling before decoding each name and value once. The decoder treats
only `%HH` as an escape and validates UTF-8; it never applies HTML form `+`
conversion. Encoded `&` or `=` therefore stays within its original scalar.

Decoded names and values are checked against 256-byte and 4 KiB ceilings and
for control characters. A `BTreeSet` rejects duplicate decoded names before
projection. Empty names fail; empty extension values are preserved.

## Project known fields explicitly

`expandRelativeUrls` and `noCache` accept exact lower-case booleans.
`versionId` and `versionTime` use `VersionId` and `DidResolutionDateTime`.
`accept` is rejected because the W3C HTTP binding assigns it to the header.
Unknown names become string-valued extensions.

Simultaneous version selectors fail at the transport boundary. The broader
core option API is not changed in this slice because it also serves local and
method-specific callers whose policy requires a separate compatibility issue.

The selected document representation sets the option `accept`. Full-result
selection leaves it absent because `application/did-resolution` describes the
transport envelope, not a DID document representation.

## Keep one failure boundary

All decoder/builder failures collapse to `DidResolutionErrorKind::InvalidOptions`.
No raw or decoded scalar enters an error. The resolver is not invoked on any
failure.

# Compatibility matrix

| Request | Resolver options/outcome |
| --- | --- |
| No query or bare `?` | Existing options and response bytes |
| Known valid options | Corresponding typed fields |
| Unknown valid option | String-valued extension |
| Document `Accept` plus query | Negotiated `accept` plus query fields |
| Full-result `Accept` plus query | No option `accept`; query fields preserved |
| Malformed, duplicate, conflicting or excessive query | No call; redacted 400 |

# Rollback

Revert the focused delivery commit/PR. The prior blanket rejection is restored
without data migration or dependency change.
