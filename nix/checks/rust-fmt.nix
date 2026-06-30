{
  perSystem =
    { craneLib, ... }:
    {
      checks.rust-fmt = craneLib.cargoFmt {
        src = craneLib.cleanCargoSource ./../..;
      };
    };
}
