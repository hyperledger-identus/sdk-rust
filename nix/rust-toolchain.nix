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
          "llvm-tools-preview"
          "rust-src"
          "rust-analyzer"
        ];
        targets = [
          "aarch64-apple-ios"
          "aarch64-linux-android"
          "wasm32-unknown-unknown"
        ];
      };
      # During the temporary active-development phase, compatibility and MSRV
      # labels intentionally resolve to the same exact stable compiler. This
      # keeps existing gate identities without paying for three compilers.
      etalonToolchain = toolchain;
      msrvToolchain = toolchain;
      # libFuzzer sanitizer instrumentation remains a tooling-only nightly
      # exception and is never an ordinary SDK compatibility provider.
      fuzzToolchain = pkgs.rust-bin.nightly."2026-03-18".default.override {
        extensions = [
          "rust-src"
        ];
      };
      craneLib = (inputs.crane.mkLib pkgs).overrideToolchain toolchain;
      etalonCraneLib = craneLib;
      msrvCraneLib = craneLib;
    in
    {
      _module.args = {
        inherit
          craneLib
          etalonCraneLib
          etalonToolchain
          fuzzToolchain
          msrvCraneLib
          msrvToolchain
          toolchain
          ;
      };
    };
}
