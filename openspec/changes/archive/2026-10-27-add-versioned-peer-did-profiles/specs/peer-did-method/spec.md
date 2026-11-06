# peer-did-method delta

## MODIFIED Requirements

### Requirement: peer DID profiles are explicit and independently evidenced

The SDK SHALL expose peer DID through an independently selectable method crate.
Numalgo 4 SHALL be the current profile and numalgo 2 SHALL be an explicit
compatibility profile with separate public modules and values. A caller SHALL
NOT silently parse, create, resolve, or emit one profile through the other
profile's API.

#### Scenario: current numalgo 4 long form is resolved

- **WHEN** the bounded encoded input document and its SHA2-256 multihash agree
- **THEN** the SDK contextualizes the validated input as a DID document and
  returns its derived short-form relationship without external I/O

#### Scenario: numalgo 4 short form lacks matching caller material

- **WHEN** resolution receives only a short form or non-matching long-form
  material
- **THEN** the SDK returns a stable redacted not-found or hash-mismatch outcome
  and performs no network or persistence work

#### Scenario: an existing Identus consumer explicitly requests numalgo 2

- **WHEN** the `numalgo2_compat` profile is selected
- **THEN** creation and resolution use the pinned DIF and Identus compatibility
  evidence without changing or weakening the numalgo-4 profile

#### Scenario: an unsupported or disabled algorithm is received

- **WHEN** a peer DID selects numalgo 0, 1, 3, an unknown value, a malformed
  value, or a profile not accepted by the selected module
- **THEN** the method rejects it before proportional decoding or allocation

### Requirement: peer DID remains bounded and custody-neutral

Peer DID parsing SHALL apply finite public limits to raw/decoded bytes, keys,
services, elements, JSON traversal and document projection. Creation SHALL
accept caller-owned public material and SHALL NOT own secret generation,
custody, persistence, network, executor, relationship, or agent policy.

#### Scenario: a peer DID exceeds a resource boundary

- **WHEN** encoded bytes, decoded bytes, elements, keys, services, JSON work or
  expansion exceed the selected profile limit
- **THEN** resolution fails with a stable redacted resource error before the
  narrowest available expensive operation

#### Scenario: malformed key or service material is supplied

- **WHEN** a purpose, multicodec, local reference, service shape or private key
  member is outside the pinned profile
- **THEN** validation fails without retaining or exposing the material
