## ADDED Requirements

### Requirement: Pinned four-consumer DID compatibility evidence

The DID Core capability SHALL maintain independently authored executable cases
showing that NeoPRISM, midnight-identity, Lace ID Portal and Oxid generic DID
document and query shapes are expressible through public `identus-did` APIs.
Each case SHALL identify an immutable evidence revision, use no production
identifier or secret, and distinguish generic SDK representation from method,
chain, transport, trust and wallet policy that remains downstream. Evidence
from a repository with unresolved license provenance SHALL inform an
independently authored case only and SHALL NOT be copied.

#### Scenario: consumer document shapes remain lossless and bounded

- **WHEN** independently authored PRISM-, Midnight-, Lace- and Oxid-shaped DID
  documents enter through the applicable strict public construction boundary
- **THEN** their generic identifiers, public verification material,
  relationships, services and extensions SHALL remain typed or semantically
  preserved under existing limits without a downstream dependency

#### Scenario: stricter method profiles remain adapter-owned

- **WHEN** a consumer requires network, identifier, controller, curve,
  relationship or service policy stricter than generic DID Core
- **THEN** an adapter projection SHALL be able to validate that profile before
  or after the generic model without the SDK claiming method correctness

#### Scenario: legacy resolver errors migrate only explicitly

- **WHEN** a pinned consumer uses a historical DID Resolution error keyword
- **THEN** only the explicit legacy-keyword migration helper SHALL map a
  recognized value to the current URI-valued error contract, while strict wire
  parsing continues to reject the legacy scalar form

#### Scenario: independent method adapters share the generic query seam

- **WHEN** PRISM- and Midnight-shaped adapters are composed through the public
  object-safe registry/query ports used by service and wallet consumers
- **THEN** exact method dispatch and result validation SHALL succeed without
  importing method implementation, runtime, HTTP, VDR, storage or product types
