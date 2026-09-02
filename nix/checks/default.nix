{
  imports = [
    ./rust-fmt.nix
    ./rust-msrv.nix
    ./rust-clippy.nix
    ./rust-clippy-kmp-compat.nix
    ./rust-feature-matrix.nix
    ./rust-test.nix
    ./rust-test-kmp-compat.nix
    ./rust-build-wasm32.nix
    ./rust-build-mobile.nix
    ./rust-deny.nix
    ./rust-audit.nix
    ./rust-doc.nix
  ];

  perSystem =
    {
      pkgs,
      craneLib,
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
      msrvCargoArtifacts = msrvCraneLib.buildDepsOnly {
        src = cleanedSrc;
      };
    in
    {
      _module.args = {
        inherit cargoArtifacts msrvCargoArtifacts;
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
