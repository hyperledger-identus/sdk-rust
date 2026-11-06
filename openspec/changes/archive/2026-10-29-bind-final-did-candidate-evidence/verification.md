# Verification receipt

Verification date: 2026-10-29
Issue: #388
Develop base: `34420235103fe9dc599aa061047525c459d422cf`
Planning commit: `a02dce1b75283c0a6b01b7e43129b5f26e86042e`
Reviewed implementation: `66eaf5974f896ac5c08c0e784b365b1faebcc25b`

## Candidate and E2E evidence

- Clean `nix run .#did-candidate` preparation completed 12 staged commands in
  70.894 seconds and produced byte-identical two-pass archives.
- The staged lock SHA-256 is
  `f7535a6819c9b9e4474c95b427b54646d46ccac51703c680086ef75c790fba4f`.
- Public API SHA-256 values are
  `5c985cd2d3f26c07230ace1086d7a178662a4f583d0086574ac9baa6a00be639`
  and `9c432f0d0c3d5db92da3aa88aed05164aa2adf483c11be84b9239c9973179df6`.
- Candidate archive SHA-256 values are
  `7736e445ffce327fae19a55c7b8b7adf38887f93f56aba4a9a804d18d765fb68`
  and `90366988dd00a4730d398431531d0066eb109ff48a1fcc8f0aed4d9673678480`.
- Normalized CycloneDX SHA-256 values are
  `d0e0976fa6d96918d59202cd173c4162459f19b253120c41fce3058e5c59d109`
  and `fbaa3985eacf2c89649799c1a33effeb2b521aa21c5ec92ff70444a6afb126d6`.
- NeoPRISM #324/#325/#327 remain immutable downstream evidence. The latest
  linked Preprod composition receipt records zero missing, extra, duplicate,
  or changed operations and diagnostic duration, CPU, RAM, cursor, and process
  measurements.

## Compiler, repository, and documentation evidence

- The macOS primary lane passed Rust 1.98.1 tests in 75.112 seconds.
- The macOS MSRV lane passed Rust 1.89.0 checks in 46.993 seconds.
- `python3 scripts/check-did-final-readiness.py .`: passed.
- `python3 scripts/tests/did-final-readiness.py`: 15 mutation tests passed.
- `scripts/factory check`: 124 OpenSpec/factory items passed.
- `git diff --check`: passed.
- `nix flake check` passed all 38 exact-head aarch64-darwin derivations in
  858.08 seconds, including factory mutations, documentation, text/TOML lint,
  dependency and RustSec policy, primary/MSRV builds, iOS/Android/WASM,
  clippy, rustdoc, and workspace nextest variants.
- Every repository-facing commit carries a verified signature and DCO
  sign-off.

## Pending protected evidence

This receipt qualifies the pending knowledge packet only. After guarded merge,
the protected merge SHA still requires a complete hosted slow receipt and an
independent human approval. Until both exist, #388 remains open and the packet
must not claim release readiness, tagging, publication, or production status.
