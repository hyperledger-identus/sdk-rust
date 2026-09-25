# Verification

Verification date: 2026-09-25
Reviewed implementation head: `93b1239dc1bc36ba8883824032b6090028e12630`

## Exact candidate evidence

- Two independent clean candidate preparations at the reviewed head passed.
- Both passes reproduced the retained #382 archives byte-for-byte:
  - `identus-did`: `ec9ed5be2f8d716d1395cdea347ec71f19def355617579a7ad26cfc14681f7a7`
    (97,727 bytes).
  - `identus-did-resolver-http`:
    `875801cbcd2bb5b1e378ca4f84903f1f58949da7447cea7e586f172845eb8434`
    (18,999 bytes).
- Both passes reproduced the committed all-feature public API snapshots:
  - `identus-did`: `3412a7dce4b5699afda75ad5c9a0bb4f6f5fbb5cb3256e187a2d107fb26763dd`
    (83,988 bytes).
  - `identus-did-resolver-http`:
    `9c432f0d0c3d5db92da3aa88aed05164aa2adf483c11be84b9239c9973179df6`
    (960 bytes).
- Normalized CycloneDX 1.5 JSON was byte-identical across the two clean
  passes:
  - `identus-did`: `371096ba00a08803548b67afcff8df15c10a228fbcd0bfbcf6297e742e475175`.
  - `identus-did-resolver-http`:
    `9f1734f3e400850c58dddc13ed36ea52d71050be899788ec988926b0ca1b5349`.
- Both receipts recorded clean source, exact Rust/Cargo 1.98.1,
  cargo-public-api 0.52.0, cargo-semver-checks 0.50.0 and cargo-cyclonedx
  0.5.9. Compatibility is explicitly `not-applicable-first-candidate`.

## Local gates

- `python3 scripts/check-release-candidates.py .`: passed.
- `python3 scripts/tests/release-candidates.py`: passed, including command
  allowlist, resource-bound, API/tool drift and CycloneDX identity mutations.
- Existing crypto candidate and protected release-train checkers plus mutation
  suites: passed.
- `cargo fmt --all --check`: passed.
- `cargo test -p identus-did -p identus-did-resolver-http --all-features`:
  passed; only declared manual diagnostics were ignored.
- `taplo fmt --check docs/release/did-candidate.toml Cargo.toml`: passed.
- `scripts/check-factory.sh`: passed.
- `scripts/tests/factory-contract.sh`: passed.
- The immutable preimplementation receipt binds specification commit
  `c9c6aeb7edc3c14972dc196f22b78ddab8c8bd69` to protected
  `develop@0fed1ec7f002fbf9ca5d7b84eaa0e7b62b068183` for issue #384.

## Claim boundary

This evidence qualifies credential-free local candidate artifacts only. It is
not a stable API promise, vulnerability-free claim, signed provenance,
platform matrix, registry-availability proof, publication, tag or release.
Repository Cargo-deny and weekly slow evidence remain separate promotion
controls.
