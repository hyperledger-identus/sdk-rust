{
  perSystem =
    {
      pkgs,
      toolchain,
      ...
    }:
    {
      # Local-first shell for the private TypeScript DID canary. Keep it
      # separate from the default Rust shell so target tooling cannot become a
      # prerequisite for core development or validation.
      devshells.typescript = {
        devshell.name = "sdk-rust-typescript";

        packages =
          with pkgs;
          [
            toolchain
            wasm-pack
            wasm-bindgen-cli
            nodejs_24
            jq
            git
            cacert
            chromedriver
            geckodriver
          ]
          ++ lib.optionals stdenv.hostPlatform.isLinux [
            chromium
            firefox
          ];

        env = [
          {
            name = "SSL_CERT_FILE";
            value = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";
          }
          {
            name = "LANG";
            value = "C.utf8";
          }
          {
            name = "OPENSPEC_TELEMETRY";
            value = "0";
          }
        ];
      };
    };
}
