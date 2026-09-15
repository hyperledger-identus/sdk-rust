# Verification

- **Date:** 2026-09-15
- **Planning head:** `ddc3df0688ad3c13b81639aa6ee81abf436c57ee`
- **Implementation head:** `dd0f594a101c34adf2b748f0f18c764eafdeaf17`
- **Implementation scope:** candidate preparation and its offline policy tests

## Passed locally

- `python3 scripts/tests/crypto-candidate.py`: behavioral and six mutation
  cases passed.
- `python3 scripts/check-crypto-candidate.py`: unpublished three-package
  contract passed.
- Clean package-only candidate: passed at exact implementation head in 40.635s;
  receipt recorded `sourceDirty: false` and Cargo/Rust 1.98.1.
- Full `nix run .#crypto-candidate`: passed at the same clean head in 79.973s;
  archive closure, five compile/test profiles, public API baseline, semver
  check, and three CycloneDX 1.5 documents completed.
- Both real paths produced the same archive digests:
  `identus-derive` `f151dfb2…79b6`, `identus-core` `b4ec2fd0…816c`, and
  `identus-crypto` `e7ed1a3f…db33`.
- `scripts/tests/factory-contract.sh`: complete factory self-test passed.
- `cargo fmt --all -- --check`: passed.
- `cargo test --workspace --all-features`: all unit, integration, compile-fail,
  and doctests passed; explicitly ignored diagnostic benchmarks remained
  ignored.
- `nix flake check`: all 31 checks passed locally on `aarch64-darwin`; Nix
  truthfully reported that the incompatible `x86_64-linux` system was omitted.
- `git diff --check`: passed.

## Hosted evidence before repair

Manual run
[`34951058164`](https://github.com/hyperledger-identus/sdk-rust/actions/runs/34951058164)
at `develop@7bc2030a45e59cfea3f994d13d411a32e79f9b7c` completed with Linux,
browser DID, coverage, performance, and immutable receipt jobs green. The
candidate job failed with the reproduced archive-determinism defect. The macOS
job independently failed later because the hosted Android `sdkmanager` was not
on `PATH`; that separately planned repair must follow this PR.

## Post-merge evidence boundary

The repair must merge through protected `develop` and then pass an exact-head
manual slow canary. That canary and its immutable receipt will be attached to
issue #276. It is recovery evidence only: issue #276 remains open until a later
attempt-one natural scheduled run succeeds.

No check was silently skipped. The hosted Linux and protected PR matrices remain
the authority for systems not locally available.
