# Verification

## Failure reproduction

Manual slow run `34972066673` at merged `develop`
`59a8db6b7c34439f4f84d762bc1fd56c6ecba9d0` failed only the unpublished
candidate among the completed evidence jobs. Job `104390541635` showed
`cargo-public-api` requesting absent rustup toolchain
`nightly-x86_64-unknown-linux-gnu`. Browser DID, crypto performance, crypto
line coverage and the complete Ubuntu matrix passed; macOS was still executing
when implementation began.

## Focused local evidence

- `scripts/tests/crypto-candidate.py`: passed, including removal mutations for
  explicit `cargo rustdoc`, scoped bootstrap and parser JSON input.
- `scripts/check-crypto-candidate.py .`: passed.
- Full `nix run .#crypto-candidate -- --output <temporary>/output
  --allow-dirty` with `RUSTUP_HOME` pointing to a new empty directory: passed
  in 78.755 seconds.
- The generated receipt reported `rustc 1.98.1`, `cargo 1.98.1`,
  `cargo-public-api 0.52.0`, `cargo-semver-checks 0.50.0`, and
  `cargo-cyclonedx 0.5.9`; all three archives, closure profiles, API baseline,
  SemVer check and CycloneDX outputs completed.
- `git diff --check`: passed before the implementation commit.

The initial shell wrapper attempted to assign zsh's read-only `status` variable
after the candidate had completed, so that wrapper exited nonzero. The candidate
process itself printed success and its complete receipt was inspected. Clean
exact-head validation below is authoritative delivery evidence.

## Clean implementation-head evidence

- Commit `c2a95c3c940c4e6e629f6f2101edb365f3e54261` ran the full candidate with
  exact `--source-revision`, `sourceDirty: false`, and a new empty
  `RUSTUP_HOME`: passed in 65.771 seconds with the same package checksums and
  exact Rust/Cargo/release-tool versions.
- `scripts/factory preflight stabilize-candidate-public-api-toolchain
  --validate-receipt`: passed against planning head
  `af364960fe776bcd7a3568e1e92c9a756fe2ce69`.
- `scripts/factory check --change
  stabilize-candidate-public-api-toolchain`: passed.
- `nix flake check --fallback`: all 31 compatible `aarch64-darwin` checks
  passed, including factory, lint, source policy, build, complete Clippy,
  feature-profile nextest, stable/MSRV/etalon labels, WASM, Android, iOS, docs,
  deny and audit. `x86_64-linux` was explicitly omitted locally and remains
  hosted CI evidence.

## Review

Architecture, security, dependency and evidence-semantics review found no code
blocker. One P2 documentation overclaim said the parser no longer performs
toolchain discovery. Source inspection proved version 0.52.0 still probes
Cargo/rustup metadata and computes an unused nightly value during startup; all
normative language now promises the narrower executable property that a local
JSON input prevents rustup-owned compiler execution. No dependency, public API,
wire contract, target claim, unsafe/native code or secret boundary changes.

## Outstanding hosted evidence

Protected PR CI/review and the post-merge complete slow canary have not yet run.
The first natural weekly schedule is also not yet due and remains issue #276
acceptance evidence rather than this repair's manual canary. The failed canary
also identified a separate macOS Android-license prompt; it will be repaired in
a sequential zero-stack-depth PR before the next full canary.
