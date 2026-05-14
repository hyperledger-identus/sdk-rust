{ pkgs, identus-crypto-wasm }:

let
  inherit (pkgs) lib;
in

pkgs.stdenv.mkDerivation {
  pname = "identus-crypto-wasm-demo-assets";
  version = "0.1.0";

  src = builtins.path {
    path = ./../../examples/wasm-app;
    name = "wasm-app-src";
  };

  dontBuild = true;

  installPhase = ''
    mkdir -p "$out"

    # Copy the WASM/JS bindings from the identus-crypto-wasm package.
    cp -r ${identus-crypto-wasm}/* "$out/"

    # Copy static example app files (HTML, CSS, JS).
    cp -r "$src"/* "$out/"
  '';
}
