{ pkgs, sdk-rustLib }:

let
  inherit (sdk-rustLib) rustTools;

  # Build wasm-bindgen-cli from source using the same nightly toolchain.
  # This ensures the CLI version matches the wasm-bindgen crate (0.2.121)
  # used by identus-crypto-wasm.
  rustPlatform = pkgs.makeRustPlatform {
    cargo = rustTools.rust;
    rustc = rustTools.rust;
  };
in

rustPlatform.buildRustPackage rec {
  pname = "wasm-bindgen-cli";
  version = "0.2.121";

  src = pkgs.fetchCrate {
    inherit pname version;
    sha256 = "sha256-ZOMgFNOcGkO66Jz/Z83eoIu+DIzo3Z/vq6Z5g6BDY/w=";
  };

  cargoHash = "sha256-DPdCDPTAPBrbqLUqnCwQu1dePs9lGg85JCJOCIr9qjU=";

  nativeBuildInputs = [ pkgs.pkg-config ];

  buildInputs = [ pkgs.openssl pkgs.zlib ];

  doCheck = false;

  meta = {
    description = "CLI tool for generating JavaScript bindings from WASM";
    homepage = "https://github.com/rustwasm/wasm-bindgen";
  };
}
