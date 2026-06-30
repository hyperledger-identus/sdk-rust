{
  perSystem =
    { craneLib, cargoArtifacts, ... }:
    {
      checks.rust-test = craneLib.cargoNextest {
        inherit cargoArtifacts;
        src = craneLib.cleanCargoSource ./../..;
        cargoNextestExtraArgs = "--no-fail-fast --no-tests=pass";
      };
    };
}
