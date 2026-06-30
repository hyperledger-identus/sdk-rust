{
  perSystem =
    {
      pkgs,
      toolchain,
      ...
    }:
    {
      apps = {
        format = {
          type = "app";
          program = "${pkgs.lib.getExe (pkgs.callPackage ./format.nix { cargo = toolchain; })}";
          meta.description = "Format Rust and Nix files";
        };

        format-nix = {
          type = "app";
          program = "${pkgs.lib.getExe (pkgs.callPackage ./format-nix.nix { })}";
          meta.description = "Format Nix files using nixfmt";
        };
      };
    };
}
