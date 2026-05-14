_: {
  perSystem =
    { pkgs, rust-overlay, ... }:
    {
      _module.args.sdk-rustLib = {
        rustTools = pkgs.callPackage ./rustTools.nix { inherit rust-overlay; };
      };
    };
}
