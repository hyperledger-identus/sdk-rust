## Context

`cargo-public-api 0.52.0` builds rustdoc JSON itself unless it receives an
existing JSON file. When its active Cargo reports stable, the tool selects the
literal rustup toolchain name `nightly`. That behavior is appropriate for its
normal rustup installation model but crosses the SDK's Nix-owned toolchain
boundary. Merely setting `RUSTC_BOOTSTRAP=1` does not affect that selection.

## Decisions

### Separate JSON production from API rendering

The candidate runner invokes stable Cargo 1.98.1 directly with `cargo rustdoc`,
the package manifest, all features, a dedicated target directory, and rustdoc's
JSON output flags. Only this command receives `RUSTC_BOOTSTRAP=1`. The runner
then verifies the expected `identus_crypto.json` regular file exists before
passing it to `cargo-public-api --rustdoc-json` for deterministic simplified
rendering.

This uses each tool for one cohesive role: Nix-pinned Cargo controls compiler
identity and JSON generation; the locked parser controls public-API formatting.
The parser cannot invoke rustup to build JSON on this input path. Version 0.52.0
still probes the active Cargo/rustup metadata during CLI startup and may compute
an unused toolchain value; this bounded probe neither installs nor executes a
compiler and is not candidate compiler evidence.

### Keep the boundary structural and executable

The repository checker requires the explicit `cargo rustdoc` command, the
dedicated target directory, the expected JSON identity, the hidden parser input
flag, and the subprocess-scoped bootstrap environment. Mutation tests remove
each important edge and require rejection. Runtime failure remains authoritative
for a malformed or missing JSON artifact.

### Preserve evidence semantics

The receipt continues to report Cargo/Rust from the primary Nix toolchain and
`cargo-public-api 0.52.0`. No nightly is added to the app or recorded as SDK
compatibility evidence. Disposable scratch cleanup removes rustdoc build
artifacts together with the staged workspace.

## Risks and mitigations

- Rustdoc JSON is unstable: the parser and Rust version remain exact, the API
  baseline detects representation drift, and any toolchain update requires a
  separate reviewed change.
- The `--rustdoc-json` parser option is intentionally low-level: lock the parser
  version and cover the invocation structurally plus end-to-end.
- A target path could be guessed incorrectly: require the exact package JSON
  file before parsing and fail closed with a specific error.
- Bootstrap could leak into package tests: construct a new environment only for
  `cargo rustdoc`; every preceding and following command retains the base env.

## Alternatives

- Add rustup and install `nightly`: rejected because it introduces ambient
  mutable state and a floating network operation.
- Reuse the sanitizer nightly: rejected because it changes the documented
  candidate compiler and expands a nightly exception beyond its owned role.
- Spoof Cargo's version or toolchain environment: rejected because it relies on
  implementation quirks and makes evidence misleading.
- Remove public-API evidence: rejected because the candidate baseline is an
  accepted release-readiness control.

## Rollback

Revert the runner/checker/spec amendment and treat the candidate slow job as
known-red until another deterministic JSON producer is accepted. No external
artifact or release state changes.
