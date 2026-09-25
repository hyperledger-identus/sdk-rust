# Design

## Descriptor and compatibility origin

Extend the closed DID descriptor with the already-governed exact evidence tool
versions and one API-baseline path per package. The checker rejects missing,
extra, reordered or mismatched values. ADR 0154 records that an initial
candidate establishes a public-API comparison origin; SemVer comparison is
`not-applicable` until a genuine predecessor exists.

## Evidence generation

After deterministic archives and extracted closure pass, the assembler:

1. generates explicit all-feature rustdoc JSON for each staged package;
2. renders cargo-public-api output and either initializes or compares each
   committed baseline;
3. generates CycloneDX 1.5 JSON for the staged workspace;
4. validates exactly two unique candidate components, version/license, bounded
   regular files and absence of Git dependency references;
5. copies API/SBOM artifacts into the atomic output and records their SHA-256,
   sizes, exact tool versions and compatibility status.

Initialization is a deliberate dirty-source authoring mode. A normal clean
candidate rerun must reproduce the committed baselines and the #382 archive
hashes.

## Enforcement and tests

The process allowlist adds only Cargo `rustdoc`, `public-api`, and `cyclonedx`.
Mutation tests cover tool/version/path/schema drift, missing API baselines,
unexpected SBOM components, Git sources, oversize artifacts and attempts to
execute SemVer comparison without a predecessor. Existing release/crypto tests
remain green.

## Rollback

All outputs are additive release evidence. Revert descriptor fields, baselines
and evidence generation without changing canonical package or remote state.
