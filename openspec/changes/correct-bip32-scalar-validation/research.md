# Research readiness

Research class: cryptography-security
Research status: ready
Decision date: 2026-09-08
Source retrieval date: 2026-09-08
Research blockers: none

## Problem and existing implementation

The current implementation in `crates/crypto/src/derivation/hdkey.rs` computes the correct hardened BIP-32
HMAC message but parses both `IL` and the parent through
`Reduce<U256>::reduce_bytes`. That silently maps `IL >= n` into a different
valid scalar, contrary to BIP-32. The master HMAC output is not validated as a
nonzero scalar, every seed length is accepted, and `depth + 1` can wrap in a
release build. Published vectors pass because they do not exercise these rare
failure boundaries.

Apollo at `ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` requires a 64-byte seed,
hardened-only paths and the same raw key/chain-code outputs. The generic SDK
must retain that compatibility while accepting every normative BIP-32 seed
length and owning no Cardano, wallet or extended-key serialization policy.
This is direct consumer evidence from Apollo; NeoPRISM, Midnight Identity,
Lace ID Portal and Oxid consume or are expected to consume the generic crypto
facade rather than dependency types.

## Normative sources

- [Bitcoin BIP-32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)
  specifies 128–512-bit seed material, invalid master private
  keys when `parse256(IL)` is zero or at least `n`, and invalid children when
  `parse256(IL) >= n` or the derived key is zero.
