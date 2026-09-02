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
        targets = [ "wasm32-unknown-unknown" ];
      };
      craneLib = (inputs.crane.mkLib pkgs).overrideToolchain toolchain;
    in
    {
      _module.args.toolchain = toolchain;
      _module.args.craneLib = craneLib;
    };
}
