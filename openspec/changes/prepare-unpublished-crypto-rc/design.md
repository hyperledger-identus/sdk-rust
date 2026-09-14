# Design: isolated unpublished cryptography candidate

## Boundary

Canonical development stays a `0.0.0`, `publish = false` monorepo. A versioned
descriptor selects three source crates and drives a deterministic staging
workspace. Generated directories and artifacts are never committed.

## Candidate pipeline

1. Validate a strict descriptor and a clean exact Git source revision.
2. Copy allow-listed package files, READMEs, and Apache-2.0 license into two
   independently created temporary staging workspaces.
3. Render explicit package metadata and exact internal
   `=0.1.0-rc.1` path-plus-version dependencies; retain canonical external
   versions and features.
4. Run Cargo package listing and archive assembly with `--no-verify` only
   because the dependent candidates are not on crates.io.
5. Compare archive bytes from both stages, inspect normalized manifests and
   bounded contents, and extract the exact archives.
6. Build/test the three-package closure and a minimal crypto consumer against
   the extracted archives with only local `[patch.crates-io]` entries.
7. Generate a CycloneDX SBOM from the staged exact closure, a public-API
   baseline/check, and a deterministic JSON receipt containing source, tool,
   package, checksum, size, feature, and limitation data.

## Tool routing

`cargo-semver-checks` 0.50.0 and `cargo-cyclonedx` 0.5.9 come from the already
locked nixpkgs input. They enter a dedicated Nix application/check, not the
normal development shell or required fast matrix. The candidate script refuses
other tool versions in evidence mode.

## Package surface

Each package receives a crate-local README and candidate metadata: description,
repository, homepage, documentation, keywords, categories, license, and
rust-version. The descriptor records default/all/no-default/`kmp-compat`
profiles for crypto. No facade alias is generated.

## API baseline

The first candidate commits the deterministic `cargo-public-api` rendering for
all features and runs `cargo-semver-checks` against protected base `8110277`.
The source baseline is appropriate only for detecting accidental packaging/API
loss in the first candidate; later candidates must compare to the committed
`0.1.0-rc.1` baseline.

## Failure and cleanup

Every stage fails closed. Output is created under an explicit caller directory
or a new temporary directory, never the repository root. Partial output is
removed unless `--keep-work` is requested for diagnosis. No publisher command,
registry credential, tag, or remote mutation is present.

## Alternatives

Making canonical manifests publishable would weaken an effective safety
constraint. Hand-writing normalized manifests or SBOM/API semantics would
duplicate maintained tools. Adding all release tooling to `fast` would make
ordinary feature feedback pay a release-only cost.
