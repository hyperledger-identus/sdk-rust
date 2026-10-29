{
  imports = [
    ./rust-gates.nix
  ];

  perSystem =
    {
      pkgs,
      craneLib,
      etalonCraneLib,
      msrvCraneLib,
      inputs',
      ...
    }:
    let
      # Like `craneLib.cleanCargoSource ./../..` but also keeps trybuild
      # `.stderr` fixtures, the seven immutable error goldens, the bounded
      # OID4VCI interoperability packet and the shared DID/key/peer/crypto/OID4VP/SD-JWT vector packets,
      # which the default cargo source filter strips. Keep every exception
      # path-scoped so planning evidence and unrelated data files do not enter
      # Rust build sources.
      cleanedSrc = pkgs.lib.cleanSourceWith {
        src = pkgs.lib.cleanSource ./../..;
        filter =
          path: type:
          let
            sourcePath = toString path;
            oid4vciInteropRoot = "${toString ./../..}/crates/oid4vci/tests/fixtures/interop-v1";
            sdJwtVcDifferentialRoot = "${toString ./../..}/crates/sd-jwt-vc/tests/fixtures/differential-v1";
            sdJwtVcLegacyReceiptRoot = "${toString ./../..}/crates/sd-jwt-vc/tests/fixtures/legacy-draft03-v1";
            sdJwtVcVector = "${toString ./../..}/docs/conformance/fixtures/sd-jwt-vc-draft22/v1/vectors.json";
            sdJwtVcLegacyVector = "${toString ./../..}/docs/conformance/fixtures/sd-jwt-vc-draft03-adapter/v1/vectors.json";
            sdJwtFixtureRoot = "${toString ./../..}/docs/conformance/fixtures/sd-jwt-rfc9901/v1";
          in
          craneLib.filterCargoSources path type
          || pkgs.lib.hasSuffix ".stderr" (baseNameOf sourcePath)
          || pkgs.lib.hasSuffix "/crates/credentials/tests/fixtures/credentials-error-contract-v1.csv" sourcePath
          || pkgs.lib.hasSuffix "/crates/presentations/tests/fixtures/presentations-error-contract-v1.csv" sourcePath
          || pkgs.lib.hasSuffix "/crates/jose/tests/fixtures/jose-error-contract-v1.csv" sourcePath
          || pkgs.lib.hasSuffix "/crates/jose/tests/fixtures/jose-error-contract-v2.csv" sourcePath
          || pkgs.lib.hasSuffix "/crates/jose/tests/fixtures/jwe-encryption-error-contract-v1.csv" sourcePath
          || pkgs.lib.hasSuffix "/crates/oid4vci/tests/fixtures/oid4vci-error-contract-v1.csv" sourcePath
          || pkgs.lib.hasSuffix "/crates/oid4vp/tests/fixtures/oid4vp-error-contract-v1.csv" sourcePath
          || pkgs.lib.hasSuffix "/docs/conformance/fixtures/did/v1/vectors.json" sourcePath
          || pkgs.lib.hasSuffix "/docs/conformance/fixtures/did-key/v1/vectors.json" sourcePath
          || pkgs.lib.hasSuffix "/docs/conformance/fixtures/did-web/v1/vectors.json" sourcePath
          || pkgs.lib.hasSuffix "/docs/conformance/fixtures/did-jwk/v1/vectors.json" sourcePath
          || pkgs.lib.hasSuffix "/docs/conformance/fixtures/did-peer/v1/vectors.json" sourcePath
          || pkgs.lib.hasSuffix "/docs/conformance/fixtures/did-prism/v1/vectors.json" sourcePath
          || pkgs.lib.hasSuffix "/crates/crypto/tests/fixtures/crypto/v1/vectors.json" sourcePath
          || pkgs.lib.hasSuffix "/docs/conformance/fixtures/did-prism-vdr-indexing/v1/vectors.json" sourcePath
          || pkgs.lib.hasSuffix "/docs/conformance/fixtures/oid4vp-direct-post/v1/vectors.json" sourcePath
          || pkgs.lib.hasSuffix "/docs/conformance/fixtures/oid4vp-redirect-response/v1/vectors.json" sourcePath
          || pkgs.lib.hasSuffix "/docs/conformance/fixtures/oid4vp-encrypted-direct-post/v1/vectors.json" sourcePath
          || pkgs.lib.hasSuffix "/docs/conformance/fixtures/oid4vp-transaction-format/v1/vectors.json" sourcePath
          || pkgs.lib.hasSuffix "/docs/conformance/fixtures/siopv2/v1/vectors.json" sourcePath
          || pkgs.lib.hasSuffix "${sdJwtFixtureRoot}/vectors.json" sourcePath
          || pkgs.lib.hasPrefix "${sdJwtFixtureRoot}/oracles/" sourcePath
          || pkgs.lib.hasPrefix sdJwtVcDifferentialRoot sourcePath
          || pkgs.lib.hasPrefix sdJwtVcLegacyReceiptRoot sourcePath
          || sourcePath == sdJwtVcVector
          || sourcePath == sdJwtVcLegacyVector
          || pkgs.lib.hasPrefix oid4vciInteropRoot sourcePath;
      };
      cargoArtifacts = craneLib.buildDepsOnly {
        src = cleanedSrc;
      };
      etalonCargoArtifacts = etalonCraneLib.buildDepsOnly {
        src = cleanedSrc;
        cargoExtraArgs = "--locked --workspace --all-features";
      };
      msrvCargoArtifacts = msrvCraneLib.buildDepsOnly {
        src = cleanedSrc;
        cargoExtraArgs = "--locked --workspace --all-features";
      };
    in
    {
      _module.args = {
        inherit cargoArtifacts etalonCargoArtifacts msrvCargoArtifacts;
        rustSrc = cleanedSrc;
      };

      checks = {
        factory-contract = pkgs.callPackage ./factory-contract.nix {
          openspec = inputs'.openspec.packages.default;
        };
        lint-nix = pkgs.callPackage ./lint-nix.nix { };
        lint-toml = pkgs.callPackage ./lint-toml.nix { };
        lint-text = pkgs.callPackage ./lint-text.nix { };
        rust-source-contract = pkgs.runCommand "rust-source-contract" { src = cleanedSrc; } ''
          test -f "$src/crates/credentials/tests/fixtures/credentials-error-contract-v1.csv"
          test -f "$src/crates/presentations/tests/fixtures/presentations-error-contract-v1.csv"
          test -f "$src/crates/jose/tests/fixtures/jose-error-contract-v1.csv"
          test -f "$src/crates/jose/tests/fixtures/jose-error-contract-v2.csv"
          test -f "$src/crates/jose/tests/fixtures/jwe-encryption-error-contract-v1.csv"
          test -f "$src/crates/oid4vci/tests/fixtures/oid4vci-error-contract-v1.csv"
          test -f "$src/crates/oid4vp/tests/fixtures/oid4vp-error-contract-v1.csv"
          test -f "$src/crates/oid4vci/tests/fixtures/interop-v1/manifest.json"
          test "$(find "$src/crates/oid4vci/tests/fixtures/interop-v1" -type f | wc -l)" -eq 10
          test -f "$src/crates/sd-jwt-vc/tests/fixtures/differential-v1/packet.json"
          test -f "$src/crates/sd-jwt-vc/tests/fixtures/differential-v1/oracle-receipts.json"
          test -f "$src/crates/sd-jwt-vc/tests/fixtures/differential-v1/openwallet-dependency-graph.json"
          test -f "$src/crates/sd-jwt-vc/tests/fixtures/differential-v1/k2gl-dependency-graph.json"
          test "$(find "$src/crates/sd-jwt-vc/tests/fixtures/differential-v1" -type f | wc -l)" -eq 4
          test -f "$src/docs/conformance/fixtures/sd-jwt-vc-draft22/v1/vectors.json"
          test -f "$src/docs/conformance/fixtures/did/v1/vectors.json"
          test "$(find "$src/docs/conformance/fixtures/did/v1" -type f | wc -l)" -eq 1
          test -f "$src/docs/conformance/fixtures/did-key/v1/vectors.json"
          test "$(find "$src/docs/conformance/fixtures/did-key/v1" -type f | wc -l)" -eq 1
          test -f "$src/docs/conformance/fixtures/did-web/v1/vectors.json"
          test "$(find "$src/docs/conformance/fixtures/did-web/v1" -type f | wc -l)" -eq 1
          test -f "$src/docs/conformance/fixtures/did-jwk/v1/vectors.json"
          test "$(find "$src/docs/conformance/fixtures/did-jwk/v1" -type f | wc -l)" -eq 1
          test -f "$src/docs/conformance/fixtures/did-peer/v1/vectors.json"
          test "$(find "$src/docs/conformance/fixtures/did-peer/v1" -type f | wc -l)" -eq 1
          test -f "$src/docs/conformance/fixtures/did-prism/v1/vectors.json"
          test "$(find "$src/docs/conformance/fixtures/did-prism/v1" -type f | wc -l)" -eq 1
          test -f "$src/crates/crypto/tests/fixtures/crypto/v1/vectors.json"
          test "$(find "$src/crates/crypto/tests/fixtures/crypto/v1" -type f | wc -l)" -eq 1
          test -f "$src/docs/conformance/fixtures/did-prism-vdr-indexing/v1/vectors.json"
          test "$(find "$src/docs/conformance/fixtures/did-prism-vdr-indexing/v1" -type f | wc -l)" -eq 1
          test -f "$src/docs/conformance/fixtures/oid4vp-direct-post/v1/vectors.json"
          test "$(find "$src/docs/conformance/fixtures/oid4vp-direct-post/v1" -type f | wc -l)" -eq 1
          test -f "$src/docs/conformance/fixtures/oid4vp-redirect-response/v1/vectors.json"
          test "$(find "$src/docs/conformance/fixtures/oid4vp-redirect-response/v1" -type f | wc -l)" -eq 1
          test -f "$src/docs/conformance/fixtures/oid4vp-encrypted-direct-post/v1/vectors.json"
          test "$(find "$src/docs/conformance/fixtures/oid4vp-encrypted-direct-post/v1" -type f | wc -l)" -eq 1
          test -f "$src/docs/conformance/fixtures/oid4vp-transaction-format/v1/vectors.json"
          test "$(find "$src/docs/conformance/fixtures/oid4vp-transaction-format/v1" -type f | wc -l)" -eq 1
          test -f "$src/docs/conformance/fixtures/siopv2/v1/vectors.json"
          test "$(find "$src/docs/conformance/fixtures/siopv2/v1" -type f | wc -l)" -eq 1
          test -f "$src/docs/conformance/fixtures/sd-jwt-rfc9901/v1/vectors.json"
          test -f "$src/docs/conformance/fixtures/sd-jwt-rfc9901/v1/oracles/affinidi-0.1.8.json"
          test -f "$src/docs/conformance/fixtures/sd-jwt-rfc9901/v1/oracles/iota-0.5.1.json"
          test "$(find "$src/docs/conformance/fixtures/sd-jwt-rfc9901/v1" -type f | wc -l)" -eq 3
          if find "$src/openspec/changes" -type f \
            \( -name credentials-error-contract-v1.csv \
              -o -name presentations-error-contract-v1.csv \
              -o -name jose-error-contract-v1.csv \
              -o -name jose-error-contract-v2.csv \
              -o -name jwe-encryption-error-contract-v1.csv \
              -o -name oid4vci-error-contract-v1.csv \
              -o -name oid4vp-error-contract-v1.csv \) \
            -print -quit | grep -q .; then
            echo "planning error golden entered cleaned Rust sources" >&2
            exit 1
          fi
          touch "$out"
        '';
      };
    };
}
