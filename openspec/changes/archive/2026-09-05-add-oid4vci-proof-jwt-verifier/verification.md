# Verification receipt

## Scope

- Issue: #99, second independently reversible delivery
- Backlog: IDR-004 under #8 and #20
- Base: `17ae03c05bebb7dc2c728932781d2ad6c06e6413`
- Branch: `codex/oid4vci-proof-verifier`
- Specification commit: `ad678e8`
- Implementation commit: `b170cfe63b585347b7047a0225d6a7b28682346f`
- Owner: `identus-jose`

This slice provides issuer-side OpenID4VCI Final proof-JWT parsing, exact
key-reference and signature binding, explicit issuer policy, injected clock
and atomic replay acceptance. Certificate validation remains caller-owned;
optional proof attestation/trust-chain support remains issue #104.

## Focused evidence

- `cargo test -p identus-jose --test oid4vci_proof_verifier`: passed; 15
  behavior/conformance cases passed and one release diagnostic was ignored.
- `cargo test -p identus-jose --no-default-features`: passed.
- `cargo clippy -p identus-jose --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc -p identus-jose --no-deps
  --all-features`: passed.
- The focused dependency-layer test and `scripts/factory check` passed.
- `cargo fmt --all -- --check`, strict OpenSpec validation and
  `git diff --check` passed.

## Workspace and reproducible matrix

- `cargo test --workspace --all-features`: passed.
- `cargo test --workspace --no-default-features`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --all-features`:
  passed.
- `nix flake check`: all 28 compatible `aarch64-darwin` checks passed. This
  includes Rust 1.85 MSRV, native and minimal features, Android AArch64, iOS
  AArch64, browser-WASM, strict Clippy, documentation, formatting, release
  Nextest profiles, architecture/factory, dependency, license, advisory, Nix,
  TOML and text gates. Nix omitted incompatible local `x86_64-linux`; hosted
  Ubuntu CI supplies that independent confirmation.
- The first Nix invocation intentionally exposed that a new untracked Rust
  module is absent from a flake Git-source snapshot. The reviewed prospective
  tree was staged and the exact matrix was rerun to the passing result above;
  this was repository-state correction, not a waived product gate.

## Performance diagnostic

`cargo test -p identus-jose --release --test oid4vci_proof_verifier
issuer_verification_throughput_diagnostic -- --ignored --nocapture` verified
and authorized 20,000 representative inline-JWK proofs in 835.960833 ms,
approximately 23,925 operations/second. This is informational evidence, not a
machine-independent threshold.

## Review and boundaries

The distinct post-implementation review is recorded in `review.md`. Its DID
provider, stable-error and performance-test-double findings were resolved before
the implementation commit and final matrix; no blocker remains.

Oxid remains at `bfe3b481568dc738f0732c2b27548fab8721fd95` with pre-existing
`.claude/` and `.pi/taskflows/` entries. Lace ID Portal remains at
`804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` with pre-existing
`.pi-subagents/`, `.pi/` and `tmp/` entries. midnight-identity remains at
`427f8571950c42967a18726cbcbefecc19ef8d79` with its pre-existing changed
`third_party/midnight-did` submodule status. NeoPRISM remains clean at
`d6ad1ecade80757f08da4f9101d14c2fb1a4d02b`. Apollo remains at
`ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` with its pre-existing changed
`secp256k1-kmp/native/secp256k1` submodule status. No consumer repository, SDK
`main`, chain state, release state or repository setting was modified.
