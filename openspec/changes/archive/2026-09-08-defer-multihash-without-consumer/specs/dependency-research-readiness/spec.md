## ADDED Requirements

### Requirement: Production dependencies require concrete consumer payoff

A technically suitable crate SHALL NOT become a production dependency solely
because it passes compiler, license, maintenance, security, target, feature and
dependency-cone gates. Its adoption decision SHALL identify a current SDK
capability or named consumer, the normative behavior it requires, and the local
implementation, demonstrated correctness risk or material maintenance burden
the crate replaces. Without that evidence, the candidate SHALL remain
conditional or deferred and SHALL NOT be added to Cargo or used to expand a
public SDK type.

#### Scenario: Cohesive crate has no current consumer

- **WHEN** a narrow standards crate passes every technical adoption gate but no current SDK capability consumes its semantics
- **THEN** the decision records exact reusable evidence and an objective activation trigger without adding the dependency or inventing public policy

#### Scenario: A named consumer activates reconsideration

- **WHEN** a focused issue pins a normative profile that needs the candidate and defines its policy, resource, compatibility and migration boundaries
- **THEN** research refreshes the evidence and may propose an independently reversible private integration behind Identus-owned types