- [`bip32 0.5.3`](https://crates.io/crates/bip32/0.5.3) crates.io artifact SHA-256:
  `db40d3dfbeab4e031d78c844642fa0caa0b0db11ce1607ac9d2986dff1405c69`.
  Its source matches unsigned tag `bip32/v0.5.3` at
  [`240679a2454945783acc4f9e7d3bae839359b0b7`](https://github.com/iqlusioninc/crates/tree/240679a2454945783acc4f9e7d3bae839359b0b7/bip32);
  the formerly cited `fe053be`
  revision is newer repository state, not the published artifact source.
- Existing exact `k256 0.13` provides `SecretKey::from_slice` and
  `PrimeField::from_repr`/`Scalar` arithmetic needed for exact validation.

## Candidate decisions

| Candidate | Exact version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| Existing `k256` | workspace 0.13 | `reuse-existing` | Exact scalar parsing and arithmetic already sit in the crypto graph and facade; no new coupling. | Reconsider only if its security or target posture regresses. |
| `bip32` | 0.5.3 / `240679a2` | `not-adopt` | k256 backend rejects zero `IL`; mandatory Base58Check/RIPEMD graph serves unused xprv/xpub behavior; master/seed adapter remains local. | A stable narrow API accepts zero `IL`, supports every 16..=64-byte seed, and feature-slices serialization dependencies. |
| `bip32` | 0.6.0-pre.1 | `defer` | Prerelease API on prerelease HMAC/k256/RIPEMD stack. | Stable release plus the 0.5.3 reconsideration conditions. |
| `coins-bip32` | 0.13.2 | `not-adopt` | Broader `std`, arithmetic, serde, `coins-core` and error coupling. | A cohesive primitive-only split materially deletes SDK code. |
| `bitcoin::bip32` | current assessment | `not-adopt` | Imports a broad Bitcoin model and secp256k1 native/FFI boundary for a narrow raw derivation need. | A target-safe primitive split with no Bitcoin/network public policy. |

## Compatibility and dependency evidence

With `default-features = false, features = ["secp256k1"]`, a clean
`bip32 0.5.3` probe resolves 29 third-party packages. Compared by package name
with the current workspace lock, it adds `bip32`, `bs58`, and `ripemd`.
Base58Check and RIPEMD are not optional under that feature despite the planned
adapter not using xprv/xpub serialization. The candidate declares Rust 1.65,
Apache-2.0 OR MIT, is not yanked, and its host and WASM probes pass. The
repository remains active. These positives do not overcome the semantic and
cohesion findings.

The direct crate forbids unsafe code and RIPEMD has no unsafe/native/build
source. `bs58` contains an unsafe String-output implementation behind its
`alloc` feature; that feature is disabled in the measured graph. A RustSec scan
of the 30-package probe lock against 1,242 advisories had no finding. These are
point-in-time signals, not an audit or cryptographic approval.

The selected no-new-dependency design reuses the already-gated k256 graph, so
it adds no MSRV, license, target, advisory, native, unsafe or feature-cone
surface. Integrated Rust 1.85/1.98, portable target, deny, audit and Nix gates
remain mandatory because implementation can still regress them.

Public and wire compatibility remains owned by the `HDKey` facade. No
third-party error, formatter, serialization or feature name crosses that
boundary. Rollback is one issue-linked implementation PR.

## Security, privacy and maintenance evidence

The direct and resolved dependency cone is unchanged by the selected design.
The rejected candidate's exact version, revision, license and provenance are
recorded above. Its repository is maintained and the release is current enough
to assess, but maintenance, release and security posture do not correct the
semantic mismatch. Protocol/draft currency is final BIP-32, not an evolving
draft. The supply-chain evidence and reachable unsafe/native review are
point-in-time evidence only.

### Semantic fitness finding

The `bip32 0.5.3` k256 implementation calls
`NonZeroScalar::from_repr(other)` for the HMAC left half. This rejects zero
before it adds the parent. BIP-32's child rule rejects `IL >= n`, not
`IL = 0`; with a valid parent, a zero tweak yields that same nonzero private key
and the new chain code. This edge is astronomically unlikely from HMAC but is a
standards distinction and must be correct under synthetic conformance testing.

The SDK would also have to retain its master HMAC to support all lengths in
16..=64 because the high-level dependency constructor accepts only 16, 32 or
64 bytes. The candidate therefore does not remove enough risky local mechanics
to justify its broader graph and a zero-tweak workaround.

## Security and lifecycle design

Use `SecretKey::from_slice` to validate master and parent key bytes. Use
`Scalar::from_repr`, not `Reduce`, for `IL`; it accepts zero and rejects values
at or above the order. Add the validated parent scalar, reject zero, serialize
into zeroizing storage, and retain the existing redacted facade. HMAC output
remains in `Zeroizing`. No raw value enters an error, formatter, serializer,
log or public third-party type.

## Rejected or deferred candidates

`bip32 0.5.3`, its prerelease successor, `coins-bip32`, and broad Bitcoin
framework adoption are rejected or deferred with objective reconsideration
triggers in the candidate table. Reimplementing scalar arithmetic is also
rejected: the existing `k256` primitive supplies it. Retaining modular
reduction is rejected because it contradicts the final protocol.

## Open questions and blockers

No research blocker remains. Invalid child handling returns an error for the
explicit requested index rather than silently incrementing it; this preserves
the current API's deterministic caller-owned path policy. Non-hardened/public
derivation and extended-key serialization remain independent future product
decisions.

## Evidence commands

- Inspected the exact registry artifact and matching tag source for feature,
  scalar, formatter, unsafe and native behavior.
- Built a locked minimal-feature probe on host and `wasm32-unknown-unknown`.
- Measured standalone and workspace-incremental dependency names and features.
- Ran the current RustSec database over the probe lock.
- Compared Apollo source and existing SDK tests with all four official private
  derivation vectors.

Exact commands included `cargo info bip32@0.5.3`, `cargo tree -e normal`,
`cargo tree -e features`, `cargo +1.98.1 check --locked`,
`cargo +1.98.1 check --locked --target wasm32-unknown-unknown`, source `rg`
scans, and pinned `cargo audit`. Integrated implementation checks are unrun at
this pre-code checkpoint: Rust 1.85/1.98, mobile targets, public API, deny,
audit, complete Nix and full factory validation remain delivery tasks.

Rollback is one focused issue #153 PR revert. Reconsider only when a stable
candidate meets the ledger trigger and a new ADR supersedes this decision.
