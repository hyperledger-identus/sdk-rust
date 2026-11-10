## ADDED Requirements

### Requirement: SDK-Rust and SDK-TS parity is capability-specific

SDK-Rust and SDK-TS SHALL define feature parity and conformance per normalized
capability. The program SHALL NOT require identical repository structure,
historical module boundaries, public API shapes, error classes, async models,
or platform implementations.

#### Scenario: portable semantics and host integration coexist

- **WHEN** one user journey combines canonical Rust protocol/domain behavior
  with browser or Node execution
- **THEN** conformance binds the portable observable behavior and explicit
  adapter mapping while TypeScript retains the accepted host responsibility

#### Scenario: roadmap progress is summarized

- **WHEN** maintainers review SDK-Rust/SDK-TS progress
- **THEN** they receive exact capability groups and missing evidence rather
  than a single percentage that treats excluded and platform-owned work as
  failed parity
