{
  perSystem =
    {
      pkgs,
      toolchain,
      inputs',
      ...
    }:
    {
      apps = {
        format = {
          type = "app";
          program = "${pkgs.lib.getExe (pkgs.callPackage ./format.nix { cargo = toolchain; })}";
          meta.description = "Format Rust, TOML, and Nix files";
        };

        format-nix = {
          type = "app";
          program = "${pkgs.lib.getExe (pkgs.callPackage ./format-nix.nix { })}";
          meta.description = "Format Nix files using nixfmt";
        };

        format-toml = {
          type = "app";
          program = "${pkgs.lib.getExe (pkgs.callPackage ./format-toml.nix { })}";
          meta.description = "Format TOML files using taplo";
        };

        factory = {
          type = "app";
          program = "${pkgs.lib.getExe (
            pkgs.callPackage ./factory.nix { openspec = inputs'.openspec.packages.default; }
          )}";
          meta.description = "Run the repository AI Software Factory";
        };

        crypto-candidate = {
          type = "app";
          program = "${pkgs.lib.getExe (
            pkgs.callPackage ./crypto-candidate.nix {
              cargoCyclonedx = pkgs.cargo-cyclonedx;
              cargoPublicApi = pkgs.cargo-public-api;
              cargoSemverChecks = pkgs.cargo-semver-checks;
              inherit toolchain;
            }
          )}";
          meta.description = "Prepare and verify the unpublished crypto candidate";
        };
      };
    };
}
