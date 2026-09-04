# Verification receipt

- **Component and issue:** `identus-crypto`, #69; parent #9 / `IDR-004`
- **Base SHA:** `688f29f399b5d9d46faa0e934182a1925c7d9120`
- **Reviewed implementation SHA:** `631a40332b32295a7a9a52b2a8f2b840c97b26bf`
- **Sources:** repository-local implementation at the base SHA; no donor code
  or fixtures; `zeroize` 1.9.0 crates.io checksum
  `e13c156562582aa81c60cb29407084cdb54c4164760106ab78e6c5b0858cf64e`
- **Normative contract:** repository secret-format policy, OpenSpec crypto
  lifecycle requirement, and ADR 0022; memory erasure is explicitly best
  effort
- **Public/wire compatibility:** public signatures, fields, algorithms,
  outputs, errors, features, wire forms, and persistence unchanged; HD `Debug`
  is intentionally redacted
- **Threats and bounds:** covers accidental safe-format disclosure and
  SDK-owned normal-drop residue; excludes caller/compiler copies, allocator,
  swap, dumps, hostile hardware, custody, and secure storage

## Commands passed

- `cargo fmt --all -- --check`
- `cargo test -p identus-crypto --all-features` (90 tests, one manual
  diagnostic ignored)
- `cargo clippy -p identus-crypto --all-targets --all-features -- -D warnings`
- `cargo check -p identus-crypto --no-default-features`
- `cargo check -p identus-crypto --no-default-features --features <derivation|ed25519|x25519|secp256k1|secp256r1>`
- `./scripts/factory check`
- `cargo test --workspace --all-features`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `RUSTDOCFLAGS=-D warnings cargo doc --workspace --no-deps --all-features`
- `/nix/var/nix/profiles/default/bin/nix flake check --print-build-logs`
  (all 31 checks; Rust 1.85, pinned nightly, native, Android, iOS, WASM,
  minimal/KMP, docs, tests, lint, supply chain, and factory)

## Commands not treated as gates

`cargo test -p identus-crypto --no-default-features --features <single>` was
attempted for several individual features. Existing integration-test files
import the default feature set and are not individually gated, so these
commands fail before exercising changed code. The supported single-feature
compile checks and Nix-declared feature lanes passed. Memory-after-drop was not
read because doing so would require unsafe or invalid access. Fuzzing was not
run because this slice changes no untrusted parser or arithmetic boundary and
the existing crypto fuzz targets do not exercise secret-buffer lifetimes.

## Review and isolation

- Pre-implementation semantic/security/API review: no blocker.
- Distinct post-implementation exact-diff security/API review: no blocker.
- Consumer preflight/final: no consumer repository was needed or mutated.
- Consumer changed: no.
- Release/adoption follow-up: Apollo lifecycle, Cardano V2 derivation,
  key-handle/custody ports, publication, release, and downstream adoption stay
  separate.

## Hosted review follow-up

PR #70 produced one valid P2 finding against head `98389f0`: the random-seed
convenience path held its internally consumed mnemonic word vector in ordinary
storage. The path now wraps that vector and its strings in `Zeroizing`. The
focused crypto all-feature suite, strict crypto Clippy, formatting, factory
contract, and diff hygiene passed after remediation; exact-head hosted gates
remain the authoritative merge evidence.
