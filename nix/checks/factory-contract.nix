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
    # GitHub introduced jobs.<job_id>.cache-mode after actionlint v1.7.12.
    # check-support-policy.py validates the exact read-only placement and rejects
    # cache-mode everywhere else; keep all other actionlint diagnostics active.
    actionlint \
      -ignore 'unexpected key "cache-mode" for "job" section' \
      .github/workflows/*.yml
    openspec doctor
    openspec validate --all --strict --no-interactive
  '';

  installPhase = "touch $out";
}
