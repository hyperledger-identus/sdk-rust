{
  lib,
  stdenv,
  taplo,
}:

stdenv.mkDerivation {
  name = "lint-toml";
  src = lib.cleanSourceWith {
    filter =
      path: type:
      let
        baseName = builtins.baseNameOf path;
        relativePath = lib.removePrefix (toString ./../..) (toString path);
        excludedPrefixes = [
          "/target"
          "/node_modules"
          "/.git"
        ];
        isExcluded = lib.any (p: lib.hasPrefix p relativePath) excludedPrefixes;
      in
      if isExcluded then
        false
      else if type == "directory" then
        true
      else
        lib.hasSuffix ".toml" baseName;
    src = ./../..;
  };

  nativeBuildInputs = [ taplo ];

  buildPhase = "true";

  doCheck = true;

  checkPhase = ''
    echo "Running taplo check..."
    taplo check
    echo "Running taplo format --check..."
    taplo format --check
  '';

  installPhase = "touch $out";
}
