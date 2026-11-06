## ADDED Requirements

### Requirement: A candidate train has one closed final readiness packet

The SDK SHALL bind one exact protected revision to its train descriptor, staged
lock, deterministic archives, API origins, SBOMs, documentation, package-
specific compiler/target matrix, security-policy result, complete required
slow lanes, downstream compatibility evidence where available, explicit
limitations, and independent approval before a candidate-only train can be
proposed for publication activation. Every receipt SHALL use immutable links
and SHALL distinguish passing, pending, failed, and not-applicable evidence.

#### Scenario: a required slow lane fails

- **WHEN** any required lane fails, is cancelled, is stale, runs from another
  branch or revision, or only a rerun is available
- **THEN** final readiness remains pending even if an aggregate artifact exists

#### Scenario: the exact candidate is approved

- **WHEN** every closed evidence class passes for one protected revision and an
  independent maintainer approves that exact receipt
- **THEN** the candidate may proceed to a separate publication-activation
  decision without implying that publication already occurred

#### Scenario: downstream interoperability is available

- **WHEN** a consumer has immutable source-pinned and network parity evidence
- **THEN** the packet links it as downstream compatibility evidence while
  preserving consumer ownership and its stated scope and limitations
