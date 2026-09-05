## MODIFIED Requirements

### Requirement: Supported feature surfaces are isolated

The target policy SHALL enumerate the default, minimal and opt-in feature
surfaces that are required to compile or test. Checks SHALL exercise compatible
surfaces independently rather than relying only on Cargo feature unification.
The `crypto-minimal` surface SHALL include both strict Clippy and test gates
with no default features, plus an independent MSRV build gate.

#### Scenario: Minimal crypto surface regresses

- **WHEN** `identus-crypto` no longer compiles, lints or runs its eligible tests
  without default features
- **THEN** its isolated minimal-feature gate fails

#### Scenario: Entropy feature surface regresses

- **WHEN** the empty, deterministic or system-random entropy feature surface
  fails independently
- **THEN** the corresponding feature-matrix gate fails
