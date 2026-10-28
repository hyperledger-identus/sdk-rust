# sd-jwt-engine-selection delta

## MODIFIED Requirements

### Requirement: Local implementation remains focused and replaceable

When no candidate passes every adoption gate, the SDK SHALL implement only the
bounded deterministic RFC 9901 mechanics needed by named issuance and
presentation consumers. The focused implementation SHALL live in the
independently selectable `identus-sd-jwt` crate and expose only SDK-owned
staged values. Signing, verification, entropy, time, key resolution, trust,
custody, replay, transport, persistence, and consent SHALL remain behind
existing focused Identus capabilities or caller-owned policy. Candidate types
and production dependencies SHALL remain outside the public and resolved
production graph.

#### Scenario: External frameworks fail cohesion or bounds

- **WHEN** no released engine clears the complete adoption matrix
- **THEN** the implementation issue builds an Identus-owned focused engine and retains external implementations only as independent differential oracles

#### Scenario: private mechanics are later replaced

- **WHEN** a future immutable engine passes every adoption and equivalence gate
- **THEN** it may replace internal mechanics without changing the staged Identus public contract or silently changing RFC 9901 behavior

### Requirement: Evidence covers hostile behavior and a complete offline journey

Implementation SHALL use versioned provenance-bound RFC 9901 and SD-JWT VC
vectors plus clean-room negative cases. The RFC 9901 base quality declaration
SHALL cover property tests, compact/disclosure/claim-graph/key-binding fuzzing,
bounded worst-case benchmarks, at least two differential oracles,
cross-language migration fixtures, and an offline
issue-store-select-present-key-bind-verify journey through public SDK APIs.
Official examples SHALL be distinguished from SDK-authored and donor evidence,
and the evolving SD-JWT VC profile SHALL remain a separate declaration.

#### Scenario: Valid round trip passes but rejection behavior differs

- **WHEN** a candidate or local engine accepts duplicate digests, unused disclosures, reserved claims, malformed encodings, mismatched key binding, or over-limit work that the pinned contract rejects
- **THEN** conformance fails even if valid issuance and presentation round trips succeed

#### Scenario: the base offline journey passes without a VC profile

- **WHEN** RFC 9901 issuance, envelope storage, claim selection, key binding, and verification complete through public SDK seams
- **THEN** the base quality declaration may become passing without claiming SD-JWT VC, OID4VC, HTTP, trust, publication, or downstream adoption completion
