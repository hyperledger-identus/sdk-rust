_: {
  perSystem =
    { pkgs, sdk-rustLib, ... }:
    let
      app = pkgs.callPackage ./example-web.nix { inherit sdk-rustLib; };
    in
    {
      apps.example-web = {
        type = "app";
        program = "${app}/bin/example-web";
      };
    };
}
