{
  perSystem =
    {
      msrvCraneLib,
      msrvCargoArtifacts,
      rustSrc,
      ...
    }:
    {
      # Build the complete current workspace surface with the declared stable
      # floor. The NeoPRISM-etalon nightly checks remain separate.
      checks.rust-msrv = msrvCraneLib.cargoBuild {
        cargoArtifacts = msrvCargoArtifacts;
        src = rustSrc;
        cargoExtraArgs = "--locked --workspace --all-targets";
        doCheck = false;
      };
    };
}
