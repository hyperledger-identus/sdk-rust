{ inputs, ... }:
{
  perSystem =
    {
      pkgs,
      stablePkgs,
      ...
    }:
    let
      # Primary development and validation compiler. Pin the corrected point
      # release rather than the affected Rust 1.98.0 compiler.
      toolchain = stablePkgs.rust-bin.stable."1.98.1".default.override {
        extensions = [
          "rust-src"
          "rust-analyzer"
        ];
        targets = [
          "aarch64-apple-ios"
          "aarch64-linux-android"
          "wasm32-unknown-unknown"
        ];
      };
      # Preserve NeoPRISM's compiler as an independent integration etalon.
      etalonToolchain = pkgs.rust-bin.nightly."2026-03-18".default.override {
        extensions = [
          "rust-src"
          "rust-analyzer"
        ];
        targets = [
          "aarch64-apple-ios"
          "aarch64-linux-android"
          "wasm32-unknown-unknown"
        ];
      };
      craneLib = (inputs.crane.mkLib pkgs).overrideToolchain toolchain;
      etalonCraneLib = (inputs.crane.mkLib pkgs).overrideToolchain etalonToolchain;
      # The stable consumer floor is intentionally independent from the
      # NeoPRISM-etalon nightly. A newer compiler passing cannot prove MSRV.
      msrvToolchain = pkgs.rust-bin.stable."1.85.0".minimal;
      msrvCraneLib = (inputs.crane.mkLib pkgs).overrideToolchain msrvToolchain;
    in
    {
      _module.args = {
        inherit
          craneLib
          etalonCraneLib
          etalonToolchain
          msrvCraneLib
          msrvToolchain
          toolchain
          ;
      };
    };
}
