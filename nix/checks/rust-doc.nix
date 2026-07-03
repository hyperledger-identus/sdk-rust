{
  perSystem =
    {
      craneLib,
      cargoArtifacts,
      rustSrc,
      ...
    }:
    {
      checks.rust-doc = craneLib.cargoDoc {
        inherit cargoArtifacts;
        src = rustSrc;
        cargoDocExtraArgs = "--no-deps";
      };
    };
}
