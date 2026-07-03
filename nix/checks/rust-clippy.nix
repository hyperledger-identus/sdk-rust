{
  perSystem =
    {
      craneLib,
      cargoArtifacts,
      rustSrc,
      ...
    }:
    {
      checks.rust-clippy = craneLib.cargoClippy {
        inherit cargoArtifacts;
        src = rustSrc;
        cargoClippyExtraArgs = "-- -D warnings";
      };
    };
}
