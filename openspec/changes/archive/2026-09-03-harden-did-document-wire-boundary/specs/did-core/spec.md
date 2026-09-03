## ADDED Requirements

### Requirement: Duplicate-free raw DID document JSON

The bounded raw DID document entry points SHALL stream every JSON object name
through a duplicate detector before typed deserialization. The detector SHALL
reject duplicate decoded names within the same object at every nesting level,
including known fields, extensions, contexts, verification material, services,
and endpoint maps. Escaped and literal spellings that decode to the same name
SHALL collide. Names repeated only in different objects SHALL remain valid.

Failures SHALL expose a stable non-sensitive document reason and the existing
public `did.invalid_document` code without the rejected name, value, offset, or
document bytes. Native maps and materialized semantic JSON values SHALL NOT be
claimed to preserve a lexical condition they cannot represent.

#### Scenario: ambiguity fails before semantic collapse

- **WHEN** raw document JSON repeats a top-level, verification-method, JWK,
  service, context, endpoint, or arbitrary nested extension name
- **THEN** the document SHALL fail before typed construction regardless of
  which duplicate value a last-value-wins parser would otherwise retain

#### Scenario: decoded names define equality

- **WHEN** one object contains literal and escaped spellings that decode to the
  same JSON member name
- **THEN** the scanner SHALL reject the object without exposing that name

#### Scenario: object-local names do not create false collisions

- **WHEN** sibling or nested objects legitimately reuse the same member name
  but no individual object repeats it
- **THEN** duplicate scanning SHALL succeed and ordinary document validation
  SHALL decide the result

### Requirement: Independently evidenced URI recognition

The dependency-free production `Uri` parser SHALL remain governed by the RFC
3986 `URI` production and the SDK's documented ASCII and 4,096-byte resource
profile. Deterministic conformance tests SHALL compare accept/reject decisions
with the exact development-only `uriparse` 0.6.4 oracle used by NeoPRISM over
scheme, authority, path, query, fragment, percent-encoding, IPv4, IPv6, and
IPvFuture classes. Every mismatch SHALL be a pinned regression with an explicit
normative or SDK-policy classification; oracle normalization SHALL NOT change
the preserved SDK spelling.

#### Scenario: standards-shaped component combinations agree

- **WHEN** bounded RFC 3986 URI component combinations are generated
  deterministically
- **THEN** the SDK and oracle SHALL agree except for explicitly recorded SDK
  resource/profile differences

#### Scenario: development evidence does not widen runtime dependencies

- **WHEN** production, minimal-feature, mobile, or WASM dependency cones are
  built
- **THEN** `uriparse` SHALL NOT be a normal or target dependency of
  `identus-did`

### Requirement: Reproducible DID document hardening evidence

The DID Core test suite SHALL deterministically generate bounded URI/document
structures and raw duplicate mutations with a fixed reproducible algorithm.
It SHALL cover native and unique-name semantic-wire equivalence, every scanner
limit, malformed/trailing input, and retained minimized regressions. A release
diagnostic SHALL report hardened scan-and-parse throughput without a
machine-specific pass threshold. Sanitizer-backed DID/DID URL cargo-fuzz and
resolution-envelope scanning SHALL remain separately tracked work.

#### Scenario: generated evidence is stable in ordinary CI

- **WHEN** the focused conformance suite runs on the same revision
- **THEN** it SHALL exercise the same bounded cases without randomness,
  network, filesystem, clock, nightly compiler, or consumer repository

#### Scenario: performance remains observable but portable

- **WHEN** the representative hardened document parser runs in release mode
- **THEN** throughput and the scanner's explicit resource shape SHALL be
  recorded without a hardware-specific acceptance threshold

## MODIFIED Requirements

### Requirement: Uniform bounded document validation

Raw DID document JSON SHALL be limited to 256 KiB before duplicate scanning or
deserialization. The streaming scanner SHALL reject more than 64 nested
containers, 16,384 visited values, 128 members in one object, or 128 KiB of
simultaneously retained decoded names. Document collections SHALL contain at
most 128 entries; extension maps SHALL contain at most 64 entries with property
names no longer than 256 bytes; arbitrary extension trees SHALL contain at most
4,096 nodes, be at most 32 levels deep and contain strings no longer than 64
KiB. Representable native construction and serde deserialization SHALL share
the semantic structural checks. Failures SHALL map to `did.invalid_uri` or
`did.invalid_document` with capability `did`, `InvalidInput` kind and no
caller-controlled public detail.

#### Scenario: raw preflight is independently bounded

- **WHEN** bounded-size raw JSON exceeds scanner depth, node, per-object member,
  or live decoded-name limits
- **THEN** it SHALL fail before typed deserialization with no caller data in
  local or public diagnostics

#### Scenario: native construction cannot bypass semantic limits

- **WHEN** the same representable excessive collection or extension tree is
  supplied through a constructor or semantic JSON deserialization
- **THEN** both paths SHALL reject it under the same public resource policy

#### Scenario: bounded validation remains performant

- **WHEN** a representative standards-shaped document is scanned and parsed
  repeatedly in a release diagnostic
- **THEN** throughput SHALL be recorded as observational evidence without an
  environment-specific CI pass threshold
