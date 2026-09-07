{ inputs, ... }:
{
  perSystem =
    {
      pkgs,
      craneLib,
      etalonCraneLib,
      msrvCraneLib,
      cargoArtifacts,
      etalonCargoArtifacts,
      msrvCargoArtifacts,
      rustSrc,
      ...
    }:
    let
      inherit (pkgs.lib)
        concatMap
        concatStringsSep
        escapeShellArgs
        getAttr
        listToAttrs
        map
        optionalAttrs
        optionals
        ;
      manifest = builtins.fromTOML (builtins.readFile ./gates.toml);
      cargoArgumentAttribute = {
        cargoBuild = "cargoExtraArgs";
        cargoClippy = "cargoClippyExtraArgs";
        cargoDoc = "cargoDocExtraArgs";
        cargoNextest = "cargoNextestExtraArgs";
      };
      cargoArgs =
        gate:
        escapeShellArgs (
          optionals gate.locked [ "--locked" ]
          ++ optionals gate.workspace [ "--workspace" ]
          ++ concatMap (package: [
            "--package"
            package
          ]) gate.packages
          ++ concatMap (package: [
            "--exclude"
            package
          ]) gate.exclude_packages
          ++ optionals gate.lib [ "--lib" ]
          ++ optionals gate.all_targets [ "--all-targets" ]
          ++ optionals gate.no_default_features [ "--no-default-features" ]
          ++ optionals gate.all_features [ "--all-features" ]
          ++ optionals (gate.features != [ ]) [
            "--features"
            (concatStringsSep "," gate.features)
          ]
          ++ optionals (gate.target != "") [
            "--target"
            gate.target
          ]
          ++ gate.extra_args
        );
      makeGate =
        gate:
        let
          selectedCrane =
            if gate.toolchain == "primary" then
              craneLib
            else if gate.toolchain == "etalon" then
              etalonCraneLib
            else
              msrvCraneLib;
          operation = getAttr gate.operation selectedCrane;
          argumentAttribute = cargoArgumentAttribute.${gate.operation} or null;
          selectedArtifacts =
            if gate.artifacts == "primary" then
              cargoArtifacts
            else if gate.artifacts == "etalon" then
              etalonCargoArtifacts
            else
              msrvCargoArtifacts;
        in
        operation (
          {
            src = if gate.source == "repository" then ./../.. else rustSrc;
          }
          // optionalAttrs (gate.artifacts != "none") {
            cargoArtifacts = selectedArtifacts;
          }
          // optionalAttrs (argumentAttribute != null) {
            ${argumentAttribute} = cargoArgs gate;
          }
          // optionalAttrs (gate.operation == "cargoBuild") {
            doCheck = false;
          }
          // optionalAttrs (gate.operation == "cargoAudit") {
            inherit (inputs) advisory-db;
          }
        );
      generatedChecks = listToAttrs (
        map (gate: {
          inherit (gate) name;
          value = makeGate gate;
        }) manifest.gates
      );
    in
    {
      checks = generatedChecks;
    };
}
