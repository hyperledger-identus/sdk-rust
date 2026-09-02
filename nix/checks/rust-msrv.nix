{
  perSystem =
    {
      msrvCraneLib,
      msrvCargoArtifacts,
      rustSrc,
      ...
    }:
    {
      # Build every policy-declared feature surface with the stable floor.
      # The NeoPRISM-etalon nightly checks remain separate evidence.
      checks = {
        rust-msrv = msrvCraneLib.cargoBuild {
          cargoArtifacts = msrvCargoArtifacts;
          src = rustSrc;
          cargoExtraArgs = "--locked --workspace --all-targets";
          doCheck = false;
        };

        rust-msrv-crypto-minimal = msrvCraneLib.cargoBuild {
          cargoArtifacts = msrvCargoArtifacts;
          src = rustSrc;
          cargoExtraArgs = "--locked -p identus-crypto --lib --no-default-features";
          doCheck = false;
        };

        rust-msrv-crypto-kmp-compat = msrvCraneLib.cargoBuild {
          cargoArtifacts = msrvCargoArtifacts;
          src = rustSrc;
          cargoExtraArgs = "--locked -p identus-crypto --all-targets --features kmp-compat";
          doCheck = false;
        };

        rust-msrv-entropy-minimal = msrvCraneLib.cargoBuild {
          cargoArtifacts = msrvCargoArtifacts;
          src = rustSrc;
          cargoExtraArgs = "--locked -p identus-adapters-entropy --all-targets --no-default-features";
          doCheck = false;
        };

        rust-msrv-entropy-deterministic = msrvCraneLib.cargoBuild {
          cargoArtifacts = msrvCargoArtifacts;
          src = rustSrc;
          cargoExtraArgs = "--locked -p identus-adapters-entropy --all-targets --no-default-features --features deterministic";
          doCheck = false;
        };

        rust-msrv-entropy-getrandom = msrvCraneLib.cargoBuild {
          cargoArtifacts = msrvCargoArtifacts;
          src = rustSrc;
          cargoExtraArgs = "--locked -p identus-adapters-entropy --all-targets --no-default-features --features getrandom";
          doCheck = false;
        };

        rust-msrv-entropy-all = msrvCraneLib.cargoBuild {
          cargoArtifacts = msrvCargoArtifacts;
          src = rustSrc;
          cargoExtraArgs = "--locked -p identus-adapters-entropy --all-targets --all-features";
          doCheck = false;
        };
      };
    };
}
