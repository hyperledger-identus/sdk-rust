## MODIFIED Requirements

### Requirement: Host consumers execute the versioned value slice

The facade SHALL expose binding API version `1`. Swift and Kotlin/JVM host tests
SHALL compile, link and execute valid, invalid, oversized, redacted-error and
version checks against the built dynamic library. The host execution and
separately locked generator supply-chain checks SHALL run in the active hosted
weekly/manual slow macOS job or its exact local reproduction, not the
Linux-only fast pull-request lane.

#### Scenario: host language consumes API version one

- **WHEN** the Swift and Kotlin/JVM smoke programs load the generated binding
- **THEN** each SHALL observe API version `1` and pass the same success and
  failure behavior families

#### Scenario: fast and slow CI remain deliberately separated

- **WHEN** ordinary pull-request CI runs during the active-development phase
- **THEN** the Linux fast line SHALL retain factory, build, lint and test gates,
  while macOS host execution and generator deny/audit evidence remain in the
  native weekly/manual slow workflow or exact local reproduction

### Requirement: Simulator execution does not activate Apple support

An Xcode-driven iOS Simulator test SHALL observe binding API version `1` and
the existing valid, invalid, oversized and redacted-error behavior families.
This evidence SHALL run in the active hosted weekly/manual slow macOS job or
its exact local reproduction. `SDK-LIM-002` SHALL remain effective because the
package is unsigned, unpublished and not executed on a physical device, and
because multi-Rust-static-library composition is unproven.

#### Scenario: local package passes on one Simulator runtime

- **WHEN** the generated local Swift package test passes on the selected arm64
  iOS Simulator
- **THEN** the receipt SHALL name Xcode, SDK, runtime and architecture while
  making no physical-device, distribution, older-runtime or public support claim

### Requirement: Emulator execution does not activate Android support

An isolated arm64 API-35 emulator consumer SHALL observe binding API version
`1` and the existing valid, invalid, oversized and redacted-error behavior
families. The evidence SHALL run only in the active hosted weekly/manual slow
macOS job or its exact local reproduction and SHALL NOT mutate a user AVD.
`SDK-LIM-002` and `SDK-LIM-003` SHALL remain effective.

#### Scenario: one ephemeral emulator passes

- **WHEN** the generated local AAR executes on the selected emulator
- **THEN** the receipt SHALL name its SDK/image/API/ABI inputs while making no
  physical-device, other-ABI, publication, compatibility-matrix or support
  claim
