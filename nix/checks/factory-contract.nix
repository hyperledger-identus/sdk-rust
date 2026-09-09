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
  python3,
  nodejs_24,
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
    python3
    nodejs_24
  ];

  buildPhase = "true";

  doCheck = true;
  checkPhase = ''
    export OPENSPEC_TELEMETRY=0
    patchShebangs scripts
    scripts/tests/factory-contract.sh
    node --test scripts/tests/factory-operations.mjs
    scripts/check-factory.sh .
    actionlint .github/workflows/*.yml
    openspec doctor
    openspec validate --all --strict --no-interactive
  '';

  installPhase = "touch $out";
}
