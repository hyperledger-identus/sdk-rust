# Verification: OID4VCI Credential Issuer Metadata core

## Receipt

- Issue: #117, child of #7 and #20 / `IDR-023`.
- Base: `develop@78860abcffee5f2a10a5377ba3bce17bc67c164b`.
- Specification: `084da90681b0c97e3e3c8d2e1831eca0cae1e170`.
- Implementation candidate: `fb3f9294b012fd8b1f1e2186d0e3d3af5c90710b`.
- Normative source: OpenID4VCI 1.0 Final sections 4.1.1, 12.2.1, and
  12.2.4; HTML SHA-256
  `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.
- Compatibility: additive API in an unpublished crate; no network, trust,
  flow selection, format profile, consumer adoption, publication, release, or
  `main` work.

## Local commands passed

```text
./scripts/factory check
cargo fmt --all -- --check
cargo clippy -p identus-oid4vci --all-targets --all-features -- -D warnings
cargo test -p identus-oid4vci --all-features
cargo test -p identus-oid4vci --no-default-features
RUSTDOCFLAGS='-D warnings' cargo doc -p identus-oid4vci --no-deps
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --all-features
/nix/var/nix/profiles/default/bin/nix flake check --print-build-logs
```

The local `aarch64-darwin` flake ran all 27 compatible checks. Evidence
includes Rust 1.85 MSRV, browser-WASM, Android ARM64, iOS ARM64, strict
Clippy/docs/format, factory/archive preservation, 461/461 principal nextest
cases, feature matrices, dependency policy, deny, lint, and audit derivations.
The configured audit derivation reported unavailable offline yanked-index
lookups while its advisory scan and the overall derivation passed. Nix omitted
the incompatible `x86_64-linux` system; hosted Linux remains mandatory.

## Focused evidence

- 10/10 issuer metadata integration tests pass with all features and without
  default features; the 9 grant, 10 semantic, and 12 transport tests remain
  green, with one pre-existing diagnostic test ignored.
- Final-shaped and reconstructed Oxid/Lace shapes cover omitted, single, and
  multiple advertised Authorization Servers and opaque extensions.
- Exact issuer/configuration agreement, default effective Authorization Server,
  valid and invalid hints, endpoints, malformed/type-confused/duplicate JSON,
  empty collections, and diagnostic redaction all pass.
- Exact-limit acceptance and one-less rejection pin every metadata ceiling.
- The normal dependency tree remains `identus-core`, `serde_json`, `uriparse`,
  and `zeroize`; manifests and lockfile are unchanged.
- Both commits in the reviewed delta have good local GPG signatures and DCO
  trailers.

## Consumer isolation

- Oxid: `bfe3b481568dc738f0732c2b27548fab8721fd95`; only pre-existing
  untracked `.claude/` and `.pi/taskflows/`. Adapter and portal path digests
  match preflight.
- Lace ID Portal: `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`;
  only pre-existing untracked `.pi-subagents/`, `.pi/`, and `tmp/`. Immutable
  Final-era issuer metadata evidence at `925ec8d04882eabd4ac7b784c70fc2f0c152faae`
  matches preflight digests.
- Consumer changed: no.

## Review and remaining gates

The distinct post-implementation review in `review.md` resolved two local
findings and found no remaining blocker or advisory issue. Hosted Linux/macOS
CI, DCO/policy/hygiene, and exact-head hosted review remain required before
protected merge. Discovery, HTTP, trust, authorization flow selection, format
profiles, consumer adoption, publication, release, and `main` remain follow-ups.
