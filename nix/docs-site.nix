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
          cacert
          coreutils
          findutils
          fontconfig
          graphviz
          gnugrep
          lychee
          mdbook
        ];
        SSL_CERT_FILE = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";
        FONTCONFIG_FILE = "${pkgs.fontconfig.out}/etc/fonts/fonts.conf";
        phases = [
          "unpackPhase"
          "buildPhase"
        ];
        buildPhase = ''
          runHook preBuild
          export XDG_CACHE_HOME="$TMPDIR/fontconfig-cache"
          mkdir -p "$XDG_CACHE_HOME"
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
