## ADDED Requirements

### Requirement: Scalar serde token generation has one private owner

The `identus-derive` crate SHALL single-own the common serde token generation
for string- and numeric-backed newtypes behind a crate-private compile-time
helper. The helper SHALL NOT create a public macro, trait, runtime abstraction,
dependency, or category-policy boundary. String- and numeric-specific
construction, conversion, parsing, validation, and ownership behavior SHALL
remain in their category modules.

#### Scenario: Unvalidated scalar serde remains transparent

- **WHEN** an unvalidated string- or numeric-backed newtype opts into serde
- **THEN** serialization SHALL delegate to the inner value
- **AND** deserialization SHALL construct the newtype from the deserialized
  inner value without adding validation or changing the JSON form

#### Scenario: Validated scalar serde rejects invalid inner values

- **WHEN** a validated string- or numeric-backed newtype opts into serde
- **THEN** deserialization SHALL invoke its configured validator exactly once
  on the deserialized inner value before construction
- **AND** validator failure SHALL map through `serde::de::Error::custom`
- **AND** serialization and every category-specific API SHALL remain unchanged

#### Scenario: Bytes encoding remains category-owned

- **WHEN** a bytes-backed newtype opts into serde
- **THEN** its existing hex or base64url encode/decode behavior SHALL remain in
  the bytes category and SHALL NOT be routed through the scalar helper
