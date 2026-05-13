{ rust-bin, rust-overlay }:

let
  nightlyVersion = "2026-05-12";
  rustOverrideArgs = {
    extensions = [
      "rust-src"
      "rust-analyzer"
    ];
    targets = [
      "wasm32-unknown-unknown"
      "aarch64-linux-android"
      "armv7-linux-androideabi"
      "x86_64-linux-android"
      "i686-linux-android"
    ];
  };
in
rec {
  rust = mkRust { };

  rustMinimal = mkRust { minimal = true; };

  mkRust =
    {
      minimal ? false,
    }:
    if minimal then
      rust-bin.nightly.${nightlyVersion}.minimal
    else
      rust-bin.nightly.${nightlyVersion}.default.override rustOverrideArgs;

  mkRustCross =
    {
      pkgsCross,
      minimal ? false,
    }:
    let
      rust-bin = rust-overlay.lib.mkRustBin { } pkgsCross.buildPackages;
    in
    if minimal then
      rust-bin.nightly.${nightlyVersion}.minimal
    else
      rust-bin.nightly.${nightlyVersion}.default.override rustOverrideArgs;

  cargoLock = {
    lockFile = ../Cargo.lock;
    outputHashes = {
      "midnight-base-crypto-1.0.0" = "sha256-D3Np8WB8mr6gdHW8SA9EcpO8NflKUTj5qXY6ssO62N8=";
    };
  };
}
