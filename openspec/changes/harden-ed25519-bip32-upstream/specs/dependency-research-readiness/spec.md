## ADDED Requirements

### Requirement: Conditional dependency remediation is upstream-first

The SDK project SHALL use an upstream-first remediation path when a conditionally adopted dependency has a focused defect that can be fixed
without changing the SDK's required semantics, the SDK project SHALL prepare a
minimal upstream contribution and record its disposition before activating an
SDK-maintained fork. The contribution SHALL preserve relevant compiler, target,
license, feature and conformance contracts and SHALL NOT make an unpublished
branch an SDK dependency. An upstream contribution SHALL NOT be represented as
an effective remediation until a reviewed immutable release is integrated.

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
