# Verification receipt

Verification date: 2026-10-29
Issue: #582
Develop base: `88efe31e3d413e6f51b0cb80f65155281f8feb3c`
Planning commit: `2f5f2a2b93ce87cd3839a3a172b1974258d72988`
Preimplementation commit: `a0ec4c0254f06a18bd914982e919547a9232964d`
Implementation commit: `8fb2f21ae178ec77bdd1afc9e128e8e7779edeed`
Reviewed implementation: `3a9cdac7156190acca9ace227805acfff3364eb7`

## API and candidate evidence

- Pinned cargo-public-api 0.52.0 produced an additive-only delta of exactly 13
  exports and zero removals.
- The `identus-did` API origin changed from
  `3ce68d571d087e78ece200154ba0e185565fbf5da9c2733eb7f70d3b1f7bbf9d`
  to `5c985cd2d3f26c07230ace1086d7a178662a4f583d0086574ac9baa6a00be639`.
- The resolver API remained
  `9c432f0d0c3d5db92da3aa88aed05164aa2adf483c11be84b9239c9973179df6`.
- Clean non-initializing candidate preparation passed all 12 staged package
  verification commands, closure checks, API/SBOM generation, source policy,
  and byte-identical two-pass archive checks in 87.328 seconds.
- Archive SHA-256 values are
  `7736e445ffce327fae19a55c7b8b7adf38887f93f56aba4a9a804d18d765fb68`
  for `identus-did` and
  `90366988dd00a4730d398431531d0066eb109ff48a1fcc8f0aed4d9673678480`
  for `identus-did-resolver-http`.
- Normalized CycloneDX hashes are
  `d0e0976fa6d96918d59202cd173c4162459f19b253120c41fce3058e5c59d109`
  and `fbaa3985eacf2c89649799c1a33effeb2b521aa21c5ec92ff70444a6afb126d6`.

## Compiler, repository, and documentation evidence

- The macOS primary lane passed Rust 1.98.1 host tests and iOS compile checks
  in 60.862 seconds.
- The macOS MSRV lane passed Rust 1.89.0 host and portable checks in 45.619
  seconds.
- `python3 scripts/check-release-candidates.py .`: passed.
- `python3 scripts/tests/release-candidates.py`: mutation suite passed.
- `scripts/factory check`: passed.
- `git diff --check`: passed.
- The complete `nix flake check` passed 38 exact-head derivations, including
  factory mutations, docs site, formatting, text/TOML lint, dependency and
  RustSec policy, primary/MSRV builds, iOS/Android/WASM targets, clippy,
  rustdoc, and 1,195 workspace nextest tests with 23 declared skips.
- All repository commits carry verified signatures and DCO sign-off.

## Review and release boundary

Codex CLI 0.163.0 with `gpt-6-sol` at high reasoning reviewed the exact diff
through `3a9cdac7156190acca9ace227805acfff3364eb7` and found no actionable
defect. This receipt qualifies the prepublication origin update only. Issue
#388 retains ownership of the final protected merge SHA, native slow evidence,
independent release approval, tag, and publication decision.
