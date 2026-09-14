# Verification evidence

- **Issue:** #269
- **Develop base:** `21cdbde6b9c5650d073a4a61f443640363585b38`
- **Planning commit:** `a4416b080f3cae21c6adaaf7c037a2b8d0603b9b`
- **Implementation commit:** `a045a212bb63ebf75a273f541cc0f476fec151d2`
- **Rebased correction evidence head:**
  `85660b0628a351a77cec83a1c459deac477bb539`
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
values. Exact source and public-API inventory review confirms that the exposure
value has no Serde, `Deref`, `AsRef`, public constructor, or binding annotation
surface. Runtime coverage verifies redacted formatting, explicit erasure and
both HD owners' erasure. Existing BIP-32, SLIP-0010 and Apollo-overlap vectors
remain byte-identical.

The same inventory confirms there is no public struct-literal, `from_parts`,
or raw extended-state import surface. Existing `init_from_seed` plus supported
derivation reconstructs state only when the original seed and path are
available; rehydration from persisted private-key, chain-code and metadata
parts remains unsupported and deferred.

## Reproducible target and repository matrix

`nix flake check --print-build-logs` passed every compatible aarch64-darwin
check at the rebased correction evidence head. This included native and
all-feature builds, minimal and KMP crypto, Rust policy lanes, WASM, iOS and
Android builds, default and all-target/all-feature strict Clippy, rustdoc,
formatting, factory/policy, TOML/text/Nix lint, dependency/license/advisory
checks and release Nextest profiles. The exact suite counts are recorded in
the command output rather than inferred here.

`./bootstrap.sh --check` passed the 20-package inventory, all factory and
OpenSpec structure checks, dependency/parity/candidate contracts and 21 Node
policy tests. Nix reported x86_64-linux as locally incompatible; the protected
PR's hosted Linux `fast` check remains mandatory before merge. Existing
nonfatal offline yanked-index and macOS Nix fixup-scanner diagnostics did not
fail a derivation.

## Unpublished candidate receipt

The exact clean rebased correction evidence head produced a passing candidate
receipt in 58.178 seconds with profiles `default`, `all-features`,
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

The preimplementation receipt retains its original creation timestamp while
its Git identities are rebound to the GPG-signed, content-equivalent planning
commit and current `develop` base created by the requested rebase. The rebased
planning diff still contains only the named OpenSpec change and ADR 0114 and
remains an ancestor of the implementation and evidence commits.

## Review and exclusions

The distinct exact-diff architecture, security and public-API review found no
unresolved defect. No relevant local command was left unrun. Hosted CI remains
mandatory. Publication, release, consumer migration, custody, hardware-backed
protection, FFI projection and work for #7 or #168 remain excluded.
