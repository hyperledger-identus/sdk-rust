## ADDED Requirements

### Requirement: OID4VCI numeric resource policy has one definition site

The `identus-oid4vci` crate SHALL define every default numeric limit and the
maximum configurable JSON depth in one private policy module. Each independent
policy role SHALL retain a distinct named constant even when two roles have
the same current value. Public limit defaults SHALL reference those constants
without dynamic lookup, ambient configuration, or a generic untyped map.

#### Scenario: Maintainer audits effective default limits

- **WHEN** a maintainer reviews the private policy module
- **THEN** every effective numeric value used by an OID4VCI public limit
  `Default` implementation SHALL be discoverable there exactly once per role

#### Scenario: Two roles currently share a number

- **WHEN** unrelated protocol fields have equal effective limits
- **THEN** their independently named policy constants SHALL preserve separate
  change authority and SHALL NOT couple future policy changes

### Requirement: Limit type mechanics follow protocol lifecycle ownership

Credential Offer, token, credential-response, and metadata limit types SHALL
be owned by separate private lifecycle modules behind one `limits` facade.
The facade and crate root SHALL preserve every existing public name, type,
derive, constructor, accessor, const qualification, `Default` implementation,
validation predicate, composition, and static error.

#### Scenario: Existing consumer upgrades across the refactor

- **WHEN** a consumer uses any existing `identus_oid4vci::*Limits` type or
  `MAX_CONFIGURABLE_JSON_DEPTH`
- **THEN** the same source path, signature, effective default, accepted and
  rejected constructor inputs, and error variant SHALL remain available

#### Scenario: Code-health decomposition is evaluated

- **WHEN** the aggregate limits module is split
- **THEN** numeric policy SHALL remain single-owned, lifecycle modules SHALL
  each have one protocol change axis, and no forwarding-only, macro-hidden,
  duplicated-policy, or over-threshold replacement SHALL count as improvement
