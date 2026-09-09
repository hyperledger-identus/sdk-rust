{
  perSystem =
    {
      pkgs,
      fuzzToolchain,
      toolchain,
      inputs',
      ...
    }:
    {
      devshells.default = {
        devshell.name = "sdk-rust";

        packages =
          with pkgs;
          [
            # rust toolchain (stable, from oxalica/rust-overlay) incl. wasm target
            toolchain

            # C toolchain + crypto build prerequisites (transitive crypto deps)
            stdenv.cc
            pkg-config
            openssl

            # cargo quality tooling
            cargo-nextest
            cargo-deny
            cargo-audit
            cargo-fuzz
            cargo-llvm-cov

            # protobuf (for codegen of identus protos)
            protobuf
            python3

            # workspace-consistency tooling
            just
            git
            jq
            curl
            which
            gh
            cacert

            # nix hygiene tooling
            nix
            nixfmt
            deadnix
            statix

            # toml and file-hygiene tooling
            taplo
            markdownlint-cli2
            yamllint
            editorconfig-checker
            shellcheck
            actionlint

            # spec-driven development tooling
            inputs'.openspec.packages.default
          ]
          ++ lib.optionals stdenv.isDarwin [
            libiconv
            gradle
            jdk17
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
        ]
        ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
          {
            name = "LIBRARY_PATH";
            value = "${pkgs.libiconv}/lib";
          }
          {
            name = "IDENTUS_JAVA_HOME";
            value = "${pkgs.jdk17}";
          }
        ];
      };

      # libFuzzer's sanitizer instrumentation uses nightly-only compiler
      # options. Keep that operational exception explicit and pinned to the
      # tooling exception instead of weakening the stable default shell.
      devshells.fuzz = {
        devshell.name = "sdk-rust-fuzz";

        packages = with pkgs; [
          fuzzToolchain
          stdenv.cc
          pkg-config
          openssl
          cargo-fuzz
          cacert
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
        ]
        ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
          {
            name = "LIBRARY_PATH";
            value = "${pkgs.libiconv}/lib";
          }
        ];
      };
    };
}
