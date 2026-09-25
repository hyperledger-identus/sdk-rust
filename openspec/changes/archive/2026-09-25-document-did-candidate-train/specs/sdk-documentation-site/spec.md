# sdk-documentation-site

## ADDED Requirements

### Requirement: The DID candidate train has a truthful review surface

The handbook SHALL present `identus-did` and
`identus-did-resolver-http` as one independent candidate-only train. It SHALL
explain each package's responsibility, dependency direction, feature policy,
source-evaluation path, evidence status and exclusions without implying
registry availability, stable compatibility, portable support, signed
provenance, vulnerability freedom, chain behavior or publication.

#### Scenario: Engineer reviews the DID train

- **WHEN** an engineer follows the handbook navigation to the DID candidate
- **THEN** prose and a static diagram distinguish the generic DID domain/ports
  from the optional host-side HTTP adapter and link the governing evidence

#### Scenario: Consumer evaluates the candidate before publication

- **WHEN** a consumer follows the source-evaluation example
- **THEN** it uses both package names from one exact protected 40-hex revision
  without a branch, pull-request ref, nonexistent tag or `0.0.0` selector

#### Scenario: M5 promotion remains incomplete

- **WHEN** compiler/target, complete slow, approval, registry or publication
  evidence is not complete
- **THEN** the readiness and limitations pages show that state explicitly
  rather than inheriting a claim from the crypto train
