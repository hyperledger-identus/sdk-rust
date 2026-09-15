## ADDED Requirements

### Requirement: Native JWK rejection cleanup

Native `PublicKeyJwk` construction SHALL own cleanup for every supplied
extension map. When profile, member, resource, or coordinate validation rejects
the value, nested extension arrays and objects SHALL be dismantled iteratively
instead of entering recursive `serde_json::Value` destruction. Cleanup SHALL
not change the public constructor, accepted extension budgets, error variant,
redaction, dependency cone, or wire representation.

#### Scenario: Hostile depth accompanies an earlier validation error

- **WHEN** native construction receives an extension tree nested far beyond the
  accepted depth together with an incompatible key profile
- **THEN** construction SHALL return the existing profile error and consume the
  owned extension tree without recursive drop or panic

#### Scenario: Accepted extensions retain existing behavior

- **WHEN** a native extension map is within member, depth, node, and text limits
  and all key-profile inputs are valid
- **THEN** the map SHALL be retained unchanged and round-trip through serde
