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

## Outstanding delivery evidence

Factory readiness, proportional Nix checks, clean exact-head candidate
generation, protected PR CI/review and the post-merge complete slow canary have
not yet run. The first natural weekly schedule is also not yet due and remains
issue #276 acceptance evidence rather than this repair's manual canary.
