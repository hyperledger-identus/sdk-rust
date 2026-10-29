# Verification

## Exact failure and repair

Protected slow run 40872151111 at
`develop@6ebd4633c987539689ae180fbccbb629572ca45a` failed while compiling the
extracted `identus-crypto` package because its compatibility test reached above
the archive for a repository fixture. The repaired archive contains both:

- `identus-crypto-0.1.0-rc.1/tests/secp256k1_compat.rs`
- `identus-crypto-0.1.0-rc.1/tests/fixtures/crypto/v1/vectors.json`

The old and new packet SHA-256 values are both
`d357c46c0369e5597c89203d92332b94dbf9684fb28b120bbe3241cdb55e09cb`.

## Passing local evidence

- `python3 scripts/check-crypto-candidate.py`
- `python3 scripts/tests/crypto-candidate.py`
- `python3 scripts/check-cross-language-vectors.py`
- `python3 scripts/tests/cross-language-vectors.py`
- `cargo test -p identus-crypto --all-features --test secp256k1_compat`
  (4 passed)
- `cargo package --list -p identus-crypto --allow-dirty` includes the exact
  crate-local packet path.
- `nix run .#crypto-candidate -- --package-only --allow-dirty ...` assembled
  the candidate twice, proved deterministic archives, inspected required
  members, extracted the three-package closure, and passed every consumer
  profile plus `cargo test --locked -p identus-crypto --all-features` without
  repository files.
- `nix flake check --print-build-logs` passed all 35 compatible
  `aarch64-darwin` checks after the exact TOML and mutation fixtures were
  formatted together. This included the 1,195-test workspace run, MSRV,
  etalon, WASM, Android, iOS, documentation, dependency, source-distribution,
  and factory mutation gates.

## Independently discovered blocker

The complete candidate advanced beyond archive-closure verification and then
failed its public-API comparison. The baseline predates four validation methods
already merged by strict DID-key work. Issue #587 owns that behavior-neutral
baseline reconciliation. This slice does not change or hide the API delta.

Full candidate, review, archive, protected CI, merge, metrics, and a fresh slow
run remain pending and are not inferred from the package-only or Nix passes.
