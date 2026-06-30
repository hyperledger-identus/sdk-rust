{ inputs, ... }:
{
  perSystem =
    { craneLib, ... }:
    {
      checks.rust-audit = craneLib.cargoAudit {
        src = ./../..;
        inherit (inputs) advisory-db;
      };
    };
}
