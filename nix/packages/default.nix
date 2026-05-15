_: {
  perSystem =
    { pkgs, sdk-rustLib, ... }:
    let
      wasm-bindgen-cli = pkgs.callPackage ./wasm-bindgen-cli.nix { inherit sdk-rustLib; };
      identus-crypto-wasm = pkgs.callPackage ./identus-crypto-wasm.nix { inherit sdk-rustLib; };
      uniffi-bindgen = pkgs.callPackage ./uniffi-bindgen.nix { inherit sdk-rustLib; };
      identus-crypto-uniffi-kotlin = pkgs.callPackage ./identus-crypto-uniffi-kotlin.nix {
        inherit sdk-rustLib;
        inherit uniffi-bindgen;
      };
      identus-crypto-uniffi-swift = pkgs.callPackage ./identus-crypto-uniffi-swift.nix {
        inherit sdk-rustLib;
        inherit uniffi-bindgen;
      };
    in
    {
      packages = {
        inherit
          wasm-bindgen-cli
          identus-crypto-wasm
          uniffi-bindgen
          identus-crypto-uniffi-kotlin
          identus-crypto-uniffi-swift
          ;
      };
    };
}
