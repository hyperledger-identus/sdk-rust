{
  lib,
  stdenv,
  markdownlint-cli2,
  yamllint,
  editorconfig-checker,
  shellcheck,
  findutils,
}:

stdenv.mkDerivation {
  name = "lint-text";
  src = lib.cleanSourceWith {
    filter =
      path: _:
      let
        baseName = builtins.baseNameOf path;
        relativePath = lib.removePrefix (toString ./../..) (toString path);
        excludedPrefixes = [
          "/target"
          "/node_modules"
          "/.git"
          "/openspec"
          "/.pi"
          "/.agents"
          "/.claude"
        ];
        isExcludedDir = lib.any (p: lib.hasPrefix p relativePath) excludedPrefixes;
        isExcludedFile = baseName == "Cargo.lock" || baseName == "flake.lock";
      in
      !(isExcludedDir || isExcludedFile);
    src = ./../..;
  };

  nativeBuildInputs = [
    markdownlint-cli2
    yamllint
    editorconfig-checker
    shellcheck
    findutils
  ];

  buildPhase = "true";

  doCheck = true;

  checkPhase = ''
    echo "Running markdownlint-cli2..."
    markdownlint-cli2 "**/*.md" "**/*.markdown"

    echo "Running yamllint..."
    yamllint -c .yamllint.yml .

    echo "Running editorconfig-checker..."
    editorconfig-checker

    echo "Running shellcheck..."
    find . -name '*.sh' -type f \
      -not -path './target/*' \
      -not -path './node_modules/*' \
      -not -path './.git/*' \
      -not -path './openspec/*' \
      -not -path './.pi/*' \
      -not -path './.agents/*' \
      -not -path './.claude/*' \
      -print0 | xargs -0 -r shellcheck
  '';

  installPhase = "touch $out";
}
