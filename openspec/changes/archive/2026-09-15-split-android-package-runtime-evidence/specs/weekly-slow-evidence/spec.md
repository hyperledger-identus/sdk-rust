## MODIFIED Requirements

### Requirement: Android command-line tool discovery is explicit

Each Android slow-lane job SHALL select the Android SDK only from
`ANDROID_SDK_ROOT`, falling back to `ANDROID_HOME`, and SHALL invoke
`sdkmanager` by its executable path below that selected SDK. Missing SDK or
command-line tools SHALL fail before package installation. The workflow SHALL
retain exact Android package identifiers rather than accepting ambient default
versions.

The macOS package job SHALL install exact NDK `ndk;27.0.12077973` and platform
`platforms;android-35`, build/inspect the exact ARM64 AAR, and SHALL NOT attempt
to boot an Android VM. The dedicated Linux runtime job SHALL install the same
NDK/platform plus exact package
`system-images;android-35;default;x86_64`, require KVM, and execute only the
explicitly test-only x86_64 behavior artifact. Installation SHALL occur without
blanket SDK license acceptance.

The installed NDK package and each verifier mode's effective compiler root
SHALL agree. Ambient hosted-runner NDK aliases SHALL not override the exact
installed package. Package and runtime evidence SHALL be uploaded on success or
failure with exact SHA/run-attempt identity and seven-day retention.

#### Scenario: Hosted SDK tools are installed but absent from PATH

- **WHEN** a selected runner declares its Android SDK and does not expose a
  bare `sdkmanager` command
- **THEN** the slow lane invokes `cmdline-tools/latest/bin/sdkmanager` beneath
  that SDK and installs only the exact packages for its evidence role

#### Scenario: Declared SDK is incomplete

- **WHEN** neither SDK environment variable is present or the derived tool is
  not executable
- **THEN** the job fails before installing packages or starting native binding
  verification

#### Scenario: Google Play image has an unaccepted license

- **WHEN** the DID smoke test requires only an API-35 x86_64 Android runtime
- **THEN** the Linux lane selects the default AOSP image instead of accepting a
  Google Play or blanket outstanding SDK license

#### Scenario: ARM64 hosted runner cannot nest a VM

- **WHEN** the hosted macOS runner lacks nested virtualization
- **THEN** it still proves the exact distributable ARM64 package while the
  separate KVM-capable Linux job owns Android runtime behavior

#### Scenario: Runner default differs from the installed exact NDK

- **WHEN** a hosted image exports a newer default NDK root
- **THEN** native verification uses the exact side-by-side package installed by
  the workflow and fails if its metadata or artifact identity differs

#### Scenario: Runtime job fails after producing diagnostics

- **WHEN** package assembly, AVD boot or behavior execution fails
- **THEN** the immutable run receipt reports failure and the role-specific
  Android evidence artifact retains available logs and partial metadata
