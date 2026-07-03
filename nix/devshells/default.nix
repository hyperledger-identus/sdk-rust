{
  perSystem =
    {
      pkgs,
      toolchain,
      inputs',
      ...
    }:
    {
      devshells.default = {
        devshell.name = "sdk-rust";

        packages = with pkgs; [
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

          # protobuf (for codegen of identus protos)
          protobuf

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

          # spec-driven development tooling
          inputs'.openspec.packages.default
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
