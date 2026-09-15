## Why

The first protected-`develop` slow canary reached the macOS Android package
install after code health, complete Clippy and all 31 Nix checks passed, then
failed because the Google Play ARM64 system image required an unaccepted
`android-sdk-arm-dbt-license` in the headless runner.

The native DID smoke test uses only ADB, an arm64 API-35 runtime and the local
AAR/JNA dependency. It does not use Google Play Store or Google Play services.
Google's stable package catalog provides an API-35 `default` AOSP ARM64 image
under the ordinary Android SDK license already represented by the runner's SDK
packages. Replacing the unnecessary Google Play image is narrower than
automatically accepting every outstanding SDK license.

## What changes

- Accept ADR 0122 selecting exact package path
  `system-images;android-35;default;arm64-v8a` for the experimental Android
  execution proof.
- Update workflow installation, AVD creation and image-directory validation to
  use the same AOSP package identity.
- Extend the ordered slow-workflow contract and mutation suite to reject Google
  APIs/Play image drift or a mismatch between installation and execution.
- Update weekly and native-binding capability specifications and record the
  external package/license limitation without changing Android support status.

## Capabilities

### Modified capabilities

- `weekly-slow-evidence`: installs the exact least-privilege AOSP image without
  broad noninteractive license acceptance.
- `native-did-bindings`: executes the same API-35 ARM64 behavior on an AOSP AVD
  because the tested AAR has no Google services dependency.

## Non-goals

- No `yes | sdkmanager --licenses`, Google-license acceptance, Android API/ABI
  expansion, physical-device claim, publication, runtime SDK API or fast-lane
  change.
- No Android CLI migration; the hosted image currently exposes the reviewed
  `cmdline-tools/latest/bin/sdkmanager` path and migration is independent work.
- No immediate canary claim; the candidate repair and this image repair must
  both merge before the next complete exact-head run.

## Delivery

Issue #276 owns the failed canary and live acceptance. This zero-stack-depth PR
starts from merged candidate repair `47136ed8bc73faba34c78d88768136cd6841364d`.
After protected merge, one manual full canary runs; the issue remains open until
the first natural weekly run is reviewed.
