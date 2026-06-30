{ inputs, ... }:
{
  perSystem =
    {
      pkgs,
      ...
    }:
    let
      toolchain = pkgs.rust-bin.stable.latest.default.override {
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
