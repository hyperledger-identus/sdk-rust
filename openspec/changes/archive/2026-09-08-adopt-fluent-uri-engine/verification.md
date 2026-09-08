# Verification receipt

## Identity and provenance

- Delivery issue: `#157`; parent dependency decision: `#151`.
- Exact develop base: `1d627801bda5e220857467b9376fcfc2f72fe908`.
- Reviewed implementation head: `7af4a6c91ef5e769cc4897d2946e005c9a98f713`.
- Candidate: `fluent-uri 0.4.1`; release commit
  `d9a6a20614f34b00476837eb8904fb01ca3e54df`; crates.io checksum
  `bc74ac4d8359ae70623506d512209619e5cf8f347124910440dbc221714b328e`.
- Compiler: Rust `1.98.1` under the pinned Nix environment.

## Local gates

- DID passed 118 tests with 8 ignored diagnostics; OID4VCI passed 167 with one
  ignored diagnostic.
- Workspace nextest passed 607/607 with 22 skipped diagnostics. The complete
  aarch64-Darwin-compatible `nix flake check --print-build-logs` passed,
  including primary/etalon builds, feature variants, WASM, Android, iOS,
  clippy, rustdoc, formatting, supply-chain policy and factory checks.
- Host no-default-feature DID/OID4VCI and direct WASM, Android and iOS compile
  checks passed against the integrated lockfile.
- Factory validation passed 48/48. `cargo deny --locked check`, online `cargo
  audit --deny warnings`, Markdown/TOML lint and `git diff --check` passed.
- The Nix offline audit emitted its existing non-fatal missing-yank-index
  diagnostics and completed; the networked audit loaded 1,242 advisories and
  reported no vulnerability.

## Dependency and compatibility receipt

- The workspace gains exact `fluent-uri 0.4.1`, `borrow-or-share 0.2.4`,
  `ref-cast 1.0.27`, `ref-cast-impl 1.0.27` and `syn 3.0.5` lock entries.
  `proc-macro2`, `quote` and `unicode-ident` are reused.
- Neither DID nor OID4VCI has a normal `uriparse` edge. The existing DID dev
  oracle keeps `uriparse`, `fnv` and `lazy_static` in the workspace lock.
- Source inspection and generated rustdoc show no third-party URI type in a
  public SDK signature. Exact owned representations and static errors are
  unchanged.
- The local generic grammar implementation and IPvFuture workaround were
  deleted, producing a net reduction in production parser code despite the
  focused ADR, test and evidence additions.

## Residuals and review result

- `ref-cast` owns a reviewed unsafe transparent-reference boundary; SDK source
  remains free of new unsafe blocks.
- `syn` 2 and 3 coexist until the exact dependency cone converges upstream.
- Compile gates prove target compatibility, not runtime platform
  certification.
- Hosted Ubuntu fast CI and hosted review remain required on the exact PR head
  before merge.

The distinct exact-diff review found no unresolved blocker. The implementation
is ready for guarded archive and exact-head hosted CI.
