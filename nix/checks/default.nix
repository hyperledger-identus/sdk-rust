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
      # `.stderr` fixtures and the two immutable error goldens, which the
      # default cargo source filter strips. Keep the CSV suffixes exact so no
      # planning golden or unrelated CSV enters Rust build sources.
      cleanedSrc = pkgs.lib.cleanSourceWith {
        src = pkgs.lib.cleanSource ./../..;
        filter =
          path: type:
          let
            sourcePath = toString path;
          in
          craneLib.filterCargoSources path type
          || pkgs.lib.hasSuffix ".stderr" (baseNameOf sourcePath)
          || pkgs.lib.hasSuffix "/crates/credentials/tests/fixtures/credentials-error-contract-v1.csv" sourcePath
          || pkgs.lib.hasSuffix "/crates/presentations/tests/fixtures/presentations-error-contract-v1.csv" sourcePath;
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
        rust-source-contract = pkgs.runCommand "rust-source-contract" { src = cleanedSrc; } ''
          test -f "$src/crates/credentials/tests/fixtures/credentials-error-contract-v1.csv"
          test -f "$src/crates/presentations/tests/fixtures/presentations-error-contract-v1.csv"
          test ! -e "$src/openspec/changes/decompose-public-error-contracts/golden/credentials-error-contract-v1.csv"
          test ! -e "$src/openspec/changes/decompose-presentation-error-contracts/golden/presentations-error-contract-v1.csv"
          touch "$out"
        '';
      };
    };
}
