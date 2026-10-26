# peer-did-method Specification

## ADDED Requirements

### Requirement: peer DID profiles are explicit and independently evidenced

The SDK SHALL expose peer DID through an independently selectable method crate.
Numalgo 4 SHALL be the current profile and numalgo 2 SHALL be an explicit
compatibility profile. A caller SHALL NOT silently parse, create, or emit one
profile through the other profile's API.

#### Scenario: an existing Identus consumer requests numalgo 2

- **WHEN** the compatibility profile is selected explicitly
- **THEN** creation and resolution use the pinned DIF and Identus compatibility
  evidence without changing the current numalgo-4 profile

#### Scenario: an unsupported or disabled algorithm is received

- **WHEN** a peer DID selects numalgo 0, 1, 3, any unknown or malformed
  algorithm identifier, or a disabled profile
- **THEN** the method rejects it before unbounded decoding or allocation work

### Requirement: peer DID remains bounded and custody-neutral

Peer DID parsing SHALL apply finite limits to key material, service entries,
decoded bytes, and document projection. The method crate SHALL accept public key
material and caller-owned entropy or crypto ports without owning secret
generation, custody, persistence, network, or agent policy.

#### Scenario: a peer DID exceeds a resource boundary

- **WHEN** its encoded bytes, decoded keys, services, elements, or expansion
  exceed the selected profile limit
- **THEN** resolution fails with a stable redacted resource error before the
  narrowest available expensive operation
