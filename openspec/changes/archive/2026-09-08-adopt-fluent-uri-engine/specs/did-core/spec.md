## MODIFIED Requirements

### Requirement: Independently evidenced URI recognition

The production `Uri` parser SHALL preserve the RFC 3986 `URI` production and
the SDK's documented ASCII and 4,096-byte resource profile using exact
`fluent-uri 0.4.1` as a private grammar engine with default features disabled.
The byte ceiling SHALL execute before parsing. Candidate types, errors,
normalization, IRI, resolution and component policy SHALL NOT enter the public
API or retained representation; successful values SHALL retain exact caller
spelling in the existing Identus-owned type.

Deterministic conformance tests SHALL compare accept/reject decisions with the
exact development-only `uriparse 0.6.4` oracle used by NeoPRISM over scheme,
authority, path, query, fragment, percent-encoding, IPv4, IPv6 and IPvFuture
classes. Every mismatch SHALL be a pinned regression with an explicit
normative or SDK-policy classification. DID and DID URL lexical parsing SHALL
remain on their existing method-specific implementation and SHALL NOT be
replaced by the generic URI engine.

#### Scenario: Standards-shaped component combinations agree

- **WHEN** bounded RFC 3986 URI component combinations are generated deterministically
- **THEN** the SDK grammar engine and independent oracle SHALL agree except for explicitly recorded SDK resource/profile or oracle defects

#### Scenario: Published dependency remains private

- **WHEN** a generic `Uri` is parsed, formatted, serialized or rejected
- **THEN** its public API, exact spelling, 4,096-byte precheck and stable Identus error facade remain unchanged and expose no `fluent-uri` type or diagnostic

#### Scenario: Development evidence does not widen runtime dependencies

- **WHEN** production, minimal-feature, mobile or WASM dependency cones are built
- **THEN** `uriparse` SHALL NOT be a normal or target dependency of `identus-did` and SHALL remain development-only

#### Scenario: Method-specific parsing is not replaced accidentally

- **WHEN** `Did` and `DidUrl` parse their bounded lexical forms
- **THEN** their current cached-parts parser, errors and fuzz targets remain independent of the generic URI dependency
