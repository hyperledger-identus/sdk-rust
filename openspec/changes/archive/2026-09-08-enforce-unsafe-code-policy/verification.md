# Verification receipt

## Identity and provenance

- Delivery issue: `#169`; follow-up limitation research: `#189`.
- Exact develop base: `7f1ab5771bbacfe400d3fa1d15117ea27f1f728b`.
- Reviewed implementation head: `175a5504bbc61210a8dbb8d546ac20a4d9aa7758`.
- Compiler: Rust `1.98.1` under the pinned Nix environment.
- Rust source evidence: annotated tag
  `18ed059b1465ce6195154de3250a668f1dd3b1fa`, peeled commit
  `48a229ceaefd4985c50990b14116b6d856af0985`.

## Focused enforcement evidence

- `cargo test --locked -p identus-conformance` passed 28/28 tests and zero
  doctests. The unsafe-policy module contributes three top-level guards; its
  behavioral guard performs eight isolated compile-fail probes.
- The configuration guard proves exact root `forbid`, exact inheritance for all
  17 current packages, and fail-closed behavior for missing/weaker settings.
- Behavioral probes cover library, binary, integration test, example, bench,
  build script, proc-macro implementation and attempted `allow` override.
- `cargo clippy --locked -p identus-conformance --lib -- -D warnings`,
  `cargo fmt --all -- --check`, `git diff --check`, constraint validation and
  `./scripts/factory check` passed.

## Complete local gates

- Exact commit `175a5504bbc61210a8dbb8d546ac20a4d9aa7758` passed all 28
  aarch64-Darwin-compatible `nix flake check --print-build-logs` checks.
- Workspace nextest passed 613/613 with 22 skipped diagnostics. The Cardano
  compatibility lane passed 113/113; entropy feature lanes passed 1/1, 3/3
  and 4/4; minimal crypto passed 8/8.
- Primary/etalon, minimal-feature and compatibility builds passed, as did WASM,
  Android and iOS compilation, strict workspace Clippy, rustdoc, formatting,
  dependency bans/licenses/sources, audit and factory/OpenSpec validation.
- The factory derivation passed its isolated contracts after ADR 0087 was added
  to the fixture. Markdown, YAML, shell, editorconfig and TOML lint passed.

## Boundaries and residuals

- No dependency or lockfile changes exist, and no public, wire, runtime, MSRV,
  feature or target-support contract changes.
- Unsafe fixture source is string data only; no unsafe block enters compiled SDK
  source. External dependencies remain outside the first-party lint claim.
- Rustc does not uniformly lint unsafe tokens emitted by external procedural
  macros whose spans allow internal unsafe. `SDK-LIM-008` and #189 retain this
  limitation; this delivery does not claim otherwise.
- Local Nix omitted incompatible `x86_64-linux`; hosted Ubuntu fast CI remains
  the integration authority on the exact PR head.

The distinct exact-diff review found no unresolved blocker. The implementation
is ready for guarded archive and hosted CI.
