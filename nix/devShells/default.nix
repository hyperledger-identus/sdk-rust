{ ... }:
{
  perSystem =
    { pkgs, sdk-rustLib, ... }:
    let
      inherit (sdk-rustLib.rustTools) rust;
    in
    {
      devshells.default = {
        name = "sdk-rust";

        packages = with pkgs; [
          # base
          just
          nix
          nixfmt
          which
          # text linters
          editorconfig-checker
          markdownlint-cli2
          shellcheck
          yamllint
          # rust
          cargo-edit
          cargo-llvm-cov
          cargo-udeps
          clang
          rust
          # fmt
          taplo
        ];

        env = [
          {
            name = "LANG";
            value = "C.utf8";
          }
          {
            name = "CC_wasm32_unknown_unknown";
            value = "${pkgs.clang.cc}/bin/clang";
          }
        ];
      };
    };
}
