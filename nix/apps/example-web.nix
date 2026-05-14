{ pkgs, sdk-rustLib }:

let
  identus-crypto-wasm = pkgs.callPackage ../packages/identus-crypto-wasm.nix { inherit sdk-rustLib; };
  exampleWebappAssets = pkgs.callPackage ../packages/example-webapp-assets.nix {
    inherit identus-crypto-wasm;
  };
in

pkgs.writeShellScriptBin "example-web" ''
  set -euo pipefail
  echo "Serving Identus Crypto WASM demo at http://localhost:8080"
  echo "Press Ctrl+C to stop."
  exec ${pkgs.python3}/bin/python3 -m http.server 8080 --directory ${exampleWebappAssets}
''
