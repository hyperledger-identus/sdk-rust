{ pkgs, sdk-rustLib }:

let
  inherit (sdk-rustLib) rustTools;

  # Build uniffi-bindgen from source using the same nightly toolchain.
  # This ensures the CLI version matches the uniffi crate (0.31.1)
  # that TASK-9 will add as a workspace dependency for identus-crypto-uniffi.
  #
  # The binary is in the `uniffi` meta-crate (not `uniffi_bindgen`) with
  # the `cli` feature enabled.
  rustPlatform = pkgs.makeRustPlatform {
    cargo = rustTools.rust;
    rustc = rustTools.rust;
  };
in

rustPlatform.buildRustPackage rec {
  pname = "uniffi-bindgen";
  version = "0.31.1";

  src = pkgs.fetchzip {
    name = "uniffi-${version}.tar.gz";
    url = "https://crates.io/api/v1/crates/uniffi/${version}/download";
    sha256 = "sha256-1yefise49dFzcmp0/+eFANDFTE1/VWQD4C00KkwdVOI=";
    extension = "tar.gz";
  };

  cargoHash = "sha256-WIU/3UWfodH3FqdrHsk2+/0iXzd4fJg5lZz3zqreUHQ=";

  buildFeatures = [ "cli" ];

  nativeBuildInputs = [ pkgs.pkg-config ];

  buildInputs = [
    pkgs.openssl
    pkgs.zlib
  ]
  ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [ pkgs.darwin.libiconv ];

  doCheck = false;

  meta = {
    description = "Multi-language bindings generator for Rust (UniFFI)";
    homepage = "https://github.com/mozilla/uniffi-rs";
    license = pkgs.lib.licenses.mpl20;
  };
}
