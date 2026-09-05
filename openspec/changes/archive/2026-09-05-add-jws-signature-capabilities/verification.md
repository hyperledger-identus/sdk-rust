# Verification evidence

## Focused capability gates

- `cargo test -p identus-jose --all-features`: compact codec 8 passed / 1
  release diagnostic ignored; signature capabilities 12 passed / 1 release
  diagnostic ignored.
- strict all-target/all-feature JOSE Clippy and warning-denied JOSE docs:
  passed.
- fixed-width P-256 regression, DID compatibility, entropy-minimal and JOSE
  no-default checks: passed, including the repository-wide no-default run.
- `cargo tree -p identus-jose -e features`: the JOSE crypto cone contains only
  `ed25519`, `secp256r1`, and their shared `jwk` prerequisite; derivation,
  COSE, X25519 and secp256k1 are absent.
- release diagnostic: 10,000 registry-mediated Ed25519 verifications in
  390.199792 ms, approximately 25,628 operations/second; measurement only.

## Host, architecture and factory gates

- immutable factory receipt: branch
  `codex/idr-004-jws-signature-ports`, head
  `1df2debad40036e2220ce7307f5fcc2d9398e5c4`, develop merge base
  `54f46e9b94ce3fe66f9e2c04c389051549ac39a4`.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `cargo test --workspace --all-features`: passed.
- `cargo test --workspace --no-default-features`: passed.
- `RUSTDOCFLAGS=-Dwarnings cargo doc --workspace --no-deps`: passed.
- dependency-ring conformance includes the accepted JOSE-to-crypto edge and
  passed with the workspace suite.
- bootstrap inventory: 16 packages passed; SSI upstream backlog: 30 rows
  passed; support-policy and strict OpenSpec/factory validation passed.

## Authoritative pinned Nix gate

`/nix/var/nix/profiles/default/bin/nix flake check` passed every compatible
`aarch64-darwin` check on the final reviewed source. Evidence includes:

- Rust 1.85 workspace/MSRV and feature-specific builds;
- pinned-etalon host tests, formatting, strict Clippy and warning-clean docs;
- WASM32, Android ARM64 and iOS ARM64 builds including `identus-jose`;
- deterministic/getrandom/all entropy variants and KMP-compatible crypto;
- Nix/TOML/text lint, factory, license/source/ban and offline advisory gates.

The first local invocation omitted untracked new Rust files from Nix's clean
Git source; staging the scoped files fixed source discovery. The next run
found one Taplo alignment error in `crates/did/Cargo.toml`; it was corrected.
The authoritative rerun ended with `all checks passed`. Nix reported
`x86_64-linux` as an incompatible host output and did not claim to execute it.

## Donor isolation

- Oxid remained on `integration` at
  `bfe3b481568dc738f0732c2b27548fab8721fd95`, with only its pre-existing
  `.claude/` and `.pi/taskflows/` paths untracked.
- Lace ID Portal remained on `main` at
  `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`, with only its pre-existing
  `.pi-subagents/`, `.pi/` and `tmp/` paths untracked.
- No donor source or fixture was copied and neither consumer checkout was
  changed.
