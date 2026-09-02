{
  perSystem =
    {
      craneLib,
      cargoArtifacts,
      rustSrc,
      ...
    }:
    {
      checks.rust-test = craneLib.cargoNextest {
        inherit cargoArtifacts;
        src = rustSrc;
        cargoNextestExtraArgs = "--workspace --no-fail-fast --no-tests=pass";
      };
    };
}
