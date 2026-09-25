# Qualify the DID candidate compiler and target matrix

## Why

The two-package DID `0.1.0-rc.1` train has deterministic archives, public-API
origins, SBOM evidence, and an engineer-facing review page. Its current
workspace gates compile canonical unpublished `0.0.0` sources and therefore do
not prove that release-shaped staged manifests resolve and build on the M5
compiler/target contract. The generic DID crate and its optional Axum adapter
also need distinct target claims.

## What changes

- Add a closed package-by-package matrix to the DID candidate descriptor.
- Test both staged packages on Linux x86_64 and macOS ARM64 with Rust 1.98.1,
  and independently compile/check them with the Rust 1.89.0 MSRV.
- Compile-check staged `identus-did` for browser WASM, Android ARM64, and iOS
  ARM64 on both compilers; explicitly record the HTTP adapter as unsupported on
  those targets.
- Add a credential-free matrix mode to the existing DID candidate builder and
  primary/MSRV Nix apps over the pinned toolchains.
- Run the host lanes only in native weekly/manual slow CI, upload bounded lane
  receipts, and aggregate them into one exact-SHA matrix receipt.
- Add structural and mutation tests for package, compiler, target, staged
  version, receipt, command, and claim drift.
- Accept an ADR for the staged-source and host-adapter boundary.

## Capabilities

### Modified capabilities

- `release-candidate-trains`: require a closed, staged-source compiler/target
  matrix and aggregate receipt before a candidate may advance.
- `sdk-support-policy`: preserve one required fast lane while candidate host,
  MSRV, and portable-target qualification remains weekly/manual slow evidence.

## Non-goals

No runtime/device/browser proof, FFI/binding package, publication, tag, GitHub
release, crates.io upload, repository setting, support for the HTTP adapter on
portable targets, downstream mutation, or automatic slow-workflow dispatch is
authorized.

## Delivery

Issue #387 owns implementation. Its PR targets protected `develop` and needs
only normal exact-head required CI. The next natural or explicitly authorized
manual slow run supplies the cross-host matrix artifact to final M5 issue #388;
this change does not manufacture or rerun that external evidence.
