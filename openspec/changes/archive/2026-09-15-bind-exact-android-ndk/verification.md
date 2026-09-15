# Verification receipt

- **Verification status:** passed locally
- **Verification date:** 2026-09-15
- **Planning head:** `6cc64e9f30964b66c23db67b2da2b8f2f5bcf9b1`
- **Implementation head:** `7da9d4b0799f105745569c30a217a8f9c61c600d`
- **Base:** `develop@6423a9e0d946c1f438f554a3b1fe6222fb28717a`

## Passed locally

- `python3 scripts/tests/support-policy.py`: 199 mutation tests passed.
- `bash -n scripts/check-uniffi-did-android.sh`: passed.
- `nix develop --command actionlint .github/workflows/nix-checks.yml`: passed.
- `./scripts/factory check --change bind-exact-android-ndk`: all 71
  specifications/changes and factory structural contracts passed.
- `nix flake check --fallback`: all 31 compatible aarch64-darwin checks passed,
  including factory/policy, Rust 1.98.1 build/test/Clippy/docs, feature profiles,
  MSRV, WASM, Android/iOS compile targets, deny/audit and text/TOML/Nix linting.
  Nix reported x86_64-linux as locally incompatible.
- `git diff --check`: passed.

## Negative and provenance evidence

The mutation suite rejects ambient `ANDROID_NDK_ROOT` precedence, a relaxed
metadata revision, an unbound child NDK root, a retained latest-NDK alias and a
missing metadata checksum receipt. Existing assertions continue to reject NDK
r27/build-number drift in the packaged ELF.

Failed canary `34983050826` is positive diagnostic evidence: the AOSP image
installed successfully and the previous ambient selection reached the exact
ELF mismatch. GitHub runner inventory commit
`f95c0c791f690fa64eaf9788bea06643a4176db5` independently records the ambient
27.3.13750724 path.

## Routed hosted evidence

This host has no Android SDK and cannot compile with or boot the selected NDK
and AOSP image. Protected exact-head Linux CI is mandatory before merge. After
merge, a complete slow canary at the exact merged `develop` SHA must pass exact
NDK installation/selection, metadata, ELF inspection, AOSP boot and AAR/JNA
behavior. The first natural Monday run remains issue #276's final acceptance.

## Exclusions

No NDK upgrade, hermetic archive checksum, physical device, wider API/ABI,
publication, signing, Maven distribution, Keystore, lifecycle or supported
Android claim is introduced.
