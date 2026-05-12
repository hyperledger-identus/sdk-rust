{ ... }:
{
  perSystem =
    { pkgs, sdk-rustLib, ... }:
    let
      inherit (sdk-rustLib.rustTools) rust;
    in
    {
      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          # base
          git
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

        shellHook = ''
          export ROOT_DIR=$(${pkgs.git}/bin/git rev-parse --show-toplevel)
          echo "Working on project root directory: $ROOT_DIR"
          cd "$ROOT_DIR"
        '';

        # envs
        LANG = "C.utf8";
        CC_wasm32_unknown_unknown = "${pkgs.clang.cc}/bin/clang";
      };
    };
}
