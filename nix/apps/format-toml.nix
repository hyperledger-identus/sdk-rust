{
  taplo,
  findutils,
  writeShellApplication,
}:

writeShellApplication {
  name = "format-toml";
  runtimeInputs = [
    taplo
    findutils
  ];
  text = ''
    set -e
    if [[ ! -f flake.nix ]]; then
      echo "Error: This app must be run from the repo root (where flake.nix resides)" >&2
      exit 1
    fi
    echo "Running taplo format on *.toml files..."
    find . -name '*.toml' -type f \
      -not -path './target/*' \
      -not -path './node_modules/*' \
      -not -path './.git/*' \
      -print0 | xargs -0 -r taplo format
    echo "Done!"
  '';
}
