{
  description = "Identus SDK for Rust";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    # Keep the Rust/Nix baseline aligned with NeoPRISM, the Identus Rust
    # repository used as the toolchain etalon. Exact revisions live in
    # flake.lock; see ADR 0002.
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    devshell.url = "github:numtide/devshell";
    rust-overlay.url = "github:oxalica/rust-overlay";
    # Current stable validation is independent from NeoPRISM's historical
    # nightly overlay pin. Keep both inputs explicit and lock-reviewed.
    stable-rust-overlay.url = "github:oxalica/rust-overlay/ca7f624be3935a5bc46d2c240515491ab8675503";
    crane.url = "github:ipetkov/crane";
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
    openspec.url = "github:Fission-AI/OpenSpec";
  };

  outputs =
    inputs@{
      flake-parts,
      nixpkgs,
      rust-overlay,
      stable-rust-overlay,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.devshell.flakeModule
        ./nix/rust-toolchain.nix
        ./nix/devshells
        ./nix/checks
        ./nix/apps
      ];

      systems = [
        "x86_64-linux"
        "aarch64-darwin"
      ];

      perSystem =
        { system, ... }:
        {
          _module.args = {
            pkgs = import nixpkgs {
              inherit system;
              overlays = [ (import rust-overlay) ];
            };
            stablePkgs = import nixpkgs {
              inherit system;
              overlays = [ (import stable-rust-overlay) ];
            };
          };
        };
    };
}
