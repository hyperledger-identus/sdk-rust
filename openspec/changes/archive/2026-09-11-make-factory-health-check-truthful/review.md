# Local review

- **Review date:** 2026-09-11
- **Review angle:** shell routing, recursion bounds, argument/exit preservation,
  failure composition and operator truthfulness
- **Scope:** exact implementation diff after planning commit `88cd34b`
- **Result:** passed after the fixture finding below was resolved

## Resolved finding

1. The first outside-shell routing assertion compared the macOS `/var` spelling
   of a temporary path with Git's canonical `/private/var` spelling. The
   implementation was routing correctly, but the test rejected it. The fixture
   now derives its expected repository root through Git, matching the production
   facade without weakening argument-boundary assertions.
2. Hosted Linux ran the fixture in an isolated Nix builder without
   `/usr/bin/env`; its generated fake executables therefore failed before the
   assertions. They now use the already-pinned `$BASH` interpreter path, matching
   the repository's established hermetic fixture pattern.

## Verification evidence

- `scripts/tests/factory-contract.sh` passed, including outside/inside routing,
  spaced argv preservation, exact failure status and bootstrap fail-fast cases.
- Direct host `scripts/factory audit --json` entered the pinned environment and
  passed with Pi 0.84.2, Node 24.19.0 and npm 11.17.0 in 4.50 seconds.
- `./bootstrap.sh --check` passed structural/OpenSpec, effective runtime and 14
  operational tests in 2.71 seconds.
- `./bootstrap.sh --pi --version` passed, prepared the shared package cache and
  returned Pi 0.84.2 without tracked cache state.
- Full `nix flake check --print-build-logs` passed all 29 aarch64-darwin
  derivations, including 701 workspace tests plus focused feature/target,
  build, Clippy, documentation, policy and factory checks, in 782.72 seconds.

## Security and privacy review

- Nix and script paths are passed as argv; no shell evaluation or user-derived
  command construction was added.
- Re-entry is bounded by `IN_NIX_SHELL`; an already-entered but incorrect shell
  reaches the unchanged audit implementation and fails on its own invariants.
- Exit codes cross the Nix boundary unchanged, and bootstrap stops before its
  operational tests when the runtime audit fails.
- No authentication, provider, model, prompt, transcript, credential or
  downstream repository state is read or modified.

## Remaining limitations

- A full direct audit pays one Nix evaluation outside the devshell; the measured
  local run was 4.50 seconds and does not justify a second host-side auditor.
- The flake still emits the pre-existing `stdenv.isDarwin` deprecation warning;
  Darwin fixup also emitted intermittent helper-process segmentation warnings
  while the affected derivations still completed and the overall 29-check
  matrix passed. This routine change does not alter Nix inputs or semantics.
- Metrics remain sparse and Pi progress observability has only one slow canary;
  both require separate repeated evidence before further harness complexity.
- Rust, targets, consumer repositories, releases, publication and `main` remain
  unchanged.

No unresolved blocking finding remains.
