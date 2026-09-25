{
  git,
  python3,
  toolchain,
  writeShellApplication,
}:

writeShellApplication {
  name = "did-candidate-matrix-primary";
  runtimeInputs = [
    git
    python3
    toolchain
  ];
  text = ''
    if [[ ! -f flake.nix ]]; then
      echo "Error: run this app from the sdk-rust repository root" >&2
      exit 1
    fi
    exec ./scripts/prepare-did-candidate.py --matrix-toolchain primary "$@"
  '';
}
