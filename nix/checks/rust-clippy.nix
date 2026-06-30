{
  perSystem =
    { craneLib, cargoArtifacts, ... }:
    {
      checks.rust-clippy = craneLib.cargoClippy {
        inherit cargoArtifacts;
        src = craneLib.cleanCargoSource ./../..;
        cargoClippyExtraArgs = "-- -D warnings";
      };
    };
}
