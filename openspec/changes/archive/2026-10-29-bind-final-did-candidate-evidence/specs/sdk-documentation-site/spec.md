## ADDED Requirements

### Requirement: Release readiness evidence is navigable and bounded

The documentation site SHALL link each candidate train to a durable readiness
packet containing exact identity, evidence classes, immutable receipts,
limitations, downstream compatibility where available, and remaining human or
administrative gates. It SHALL NOT present a candidate-only package as
published or infer success from stale, partial, aggregate-only, or failed
evidence.

#### Scenario: an engineer reviews the DID candidate

- **WHEN** the engineer opens the DID candidate or release-readiness pages
- **THEN** they can navigate to one closed report explaining what passed, what
  remains pending, how to reproduce local evidence, and which claims are out of
  scope
