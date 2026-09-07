{
  imports = [
    ./rust-gates.nix
  ];

  perSystem =
    {
      pkgs,
      craneLib,
      etalonCraneLib,
      msrvCraneLib,
      inputs',
      ...
    }:
    let
      # Like `craneLib.cleanCargoSource ./../..` but also keeps trybuild
      # `.stderr` fixtures (the compile-error expectation files under
      # `crates/derive/tests/ui/`), which the default cargo source filter
      # strips. Without them the trybuild ui tests treat every `.stderr` as
      # missing and fail.
      cleanedSrc = pkgs.lib.cleanSourceWith {
        src = pkgs.lib.cleanSource ./../..;
        filter =
          path: type:
          craneLib.filterCargoSources path type || pkgs.lib.hasSuffix ".stderr" (baseNameOf (toString path));
      };
      cargoArtifacts = craneLib.buildDepsOnly {
        src = cleanedSrc;
      };
      etalonCargoArtifacts = etalonCraneLib.buildDepsOnly {
        src = cleanedSrc;
        cargoExtraArgs = "--locked --workspace --all-features";
      };
      msrvCargoArtifacts = msrvCraneLib.buildDepsOnly {
        src = cleanedSrc;
        cargoExtraArgs = "--locked --workspace --all-features";
      };
    in
    {
      _module.args = {
        inherit cargoArtifacts etalonCargoArtifacts msrvCargoArtifacts;
        rustSrc = cleanedSrc;
      };

      checks = {
        factory-contract = pkgs.callPackage ./factory-contract.nix {
          openspec = inputs'.openspec.packages.default;
        };
        lint-nix = pkgs.callPackage ./lint-nix.nix { };
        lint-toml = pkgs.callPackage ./lint-toml.nix { };
        lint-text = pkgs.callPackage ./lint-text.nix { };
      };
    };
}
