{
  perSystem =
    { craneLib, rustSrc, ... }:
    {
      checks.rust-fmt = craneLib.cargoFmt {
        src = rustSrc;
      };
    };
}
