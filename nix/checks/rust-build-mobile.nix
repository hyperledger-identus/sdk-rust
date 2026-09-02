{
  perSystem =
    {
      craneLib,
      cargoArtifacts,
      rustSrc,
      ...
    }:
    {
      # Cross-target compilation only. Linking, device/simulator execution,
      # bindings, platform keystores and packaging remain downstream evidence.
      checks = {
        rust-build-android-aarch64 = craneLib.cargoBuild {
          inherit cargoArtifacts;
          src = rustSrc;
          cargoExtraArgs = "--locked -p identus-core -p identus-crypto -p identus-did -p identus-adapters-entropy --features identus-adapters-entropy/getrandom --target aarch64-linux-android";
          doCheck = false;
        };

        rust-build-ios-aarch64 = craneLib.cargoBuild {
          inherit cargoArtifacts;
          src = rustSrc;
          cargoExtraArgs = "--locked -p identus-core -p identus-crypto -p identus-did -p identus-adapters-entropy --features identus-adapters-entropy/getrandom --target aarch64-apple-ios";
          doCheck = false;
        };
      };
    };
}
