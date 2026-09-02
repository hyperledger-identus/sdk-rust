{
  perSystem =
    {
      craneLib,
      cargoArtifacts,
      rustSrc,
      ...
    }:
    {
      checks = {
        rust-clippy-crypto-minimal = craneLib.cargoClippy {
          inherit cargoArtifacts;
          src = rustSrc;
          cargoClippyExtraArgs = "-p identus-crypto --lib --no-default-features -- -D warnings";
        };

        rust-clippy-entropy-minimal = craneLib.cargoClippy {
          inherit cargoArtifacts;
          src = rustSrc;
          cargoClippyExtraArgs = "-p identus-adapters-entropy --no-default-features --all-targets -- -D warnings";
        };

        rust-test-entropy-deterministic = craneLib.cargoNextest {
          inherit cargoArtifacts;
          src = rustSrc;
          cargoNextestExtraArgs = "-p identus-adapters-entropy --no-default-features --features deterministic --no-fail-fast --no-tests=pass";
        };

        rust-test-entropy-getrandom = craneLib.cargoNextest {
          inherit cargoArtifacts;
          src = rustSrc;
          cargoNextestExtraArgs = "-p identus-adapters-entropy --no-default-features --features getrandom --no-fail-fast --no-tests=pass";
        };

        rust-test-entropy-all = craneLib.cargoNextest {
          inherit cargoArtifacts;
          src = rustSrc;
          cargoNextestExtraArgs = "-p identus-adapters-entropy --all-features --no-fail-fast --no-tests=pass";
        };
      };
    };
}
