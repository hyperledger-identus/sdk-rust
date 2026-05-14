{ ... }:
{
  perSystem =
    { pkgs, sdk-rustLib, ... }:
    let
      wasm-bindgen-cli = pkgs.callPackage ./wasm-bindgen-cli.nix { inherit sdk-rustLib; };
      identus-crypto-wasm = pkgs.callPackage ./identus-crypto-wasm.nix { inherit sdk-rustLib; };
      example-webapp-assets = pkgs.callPackage ./example-webapp-assets.nix {
        inherit identus-crypto-wasm;
      };
      uniffi-bindgen = pkgs.callPackage ./uniffi-bindgen.nix { inherit sdk-rustLib; };
    in
    {
      packages = {
        inherit
          wasm-bindgen-cli
          identus-crypto-wasm
          example-webapp-assets
          uniffi-bindgen
          ;
      };
    };
}
