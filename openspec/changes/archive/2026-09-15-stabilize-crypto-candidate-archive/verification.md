# Verification

- **Date:** 2026-09-15
- **Planning head:** `ddc3df0688ad3c13b81639aa6ee81abf436c57ee`
- **Implementation head:** `437712f5024296fbb9e230226906c3d39ed6907d`
- **Final reviewed head:** `da1edd304e76a331932c1f9883673d4a6b346393`
- **Protected review:** [PR #289](https://github.com/hyperledger-identus/sdk-rust/pull/289)
- **Implementation scope:** candidate preparation and its offline policy tests

## Passed locally

- `python3 scripts/tests/crypto-candidate.py`: behavioral and six mutation
  cases passed both with the normal temporary root and with `TMPDIR` beneath
  the checkout, proving the positive fixture honors the ambient VCS boundary.
- `python3 scripts/check-crypto-candidate.py`: unpublished three-package
  contract passed.
- Clean package-only candidate: passed at exact final implementation head in
  43.735s;
  receipt recorded `sourceDirty: false` and Cargo/Rust 1.98.1.
- Full `nix run .#crypto-candidate`: passed at the same clean head in 63.337s;
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
- `nix flake check`: all 31 checks passed again after the exact-head review fix
  on `aarch64-darwin`; Nix truthfully reported that the incompatible
  `x86_64-linux` system was omitted.
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
