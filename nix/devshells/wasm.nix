{
  perSystem =
    {
      pkgs,
      toolchain,
      ...
    }:
    {
      devshells.wasm = {
        devshell.name = "sdk-rust-wasm";

        packages =
          with pkgs;
          [
            toolchain
            wasm-pack
            wasm-bindgen-cli
            cargo-deny
            cargo-audit
            chromedriver
            geckodriver
            git
            cacert
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
        ];
      };
    };
}
