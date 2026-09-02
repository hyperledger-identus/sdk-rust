{
  lib,
  stdenvNoCC,
  bash,
  actionlint,
  coreutils,
  findutils,
  gitMinimal,
  gnugrep,
  openspec,
}:

stdenvNoCC.mkDerivation {
  name = "factory-contract";
  src = lib.cleanSource ./../..;

  nativeBuildInputs = [
    actionlint
    bash
    coreutils
    findutils
    gitMinimal
    gnugrep
    openspec
  ];

  buildPhase = "true";

  doCheck = true;
  checkPhase = ''
    export OPENSPEC_TELEMETRY=0
    patchShebangs scripts
    scripts/tests/factory-contract.sh
    scripts/check-factory.sh .
    actionlint .github/workflows/*.yml
    openspec doctor
    openspec validate --all --strict --no-interactive
  '';

  installPhase = "touch $out";
}
