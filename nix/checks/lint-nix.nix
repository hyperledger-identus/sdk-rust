{
  lib,
  stdenv,
  deadnix,
  statix,
  nixfmt,
}:

stdenv.mkDerivation {
  name = "lint-nix";
  src = lib.cleanSourceWith {
    filter =
      path: _:
      let
        baseName = builtins.baseNameOf path;
        relativePath = lib.removePrefix (toString ./../..) (toString path);
      in
      baseName == "flake.nix" || lib.hasPrefix "/nix" relativePath;
    src = ./../..;
  };

  nativeBuildInputs = [
    deadnix
    statix
    nixfmt
  ];

  buildPhase = "true";

  doCheck = true;

  checkPhase = ''
    echo "Running deadnix..."
    deadnix -f

    echo "Running statix..."
    statix check .

    echo "Running nixfmt..."
    nixfmt --check flake.nix
    find nix -name '*.nix' -type f -print0 | xargs -0 -I {} nixfmt --check {}
  '';

  installPhase = "touch $out";
}
