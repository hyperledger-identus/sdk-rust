## MODIFIED Requirements

### Requirement: Android command-line tool discovery is explicit

The macOS slow lane SHALL select the Android SDK only from
`ANDROID_SDK_ROOT`, falling back to `ANDROID_HOME`, and SHALL invoke
`sdkmanager` by its executable path below that selected SDK. Missing SDK or
command-line tools SHALL fail before package installation. The workflow SHALL
retain exact Android package identifiers rather than accepting ambient default
versions.

The emulator dependency SHALL be exact package path
`system-images;android-35;default;arm64-v8a`, and installation SHALL occur
without blanket SDK license acceptance. Workflow installation and native
verifier package/directory identity SHALL agree or fail closed.

The installed NDK package `ndk;27.0.12077973` and the native verifier's
effective compiler root SHALL agree. Ambient hosted-runner NDK aliases SHALL
not override the exact installed package.

#### Scenario: Hosted SDK tools are installed but absent from PATH

- **WHEN** the macOS runner declares its Android SDK and does not expose a bare
  `sdkmanager` command
- **THEN** the slow lane invokes `cmdline-tools/latest/bin/sdkmanager` beneath
  that SDK and installs the exact reviewed package set

#### Scenario: Declared SDK is incomplete

- **WHEN** neither SDK environment variable is present or the derived tool is
  not executable
- **THEN** the step fails before installing packages or starting native
  binding verification

#### Scenario: Google Play image has an unaccepted license

- **WHEN** the DID smoke test requires only an API-35 ARM64 Android runtime
- **THEN** the lane selects the default AOSP image instead of accepting a Google
  Play or blanket outstanding SDK license

#### Scenario: Runner default differs from the installed exact NDK

- **WHEN** the hosted image exports a newer default NDK root
- **THEN** native verification uses the exact side-by-side package installed by
  the workflow and fails if its metadata or artifact identity differs
