# weekly-slow-evidence Specification

## ADDED Requirements

### Requirement: Android command-line tool discovery is explicit

The macOS slow lane SHALL select the Android SDK only from
`ANDROID_SDK_ROOT`, falling back to `ANDROID_HOME`, and SHALL invoke
`sdkmanager` by its executable path below that selected SDK. Missing SDK or
command-line tools SHALL fail before package installation. The workflow SHALL
retain exact Android package identifiers rather than accepting ambient default
versions.

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
