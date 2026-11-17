{ inputs, ... }:
{
  perSystem =
    {
      pkgs,
      stablePkgs,
      ...
    }:
    let
      # Primary development, compatibility, and declared-floor compiler.
      # Rust 1.99.1 is the corrected point release selected by ADR 0200.
      toolchain = stablePkgs.rust-bin.stable."1.99.1".default.override {
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
      # Primary, etalon, and MSRV intentionally share one exact stable
      # compiler for the 0.2 source line. Keep separately named providers so
      # policy drift remains observable.
      etalonToolchain = toolchain;
      msrvToolchain = stablePkgs.rust-bin.stable."1.99.1".minimal.override {
        targets = [
          "aarch64-apple-ios"
          "aarch64-linux-android"
          "wasm32-unknown-unknown"
        ];
      };
      # libFuzzer sanitizer instrumentation remains a tooling-only nightly
      # exception and is never an ordinary SDK compatibility provider.
      # Keep the sanitizer compiler newer than the declared 1.99.1 floor so
      # Cargo can build the same workspace graph under the fuzz profile.
      fuzzToolchain = stablePkgs.rust-bin.nightly."2026-10-15".default.override {
        extensions = [
          "rust-src"
        ];
      };
      craneLib = (inputs.crane.mkLib pkgs).overrideToolchain toolchain;
      etalonCraneLib = craneLib;
      msrvCraneLib = (inputs.crane.mkLib pkgs).overrideToolchain msrvToolchain;
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
