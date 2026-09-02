{ inputs, ... }:
{
  perSystem =
    {
      pkgs,
      ...
    }:
    let
      # Match NeoPRISM's pinned Rust toolchain. Keep the exact date here rather
      # than following `latest`, so local and CI diagnostics do not drift.
      toolchain = pkgs.rust-bin.nightly."2026-03-18".default.override {
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
      # The stable consumer floor is intentionally independent from the
      # NeoPRISM-etalon nightly. A newer compiler passing cannot prove MSRV.
      msrvToolchain = pkgs.rust-bin.stable."1.85.0".minimal;
      msrvCraneLib = (inputs.crane.mkLib pkgs).overrideToolchain msrvToolchain;
    in
    {
      _module.args = {
        inherit
          craneLib
          msrvCraneLib
          msrvToolchain
          toolchain
          ;
      };
    };
}
