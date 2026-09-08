## RENAMED Requirements

- FROM: `### Requirement: MSRV and etalon toolchain are independent gates`
- TO: `### Requirement: Temporary active-development compiler and CI lanes`

## MODIFIED Requirements

### Requirement: Temporary active-development compiler and CI lanes

Until the earlier of 2026-12-08 or release-candidate preparation, the SDK SHALL
use exact stable Rust `1.98.1` as its workspace compiler floor, primary
development compiler and compatibility etalon. All ordinary Crane providers
and dependency artifacts SHALL resolve to that same compiler rather than
duplicating primary, lower-MSRV and NeoPRISM-nightly compiler builds. The SDK
SHALL make no compatibility claim below Rust 1.98.1 during this phase.

Pull requests targeting `develop` and pushes to `develop` SHALL receive one
Linux job named `fast` that runs the manifest-derived repository/factory,
formatting, workspace build, strict Clippy and normal workspace test gates.
The full Linux/macOS Nix matrix SHALL remain available as a weekly and manually
dispatchable job named `slow`, using the same Rust 1.98.1 compiler for all SDK
compatibility checks. Slow failures SHALL be visible pre-release debt and SHALL
block release-candidate preparation, but SHALL NOT be represented as required
per-PR evidence during this temporary phase.

Sanitizer fuzz campaigns MAY use a separately named, exactly pinned nightly
tooling shell because libFuzzer instrumentation requires nightly. Those
workflows SHALL run only weekly or manually, SHALL remain outside ordinary SDK
compiler providers, and SHALL NOT be represented as Rust 1.98.1 compatibility
evidence. The policy SHALL carry the 2026-12-08 review date and SHALL prohibit
release-candidate use until a separate consumer-driven compatibility decision.

#### Scenario: Pull request receives rapid deterministic evidence

- **WHEN** a pull request targets `develop`
- **THEN** one Ubuntu `fast` status runs factory structure, format, workspace
  build, strict Clippy and the normal workspace test suite on Rust 1.98.1

#### Scenario: Exhaustive evidence runs outside the pull-request critical path

- **WHEN** the weekly schedule fires or a maintainer manually dispatches it
- **THEN** `slow` runs the complete flake on Linux and macOS, including target,
  feature, documentation and supply-chain checks, on Rust 1.98.1

#### Scenario: Lower compiler is presented as supported

- **WHEN** Cargo, Nix or documentation claims compatibility below Rust 1.98.1
  during the temporary phase
- **THEN** structural policy validation fails

#### Scenario: Nightly leaks into ordinary SDK validation

- **WHEN** a non-fuzz devshell or ordinary Crane gate selects the pinned
  sanitizer nightly
- **THEN** structural policy validation fails even if compilation succeeds

#### Scenario: Release candidate is proposed under temporary evidence

- **WHEN** a release candidate is proposed before a new compatibility decision
- **THEN** release policy blocks it until slow/sanitizer debt and actual
  consumer compiler requirements are resolved

#### Scenario: Temporary policy reaches its review date

- **WHEN** the date reaches 2026-12-08 without a superseding decision
- **THEN** the policy is expired for further release planning and a focused
  review issue must choose the next compiler and CI matrix

### Requirement: Supported feature surfaces are isolated

The target policy SHALL enumerate the default, minimal and opt-in feature
surfaces that are required to compile or test. Checks SHALL exercise compatible
surfaces independently rather than relying only on Cargo feature unification.
The `crypto-minimal` surface SHALL include both strict Clippy and tests with no
default features. During the temporary active-development phase, exhaustive
feature permutations SHALL run in weekly/manual `slow` on Rust 1.98.1 rather
than through an independently lower compiler lane.

#### Scenario: Minimal crypto surface regresses

- **WHEN** `identus-crypto` no longer compiles, lints or runs its eligible tests
  without default features
- **THEN** its isolated minimal-feature gate fails

#### Scenario: Entropy feature surface regresses

- **WHEN** the empty, deterministic or system-random entropy feature surface
  fails independently
- **THEN** the corresponding feature-matrix gate fails
