{
  perSystem =
    {
      craneLib,
      cargoArtifacts,
      rustSrc,
      ...
    }:
    {
      # Browser-WASM build-smoke for the `getrandom`-backed entropy adapter:
      # confirms the `getrandom` feature (with the `js` backend) compiles for
      # `wasm32-unknown-unknown`, where entropy resolves to
      # `crypto.getRandomValues()`. Mirrors `sdk-ts`'s
      # `wasm-pack --target=web` expectation as a plain
      # `cargo build --target wasm32-unknown-unknown` smoke (WASI is out of
      # scope — the workspace target set is browser WASM only).
      checks.rust-build-wasm32 = craneLib.cargoBuild {
        inherit cargoArtifacts;
        src = rustSrc;
        cargoBuildCommand = "cargo build -p identus-adapters-entropy --features getrandom --target wasm32-unknown-unknown";
        doCheck = false;
      };
    };
}
