{ ... }:
{
  perSystem =
    { pkgs, sdk-rustLib, ... }:
    let
      wasm-bindgen-cli = pkgs.callPackage ./wasm-bindgen-cli.nix { inherit sdk-rustLib; };
      demo-dir = pkgs.callPackage ./demo-dir.nix { inherit sdk-rustLib; };
      uniffi-bindgen = pkgs.callPackage ./uniffi-bindgen.nix { inherit sdk-rustLib; };
    in
    {
      packages = {
        inherit wasm-bindgen-cli demo-dir uniffi-bindgen;
      };
    };
}
