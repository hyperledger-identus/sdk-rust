## 1. Reference Contract

- [x] 1.1 Inspect current NeoPRISM `main` and record the immutable source,
  Rust, rust-overlay, nixpkgs and Nix package versions.
- [x] 1.2 Record the alignment decision and unchanged MSRV boundary in ADR 0002
  and the `nix-tooling` delta specification.

## 2. Toolchain Alignment

- [x] 2.1 Replace `stable.latest` with Rust nightly `2026-03-18` while retaining
  the existing components and WASM target.
- [x] 2.2 Align the root nixpkgs and rust-overlay lock entries with NeoPRISM and
  verify that the devshell exposes the expected Rust and Nix versions.
- [x] 2.3 Update contributor and architecture documentation to identify
  NeoPRISM as the version etalon without claiming automatic synchronization.
- [x] 2.4 Align the three compiler-sensitive UI snapshots with the pinned
  etalon compiler's diagnostic wording.

## 3. Verification and Finalization

- [x] 3.1 Pass strict OpenSpec and factory validation.
- [x] 3.2 Pass the complete Nix flake check on the available host and record any
  platform-specific checks left to CI.
- [x] 3.3 Synchronize the current capability specification, archive this change
  and produce the factory readiness receipt.
