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
      # `.stderr` fixtures, the four immutable error goldens and the bounded
      # OID4VCI interoperability packet, which the default cargo source filter
      # strips. Keep every exception path-scoped so planning evidence and
      # unrelated data files do not enter Rust build sources.
      cleanedSrc = pkgs.lib.cleanSourceWith {
        src = pkgs.lib.cleanSource ./../..;
        filter =
          path: type:
          let
            sourcePath = toString path;
            oid4vciInteropRoot = "${toString ./../..}/crates/oid4vci/tests/fixtures/interop-v1";
          in
          craneLib.filterCargoSources path type
          || pkgs.lib.hasSuffix ".stderr" (baseNameOf sourcePath)
          || pkgs.lib.hasSuffix "/crates/credentials/tests/fixtures/credentials-error-contract-v1.csv" sourcePath
          || pkgs.lib.hasSuffix "/crates/presentations/tests/fixtures/presentations-error-contract-v1.csv" sourcePath
          || pkgs.lib.hasSuffix "/crates/jose/tests/fixtures/jose-error-contract-v1.csv" sourcePath
          || pkgs.lib.hasSuffix "/crates/oid4vci/tests/fixtures/oid4vci-error-contract-v1.csv" sourcePath
          || pkgs.lib.hasPrefix oid4vciInteropRoot sourcePath;
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
          test -f "$src/crates/jose/tests/fixtures/jose-error-contract-v1.csv"
          test -f "$src/crates/oid4vci/tests/fixtures/oid4vci-error-contract-v1.csv"
          test -f "$src/crates/oid4vci/tests/fixtures/interop-v1/manifest.json"
          test "$(find "$src/crates/oid4vci/tests/fixtures/interop-v1" -type f | wc -l)" -eq 10
          if find "$src/openspec/changes" -type f \
            \( -name credentials-error-contract-v1.csv \
              -o -name presentations-error-contract-v1.csv \
              -o -name jose-error-contract-v1.csv \
              -o -name oid4vci-error-contract-v1.csv \) \
            -print -quit | grep -q .; then
            echo "planning error golden entered cleaned Rust sources" >&2
            exit 1
          fi
          touch "$out"
        '';
      };
    };
}
