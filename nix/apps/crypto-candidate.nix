{
  cargoCyclonedx,
  cargoPublicApi,
  cargoSemverChecks,
  git,
  python3,
  toolchain,
  writeShellApplication,
}:

writeShellApplication {
  name = "crypto-candidate";
  runtimeInputs = [
    cargoCyclonedx
    cargoPublicApi
    cargoSemverChecks
    git
    python3
    toolchain
  ];
  text = ''
    if [[ ! -f flake.nix ]]; then
      echo "Error: run this app from the sdk-rust repository root" >&2
      exit 1
    fi
    exec ./scripts/prepare-crypto-candidate.py "$@"
  '';
}
