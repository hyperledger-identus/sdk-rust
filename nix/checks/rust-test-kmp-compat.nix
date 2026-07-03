{
  perSystem =
    {
      craneLib,
      cargoArtifacts,
      rustSrc,
      ...
    }:
    {
      # Exercises the opt-in `kmp-compat` surface of `identus-crypto`, which
      # the default-feature `rust-test` check never compiles. Distinct from
      # `--all-features` (would also pull `getrandom`/`deterministic` and drop
      # the default-surface "absent without feature" coverage).
      checks.rust-test-kmp-compat = craneLib.cargoNextest {
        inherit cargoArtifacts;
        src = rustSrc;
        cargoBuildFeatures = [ "kmp-compat" ];
        cargoNextestExtraArgs = "--no-fail-fast --no-tests=pass";
      };
    };
}
