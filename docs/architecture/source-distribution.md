# Source and prerelease distribution

Rust consumers may evaluate all implemented packages from the public source
repository. The protected `0.1.0-rc.1` train additionally activates
`identus-derive`, `identus-core`, and `identus-crypto` for crates.io; an
immutable registry receipt proves whether that external publication completed.

| Cargo package | Path | Default features | Current status |
| --- | --- | --- | --- |
| `identus-core` | `crates/core` | none | experimental `0.1.0-rc.1` train |
| `identus-derive` | `crates/derive` | none | experimental `0.1.0-rc.1` train |
| `identus-crypto` | `crates/crypto` | crypto suite | experimental `0.1.0-rc.1` train |
| `identus-did` | `crates/did` | none | experimental source |
| `identus-did-resolver-http` | `crates/did-resolver-http` | none | experimental source |

## Cargo contract

Use the canonical public HTTPS repository and replace `SDK_COMMIT` with the
reviewed, full lowercase 40-hex commit that the consumer intends to validate:

```toml
identus-crypto = { git = "https://github.com/hyperledger-identus/sdk-rust", rev = "SDK_COMMIT", default-features = false, features = [ "ed25519", "hash" ] }
```

`SDK_COMMIT` is an instructional placeholder, not a resolvable artifact. For a
concrete, historically validated canary, NeoPRISM used:

```toml
identus-crypto = { git = "https://github.com/hyperledger-identus/sdk-rust", rev = "04b45b7fceb094ae601e3cf7a3291de0a0248b57", default-features = false, features = [ "ed25519", "hash" ] }
```

Apply the same `git` and exact `rev` fields to any package in the table. Do not
use a branch, tag, pull-request ref, default branch, or short revision as the
artifact identity. Do not add a misleading `version = "0.0.0"` selector.
Choose features explicitly for `identus-crypto`; a downstream compatibility
facade may choose a broader set.

Commit the resulting `Cargo.lock`. Cargo records the resolved Git commit in the
lockfile, so review both the manifest and lockfile whenever the SDK moves. A
library intended for crates.io publication cannot retain this Git dependency;
it must wait for and adopt registry-published SDK packages.

## Nix consumers

Nix does not change the Cargo identity. Keep the manifest's exact SDK commit and
the committed Cargo lock, then retain the consumer's normal `flake.lock`,
`narHash`, or fixed-output hash evidence. The SDK does not currently expose a
supported Nix package or overlay; consumers build their own Cargo dependency
graph.

## Update receipt

An SDK source update records:

- the old and new full SDK commits;
- selected package names and features;
- consumer Rust compiler and tested targets;
- Cargo lock and any Nix lock/hash delta;
- build, unit, conformance, and relevant integration results.

The selected commit should be reachable from protected `develop` and have green
SDK CI evidence. Public readability is necessary but does not replace review of
the commit, dependency delta, license evidence, or the consumer's own threat
model.

## Limitations

This remains a pre-release channel. The three release-train crates make an
experimental prerelease SemVer commitment; they make no stable SemVer compatibility,
support-lifetime, binary, Nix package, FFI package,
certification, or production-release promise. Every other workspace package
retains version `0.0.0` and `publish = false`. An exact Git commit is still the
only supported evaluation identity for those source-only packages. The current
consumer compiler floor is Rust 1.89.0.

NeoPRISM's `identus-apollo` is a downstream compatibility facade over
`identus-crypto`; it is not another name for an SDK package. Registry release
and foreign-language distribution require separate decisions.

Primary reference material:

- [Cargo Git dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories)
- [Cargo manifest and lockfile roles](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html)
- [Nix flake lock attributes](https://releases.nixos.org/nix/nix-2.34.1/manual/command-ref/new-cli/nix3-flake.html)
