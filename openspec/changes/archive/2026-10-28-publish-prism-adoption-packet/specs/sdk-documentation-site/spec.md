## ADDED Requirements

### Requirement: The portable PRISM stack has an executable review surface

The handbook SHALL explain the responsibility, dependency direction, effect
boundary, target evidence, limitations, and adopter flow for
`identus-prism-did`, `identus-prism-vdr`, and `identus-prism-indexer` without
presenting downstream Cardano, persistence, service, or runtime effects as SDK
capabilities.

#### Scenario: an engineer evaluates the portable stack

- **WHEN** the engineer opens the PRISM overview and crate pages
- **THEN** one static diagram, one executable scenario, and durable evidence links show what the SDK owns and what an adopter must provide

#### Scenario: downstream parity is cited

- **WHEN** the handbook references NeoPRISM live Preprod evidence
- **THEN** it identifies the immutable candidate/run boundary and labels it downstream compatibility evidence rather than an SDK-hosted network guarantee
