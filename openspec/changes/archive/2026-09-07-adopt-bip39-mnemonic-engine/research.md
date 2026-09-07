# Research readiness

Research class: cryptography-security
Research status: ready
Decision date: 2026-09-08
Source retrieval date: 2026-09-08
Research blockers: none

## Problem and existing implementation

The current implementation in `crates/crypto/src/derivation/mnemonic.rs`
locally stores the English wordlist,
encodes entropy and performs PBKDF2. It checks only that every input word exists
in that list. Empty input is valid under that predicate, invalid checksums are
accepted, entropy outside 128–256 bits is accepted when its byte length is a
multiple of four, and NFKD normalization is absent. Existing tests cover the 24
English Trezor vectors and Apollo's ASCII KMP salt but not those invalid or
Unicode boundaries.

Named consumers require standards-correct BIP-39 through the existing generic
crypto facade. No consumer requires multilingual detection, public dependency
types, UI recovery, storage, custody, BIP-32 or chain policy in this slice.

## Normative sources

- Bitcoin BIP-39 specification and its linked Trezor vector suite.
- Published crate `bip39 2.2.2`, signed tag
  [`v2.2.2`](https://github.com/rust-bitcoin/rust-bip39/tree/d6dbc31678cc507c8cae62b3a059b0b48e866436)
  at `d6dbc31678cc507c8cae62b3a059b0b48e866436`.
- Crates.io artifact SHA-256
  `90dbd31c98227229239363921e60fcf5e558e43ec69094d46fc4996f08d1d5bc`.
- Apollo baseline
  [`hyperledger-identus/apollo@ccee22b`](https://github.com/hyperledger-identus/apollo/tree/ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c)
  for the opt-in KMP salt variant only.

The previously recorded `1a63bd457cf0643f7eee9b8768ce6ced13d01c18`
revision is seven commits ahead of the 2.2.2 tag. It contains unpublished RNG,
word-count and multilingual-prefix work and is not adopted or treated as the
published artifact source. Issue #152 was corrected before implementation.

## Candidate decisions

| Candidate | Exact version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| `bip39` | 2.2.2 / `d6dbc316` | `conditional-adopt` | Complete English entropy/checksum/NFKD/seed mechanics, `no_std`, zeroizing mnemonic option, maintained and independently reversible. Must remain private. | Advisory, maintenance loss, target regression, vector drift, or secret-lifecycle regression. |
| Current sdk-rust code | baseline `69e83bc` | `remove-after-parity` | Existing vectors pass, but validation and normalization are incomplete and the 2,048-word list duplicates a standard artifact. | Retain only if the candidate fails required compatibility/security gates. |
| `tiny-bip39` | 1.0.0 | `not-adopt` | Older fork with broader legacy ecosystem assumptions and no advantage over the maintained rust-bitcoin implementation. | A maintained release demonstrably offers a smaller, safer exact feature cone. |
| `coins-bip39` | 0.12.0 | `not-adopt` | Couples mnemonic mechanics to the broader coins ecosystem and does not reduce facade work. | A narrow independent crate is split with stronger maintenance/security evidence. |

## Compatibility and dependency evidence

The selected crate is edition 2018, CC0-1.0, `no_std`, and latest/current on
crates.io as retrieved 2026-09-08. It declares no `rust-version`; upstream
documents Rust 1.41.1 generally and 1.51 when `zeroize` is enabled. The fresh
selected-feature resolution uses `zeroize 1.9.0`, whose declared Rust 1.85
floor makes the measured effective cone compatible with the SDK's Rust 1.85
floor. Exact Rust 1.85 integration remains an implementation gate rather than
an inferred guarantee.

With `default-features = false` and `features = ["alloc", "zeroize"]`, a clean
probe resolves 13 third-party packages. Comparing names to the current sdk-rust
lock leaves seven incremental packages: `bip39`, `bitcoin_hashes`,
`hex-conservative`, `arrayvec`, `unicode-normalization`, `tinyvec`, and
`tinyvec_macros`. Host Rust 1.98.1 and `wasm32-unknown-unknown` probe builds
pass. Android, iOS, complete workspace, feature and MSRV results remain
implementation evidence.

The selected crate does not own RNG under these features. The SDK continues to
request exactly 32 bytes from `SecureRandom`, then calls `Mnemonic::from_entropy`.
No rand crate or platform entropy source enters this dependency cone.

Public and wire compatibility remains owned by `MnemonicHelper`. `wordlist`,
validation, entropy conversion and seed methods retain their signatures; no
wire format is introduced. The deliberate behavioral correction is that
unsupported word counts, bad checksums and non-standard entropy no longer
succeed. Dependency errors map to the existing redacted
`crypto.mnemonic_invalid`; dependency types and formats do not escape.

## Security, privacy and maintenance evidence

The direct crate has no lexical unsafe block and no C/C++/assembly/build source.
The seven new transitive packages are pure Rust, but source inspection finds
unsafe implementations in `bitcoin_hashes`, `arrayvec`, and
`unicode-normalization`. These packages are compiled under exact lockfile
checks; this is not represented as an unsafe-free graph.

`bip39::Mnemonic` implements `Debug` and `Display` by emitting words. With the
selected `zeroize` feature its fixed word-index storage implements
`ZeroizeOnDrop`, but its `to_seed` convenience may allocate an NFKD-normalized
passphrase in a non-zeroizing `String`. The SDK will never expose or format the
type and will instead move any owned normalized `Cow` immediately into
`Zeroizing<String>` before calling `to_seed_normalized`. The returned fixed seed
array is wrapped in `Zeroizing` before copying into the existing caller-owned
return vector. The joined mnemonic is also SDK-owned zeroizing text.

License and provenance are compatible with the workspace. The exact 14-package
probe lock (including the probe root) passed the current
RustSec database containing 1,242 advisories with no finding on 2026-09-08.
This is a supply-chain signal, not a cryptographic audit. The repository is not
archived, was updated 2026-08-20, and the 2.2.2 tag commit is GitHub-verified.
The registry artifact `src/` tree matches that tag byte-for-byte; its downloaded
archive hash matches crates.io metadata. Workspace cargo-deny already permits
CC0-1.0, Zlib, MIT and Apache-2.0 licenses present in the cone.

## Rejected or deferred candidates

Keeping local validation is rejected because the identified empty/count/checksum
and normalization defects are concrete. Reimplementing NFKD, word-index lookup
or checksum validation would preserve the same defect class while duplicating
a mature focused crate. Broader wallet/Bitcoin suites provide no useful facade
benefit and increase coupling. Multilingual support is deferred because it
changes product/API policy independently of the English mechanics decision.
The protocol is the final BIP-39 standard rather than a moving draft; library
updates do not authorize semantic drift from its published vectors.

## Open questions and blockers

No research blocker remains. The dependency's word-revealing formatters and
non-zeroizing convenience normalization are explicit integration hazards, not
reasons to reject the crate: the proposed private adapter avoids both. Hosted
Linux and complete workspace evidence remain mandatory before merge.

## Evidence commands

- `cargo info bip39@2.2.2` recorded exact features and metadata.
- GitHub API verified tag `v2.2.2` at `d6dbc316...`, repository state, commit
  signature and the seven-commit difference to the formerly cited revision.
- Crates.io API recorded current/latest state, publish date and artifact hash;
  local SHA-256 and a recursive `src/` diff matched the tag.
- A standalone locked probe with `default-features = false, alloc, zeroize`
  produced dependency/feature trees, license/MSRV metadata and passed host plus
  WASM builds.
- `rg` and native-file scans covered the exact resolved source cone and are
  summarized above.
- Pinned Nix `cargo audit` loaded 1,242 RustSec advisories and found no
  vulnerability in the probe lock.
- Workspace integration, API diff, Rust 1.85, portable targets and complete Nix
  are deliberately unrun at this checkpoint and deferred to the implementation
  verification receipt.

Rollback is one focused issue #152 PR revert. The reconsideration trigger is an
advisory, maintenance loss, target regression, vector drift, public-boundary
leak, or a narrower maintained implementation with equivalent lifecycle and
protocol behavior.
