{
  nixfmt,
  findutils,
  writeShellApplication,
}:

writeShellApplication {
  name = "format-nix";
  runtimeInputs = [
    nixfmt
    findutils
  ];
  text = ''
    set -e
    if [[ ! -f flake.nix ]]; then
      echo "Error: This app must be run from the repo root (where flake.nix resides)" >&2
      exit 1
    fi
    echo "Formatting flake.nix..."
    nixfmt flake.nix
    echo "Formatting nix/**/*.nix files..."
    find nix -name '*.nix' -type f -print0 | xargs -0 -r nixfmt
    echo "Done!"
  '';
}
