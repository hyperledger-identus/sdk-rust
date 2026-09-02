{
  perSystem =
    {
      craneLib,
      cargoArtifacts,
      rustSrc,
      ...
    }:
    {
      # Compile-only browser-WASM evidence for the implemented portable crate
      # set. This does not claim browser runtime, bundler, storage or FFI
      # support; those require downstream runtime evidence.
      checks.rust-build-wasm32 = craneLib.cargoBuild {
        inherit cargoArtifacts;
        src = rustSrc;
        cargoExtraArgs = "--locked -p identus-core -p identus-crypto -p identus-did -p identus-adapters-entropy --features identus-adapters-entropy/getrandom --target wasm32-unknown-unknown";
        doCheck = false;
      };
    };
}
