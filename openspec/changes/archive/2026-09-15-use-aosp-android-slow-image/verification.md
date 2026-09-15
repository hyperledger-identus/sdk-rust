# Verification receipt

- **Verification status:** passed locally
- **Verification date:** 2026-09-15
- **Planning head:** `e127bb6db8c7b62cc9eb70d076961f72630284f4`
- **Implementation head:** `cd816d6e62e2b92f6a4a08720efe2d0cbca05915`
- **Base:** `develop@47136ed8bc73faba34c78d88768136cd6841364d`

## Passed locally

- `python3 scripts/tests/support-policy.py`: 194 mutation tests passed.
- `bash -n scripts/check-uniffi-did-android.sh`: passed.
- `nix develop --command actionlint .github/workflows/nix-checks.yml`: passed.
- `./scripts/factory check --change use-aosp-android-slow-image`: all 71
  specifications/changes and every factory structural contract passed.
- `nix flake check --fallback`: all 30 compatible aarch64-darwin checks passed,
  including factory/policy, Rust 1.98.1 build/test/Clippy/docs, feature profiles,
  MSRV evidence, WASM, Android/iOS compile targets, deny/audit and text/TOML/Nix
  linting. Nix reported x86_64-linux as locally incompatible.
- `git diff --check`: passed.
- Three implementation-branch commits through the implementation head have
  valid local GPG signatures and DCO trailers.

## Deliberate negative evidence

Mutation tests reject a Google Play workflow package, blanket
`sdkmanager --licenses`, a Google Play verifier package and a Google Play image
directory. The structural checker also rejects missing/out-of-order SDK root,
tool, NDK, platform and image installation evidence.

## Routed hosted evidence

This host has no Android SDK, so it cannot install or boot the AOSP image. The
protected pull request supplies independent exact-head Linux CI. After merge,
one complete manual slow run at the exact merged `develop` SHA is authoritative
for candidate API evidence, macOS Clippy/Nix, package installation, AVD boot and
AAR/JNA behavior. The first successful natural Monday run remains issue #276's
final scheduling acceptance.

## Exclusions

No physical device, additional API/ABI, Google service, publication, signing,
Maven distribution, Keystore, application lifecycle or supported Android claim
is tested or introduced. The exact SDK path does not pin the underlying system
image archive checksum or future catalog revision.
