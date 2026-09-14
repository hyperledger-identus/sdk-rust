# Verification evidence

- **Issue:** #269
- **Develop base:** `dbef9923e65d1a8332c4ba38f42532c8a12811f7`
- **Planning commit:** `d96f4781475441059726d9da1d78a5ecdaa40b54`
- **Implementation commit:** `b60ec9ef5d9fa06b22614eb216a1655f519002e0`
- **Environment:** aarch64-darwin, repository-pinned Rust 1.98.1 and Nix

## Focused crypto evidence

- `cargo test -p identus-crypto`: passed.
- `cargo test -p identus-crypto --all-features`: passed.
- `cargo test -p identus-crypto --no-default-features`: passed.
- `cargo test -p identus-crypto --no-default-features --features derivation`:
  passed.
- `cargo test -p identus-crypto --no-default-features --features kmp-compat`:
  passed.
- `cargo clippy -p identus-crypto --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc -p identus-crypto --no-deps
  --all-features`: passed.
- `cargo fmt --all -- --check` and `git diff --check`: passed.

The compile-fail suite rejects public access to the private-key and chain-code
fields of both HD types, and rejects `Clone` and `Display` on exposed secret
values. Runtime coverage verifies redacted formatting, explicit erasure and
both HD owners' erasure. Existing BIP-32, SLIP-0010 and Apollo-overlap vectors
remain byte-identical.

## Reproducible target and repository matrix

`nix flake check --print-build-logs` passed all 22 compatible aarch64-darwin
checks. This included native and all-feature builds, minimal and KMP crypto,
Rust policy lanes, WASM, iOS and Android builds, strict Clippy, rustdoc,
formatting, factory/policy, TOML/text/Nix lint, dependency/license/advisory
checks and release Nextest profiles. The workspace suite ran 703 tests: 703
passed and 22 configured diagnostics were skipped. KMP crypto ran 133/133
tests with one configured diagnostic skipped.

`./bootstrap.sh --check` passed the 20-package inventory, all factory and
OpenSpec structure checks, dependency/parity/candidate contracts and 21 Node
policy tests. Nix reported x86_64-linux as locally incompatible; the protected
PR's hosted Linux `fast` check remains mandatory before merge. Existing
nonfatal offline yanked-index and macOS Nix fixup-scanner diagnostics did not
fail a derivation.

## Unpublished candidate receipt

The exact clean implementation head produced a passing candidate receipt in
80.393 seconds with profiles `default`, `all-features`,
`no-default-features` and `kmp-compat`. Package evidence:

- `identus-derive-0.1.0-rc.1.crate`: 18,526 bytes, SHA-256
  `f151dfb2526e6375d2eb0ee7b218d0bdc68b2a589ca385142c966e24787679b6`.
- `identus-core-0.1.0-rc.1.crate`: 12,908 bytes, SHA-256
  `b4ec2fd0c3fa5fbfb92f228177083de015b1efbae6ebb995a485fdc8310e816c`.
- `identus-crypto-0.1.0-rc.1.crate`: 79,183 bytes, SHA-256
  `f933af1ae625d10fca4cbd3b6a90340f05c5ea651e6be1f9f3547068785a9d4d`.

Package, API, SemVer and CycloneDX SBOM stages passed with
`cargo-public-api 0.52.0`, `cargo-semver-checks 0.50.0` and
`cargo-cyclonedx 0.5.9`. The candidate remains unpublished and no release
artifact was promoted.

## Review and exclusions

The distinct exact-diff architecture, security and public-API review found no
unresolved defect. No relevant local command was left unrun. Hosted CI remains
mandatory. Publication, release, consumer migration, custody, hardware-backed
protection, FFI projection and work for #7 or #168 remain excluded.
