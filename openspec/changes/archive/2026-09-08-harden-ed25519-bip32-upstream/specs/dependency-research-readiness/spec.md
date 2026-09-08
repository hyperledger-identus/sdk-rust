## ADDED Requirements

### Requirement: Conditional dependency remediation is upstream-first

The SDK project SHALL prefer a minimal upstream contribution before activating an SDK-maintained dependency fork.

This applies when a conditionally adopted dependency has a focused defect that
can be fixed without changing the SDK's required semantics. The project SHALL
record the contribution's disposition and preserve relevant compiler, target,
license, feature and conformance contracts. It SHALL NOT make an unpublished
branch an SDK dependency or represent an upstream contribution as an effective
remediation until a reviewed immutable release is integrated.

#### Scenario: Focused upstream hardening is viable

- **WHEN** a private implementation dependency exposes secret diagnostics,
  carries bespoke unsafe code, or activates unrelated dependency features and
  those concerns can be removed without changing required cryptographic bytes
- **THEN** the project offers the minimal tested patch upstream and keeps the
  current SDK facade until a released artifact passes a separate update issue

#### Scenario: Upstream does not provide a usable release

- **WHEN** the upstream patch is declined, remains inactive through an accepted
  release trigger, or produces an incompatible artifact
- **THEN** an SDK-maintained fork requires a separate bounded decision with an
  exact base, unchanged conformance vectors, maintenance owner, rollback and a
  sunset condition back to upstream
