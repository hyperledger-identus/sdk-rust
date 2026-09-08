## Why

The pre-release SDK currently blocks each pull request on release-style Linux,
macOS, MSRV, nightly-etalon, target, feature and sanitizer work. The latest 20
successful pull-request runs have a 17m13s p50 and 24m27s p95. Sponsor direction
in discussion #172 and implementation issue #173 selects a temporary
active-development policy: one exact Rust 1.98.1 compiler, one Linux fast merge
signal, and exhaustive weekly/manual evidence.

## What Changes

- Make Rust 1.98.1 the workspace compiler floor, development compiler and
  compatibility etalon for this temporary phase.
- Collapse the primary/MSRV/etalon providers onto the same stable toolchain;
  retain a separately named nightly only for manual/weekly sanitizer tooling.
- Make one Linux `fast` job run factory/repository structure, formatting,
  workspace build, Clippy and the normal workspace test suite on pull requests
  and pushes to `develop`.
- Move the complete Linux/macOS Nix matrix, portable targets, feature
  permutations, docs and supply-chain evidence to a weekly/manual `slow` job.
- Move sanitizer fuzzing off pull requests and pushes to weekly/manual
  experimental workflows.
- Require policy review by 2026-12-08 and prohibit treating this temporary
  matrix as release-candidate evidence.

## Capabilities

### Modified Capabilities

- `sdk-support-policy`: activates one stable compiler and explicit fast/slow CI
  lanes for active development.
- `dependency-research-readiness`: keeps compiler-floor evidence mandatory but
  removes the release-phase requirement for an independently lower MSRV during
  the temporary pre-release phase.
- `nix-tooling`: aligns the canonical workspace compiler-floor scenario with
  the Rust 1.98.1 policy applied to Cargo and Nix.

## Impact

Cargo, Nix providers, workflows, support policy, constraints, contributor and
release guidance, structural validators and their tests change together. No
Rust API, wire format, Cargo dependency, supported target inventory, FFI state,
runtime behavior, release, `main` branch or downstream repository changes.
