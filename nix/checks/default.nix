{
  imports = [
    ./rust-fmt.nix
    ./rust-clippy.nix
    ./rust-test.nix
    ./rust-deny.nix
    ./rust-audit.nix
  ];

  perSystem =
    {
      pkgs,
      craneLib,
      ...
    }:
    let
      cargoArtifacts = craneLib.buildDepsOnly {
        src = craneLib.cleanCargoSource ./../..;
      };
    in
    {
      _module.args.cargoArtifacts = cargoArtifacts;

      checks.lint-nix = pkgs.callPackage ./lint-nix.nix { };
    };
}
