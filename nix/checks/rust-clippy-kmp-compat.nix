{
  perSystem =
    {
      craneLib,
      cargoArtifacts,
      rustSrc,
      ...
    }:
    {
      # `cargo clippy` over the opt-in `kmp-compat` surface of
      # `identus-crypto`, which the default-feature `rust-clippy` check never
      # compiles. See `rust-test-kmp-compat.nix` for why this is distinct
      # from `--all-features`.
      checks.rust-clippy-kmp-compat = craneLib.cargoClippy {
        inherit cargoArtifacts;
        src = rustSrc;
        cargoClippyExtraArgs = "-p identus-crypto --features kmp-compat -- -D warnings";
      };
    };
}
