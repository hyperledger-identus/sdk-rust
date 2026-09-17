{
  perSystem =
    { pkgs, ... }:
    let
      source = pkgs.lib.fileset.toSource {
        root = ./..;
        fileset = pkgs.lib.fileset.unions [
          ../site
          ../scripts/build-docs-site.sh
        ];
      };
      docsSite = pkgs.stdenvNoCC.mkDerivation {
        pname = "sdk-rust-docs-site";
        version = "0.1.0";
        src = source;
        nativeBuildInputs = with pkgs; [
          bash
          coreutils
          findutils
          graphviz
          gnugrep
          lychee
          mdbook
        ];
        phases = [
          "unpackPhase"
          "buildPhase"
        ];
        buildPhase = ''
          runHook preBuild
          patchShebangs scripts/build-docs-site.sh
          scripts/build-docs-site.sh "$out"
          runHook postBuild
        '';
        meta = {
          description = "Reproducible SDK-Rust architecture and release handbook";
          license = pkgs.lib.licenses.asl20;
          platforms = pkgs.lib.platforms.unix;
        };
      };
    in
    {
      packages.docs-site = docsSite;
      checks.docs-site = docsSite;
    };
}
