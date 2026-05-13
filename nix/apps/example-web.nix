{ pkgs, sdk-rustLib }:

let
  demoDir = pkgs.callPackage ./demo-dir.nix { inherit sdk-rustLib; };
in

pkgs.writeShellScriptBin "example-web" ''
  set -euo pipefail
  echo "Serving Identus Crypto WASM demo at http://localhost:8080"
  echo "Press Ctrl+C to stop."
  exec ${pkgs.python3}/bin/python3 -m http.server 8080 --directory ${demoDir}
''
