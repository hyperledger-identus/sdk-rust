{
  git,
  msrvToolchain,
  python3,
  writeShellApplication,
}:

writeShellApplication {
  name = "did-candidate-matrix-msrv";
  runtimeInputs = [
    git
    python3
    msrvToolchain
  ];
  text = ''
    if [[ ! -f flake.nix ]]; then
      echo "Error: run this app from the sdk-rust repository root" >&2
      exit 1
    fi
    exec ./scripts/prepare-did-candidate.py --matrix-toolchain msrv "$@"
  '';
}
