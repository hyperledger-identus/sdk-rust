## ADDED Requirements

### Requirement: Support-policy mutation parsing is deterministic

Required support-policy mutation evidence SHALL use an explicit parser from
the repository's pinned Nix derivation and SHALL NOT discover an arbitrary host
Nix executable. Every fixture SHALL distinguish valid Nix that the policy
checker rejects from intentionally invalid Nix that fails closed at both
boundaries.

#### Scenario: Pinned derivation evaluates parser-valid fixtures

- **WHEN** the factory-contract derivation runs a mutation declared parser-valid
- **THEN** its exact pinned `nix-instantiate` accepts the fixture before the
  support-policy checker rejects the prohibited policy shape

#### Scenario: Pinned derivation evaluates parser-invalid fixtures

- **WHEN** the factory-contract derivation runs a mutation declared
  parser-invalid
- **THEN** its exact pinned `nix-instantiate` and the support-policy checker
  both reject the fixture

#### Scenario: Direct host execution has ambient Nix

- **WHEN** a developer runs the mutation suite without the explicit pinned
  parser input and an unrelated `nix-instantiate` exists on `PATH`
- **THEN** the suite ignores the ambient executable and produces the same
  checker verdict as a host without Nix
