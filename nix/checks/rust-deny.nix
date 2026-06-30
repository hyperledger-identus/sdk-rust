{
  perSystem =
    { craneLib, ... }:
    {
      checks.rust-deny = craneLib.cargoDeny {
        src = ./../..;
      };
    };
}
