## MODIFIED Requirements

### Requirement: Emulator execution does not activate Android support

An isolated arm64 API-35 emulator consumer SHALL observe binding API version
`1` and the existing valid, invalid, oversized and redacted-error behavior
families. The evidence SHALL run only in the active hosted weekly/manual slow
macOS job or its exact local reproduction and SHALL NOT mutate a user AVD.
`SDK-LIM-002` and `SDK-LIM-003` SHALL remain effective.

The selected emulator SHALL be the API-35 default AOSP ARM64 image and the test
SHALL NOT depend on Google APIs or Play Store.

#### Scenario: one ephemeral emulator passes

- **WHEN** the generated local AAR executes on the selected emulator
- **THEN** the receipt SHALL name its SDK/image/API/ABI inputs while making no
  physical-device, other-ABI, publication, compatibility-matrix or support
  claim

#### Scenario: Google services are not a runtime dependency

- **WHEN** the native DID smoke test provisions its API-35 ARM64 AVD
- **THEN** it selects the default AOSP image and makes no Google API, Play Store
  or additional license dependency part of the proof
