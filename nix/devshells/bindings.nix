{
  perSystem =
    {
      pkgs,
      stablePkgs,
      ...
    }:
    let
      # Apple packaging is weekly/manual evidence. Keep its Simulator component
      # out of the primary toolchain so the fast Linux line does not pay for it.
      bindingsToolchain = stablePkgs.rust-bin.stable."1.98.1".default.override {
        extensions = [ "llvm-tools-preview" ];
        targets = [
          "aarch64-apple-ios"
          "aarch64-apple-ios-sim"
        ];
      };
    in
    {
      # Native binding generation is intentionally isolated from the default
      # shell because Gradle/JDK are slow-lane tooling, not Rust prerequisites.
      devshells.bindings = {
        devshell.name = "sdk-rust-bindings";

        packages =
          with pkgs;
          [
            bindingsToolchain
            stdenv.cc
            pkg-config
            openssl
            cargo-deny
            cargo-audit
            gradle
            jdk17
            python3
            git
            cacert
          ]
          ++ lib.optionals stdenv.isDarwin [ libiconv ];

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
            name = "IDENTUS_JAVA_HOME";
            value = "${pkgs.jdk17}";
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
