# Research readiness

Research class: cryptography-security
Research status: ready
Decision date: 2026-09-08
Source retrieval date: 2026-09-08
Research blockers: none

## Problem and existing implementation

The current implementation provides `EdHDKey`, a locally implemented
hardened-only SLIP-0010 hierarchy with 32-byte private keys. Apollo at revision
`ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` instead pins
`input-output-hk/rust-ed25519-bip32@e9d995b1fe29c428d5f569ba96700872f68fab88`
and exposes a 64-byte extended secret plus 32-byte chain code, V2 private
derivation, and soft public derivation through native and WASM wrappers.

The named consumers are Apollo parity in `identus-crypto`, subsequent
NeoPRISM reduction, and reusable Cardano-compatible cryptography. No consumer
requires ledger, address, CIP-1852 path-policy, custody or binding behavior in
this slice. The current implementation and proposed types remain separate so
SLIP-0010 callers cannot silently change algorithm.

## Normative sources

- Khovratovich and Law,
  [Ed25519 BIP32](https://input-output-hk.github.io/adrestia/static/Ed25519_BIP.pdf),
  scheme V2.
- Apollo baseline
  [`hyperledger-identus/apollo@ccee22b`](https://github.com/hyperledger-identus/apollo/tree/ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c).
- Apollo native donor
  [`input-output-hk/rust-ed25519-bip32@e9d995b`](https://github.com/input-output-hk/rust-ed25519-bip32/tree/e9d995b1fe29c428d5f569ba96700872f68fab88).
- Published candidate signed tag
  [`ed25519-bip32-v0.4.3`](https://github.com/typed-io/rust-ed25519-bip32/releases/tag/ed25519-bip32-v0.4.3)
  at revision `6539dc9f792174fa5c2290c9e0a23710a1e1ecef`.

## Candidate decisions

| Candidate | Exact version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| `ed25519-bip32` | 0.4.3 / `6539dc9` | `conditional-adopt` | Exact V2 mechanics, maintained donor continuation, two-package cone, supported targets. Must remain behind redacted zeroizing SDK types. | Formatting/derivation regression, advisory, maintenance loss, target failure, or a safer exact implementation. |
| `ed25519-bip32-core` | 0.1.1 / `KeystoneHQ` master | `not-adopt` | Same key model but still exposes `XPrv` through `Debug`/`Display`, uses older `cryptoxide 0.4`, and last source update was 2023. | Maintained release removes secret formatting and passes the full candidate gates. |
| `outscript` | 0.1.2 | `oracle` | Independent Cardano V2 and Icarus behavior, but a young broad multi-chain cone with no zeroizing Cardano secret type. | A narrow audited derivation crate is split out with stable API and secret hygiene. |
| `pallas-wallet` | 1.0.0-alpha.1 | `not-adopt` | Wraps `ed25519-bip32 0.4.1`; does not replace the implementation and derives `Debug` for the wrapper. | Pallas publishes an independent hardened implementation with safe key types. |
| `ed25519-dalek-bip32` | 0.3.0 | `not-adopt` | SLIP-0010 hardened-only semantics; no soft child or extended public derivation. | Never for Cardano V2; it may be reconsidered for the separate SLIP-0010 contract. |
| RustCrypto `bip32` | 0.5.3 | `not-applicable` | Standard BIP-32 generic core and secp256k1 backends do not implement the Cardano Ed25519 V2 algorithm. | Never for this algorithm; issue #153 evaluates it for secp256k1 BIP-32. |

## Compatibility and dependency evidence

The candidate declares an MSRV of Rust 1.81, edition 2021, no crate-level
default features, `no_std`, MIT OR Apache-2.0 and one direct normal dependency:
`cryptoxide 0.6`. Its published lockfile resolves `cryptoxide 0.6.0`; SDK
integration resolves and locks `cryptoxide 0.6.5` with checksum
`b8bb2a43afb0fd8b53101f79f153795c449ba80762361d8b868f4a75c6d03057`.
A fresh probe resolved exactly two packages. It compiled on the effective Rust
1.85 host and on primary Rust 1.98.1 for `aarch64-apple-darwin`,
`wasm32-unknown-unknown`, `aarch64-apple-ios` and
`aarch64-linux-android`. These are compile checks, not runtime certification.

A tree diff from Apollo's donor to 0.4.3 shows no change to `src/derivation/v2.rs`.
Behavior-relevant changes are `cryptoxide 0.4` to `0.6` API migration; remaining
changes set edition 2021, declare Rust 1.81, make the crate unconditional
`no_std`, replace allocating hex formatting with streaming formatting, and
move `std` imports to `core`. Apollo's hard private vector is retained, and
focused tests will prove the published release matches hard, soft and public
V2 behavior byte-for-byte before delivery.

The public and wire compatibility boundary is entirely SDK-owned. No
dependency type, error, serialization, formatter or feature name is reused as
an Identus contract. The facade stores 96 owned bytes and reconstructs
short-lived dependency values internally. Rollback removes one module, one
feature and one workspace dependency.

## Security, privacy and maintenance evidence

The direct and resolved dependency cone contains the candidate plus its one
normal dependency. The package cone is narrow, but the feature cone is not:
`ed25519-bip32` declares
`cryptoxide = "0.6"` without `default-features = false`, so Cargo activates all
of `cryptoxide 0.6.5`'s defaults (including unrelated symmetric, password,
hashing, and post-quantum modules) even though this derivation uses only
Curve25519/Ed25519, SHA-512, HMAC, and constant-time helpers. A direct SDK
dependency cannot subtract features requested by an upstream dependency; this
requires an upstream release or a reviewed fork.

The candidate's `XPrv::Debug` and `Display` render all 96 bytes. Its drop path
contains one unsafe `core::ptr::write_bytes` block. A full lexical scan of the
SDK-resolved `cryptoxide 0.6.5` found 161 unsafe/target-feature/native-keyword
occurrences across 24 Rust source files and found no C/C++/ASM source or build
script. The earlier candidate-lock probe of `cryptoxide 0.6.0` found 90 across
15 files. This is transitive unsafe and compiled third-party surface, not an
unsafe block added to SDK source. The subset invoked by the facade remains
bounded by the candidate's imports, but the broad defaults are explicitly
accepted only as a temporary residual risk behind the private boundary.

The SDK-owned private type independently derives `Zeroize` and
`ZeroizeOnDrop`, implements a redacted `Debug`, has no `Display` or serde, and
does not retain dependency values between operations. Errors collapse to the
stable redacted `crypto.derivation_failed` surface. Explicit raw export is the
only caller-visible secret-copy operation.

The exact candidate lockfile passed the current RustSec supply-chain scan on
2026-09-08 with two crate dependencies and no advisory reported; this is not a
cryptographic audit. The signed 0.4.3 tag and commit are verified, the upstream
repository is not archived and was updated 2026-07-29, and `cryptoxide` was
updated 2026-09-03. License and provenance are compatible with the workspace.
Protocol/draft currency is scheme V2 as used by Apollo and Cardano; this change
does not infer BIP-32 or SLIP-0010 equivalence.

## Rejected or deferred candidates

The candidates above are rejected for semantic mismatch, wrapper-only value,
secret-formatting, maintenance or dependency-coupling reasons. Unpublished
implementations such as `pallas-extras/bip32` remain source references only:
its private key is `Debug + Copy` and its current private derivation accepts
only hardened indices. `cardano-serialization-lib` and `cml-crypto` bring broad
cones while delegating to an older `ed25519-bip32`, so neither reduces the
identified risk.

No fresh cryptographic implementation is proposed. If upstream hardening is
unavailable, a minimal temporary fork of the exact adopted revision is safer
than transcribing the algorithm; the fork must retain vectors and carry a
sunset trigger back to upstream.

## Open questions and blockers

No blocker remains for the bounded facade. Upstream removal of secret
formatting, replacement of manual zeroing, and disabling unrelated
`cryptoxide` defaults are desirable but are not claimed complete. They are a
focused follow-up in issue
[#179](https://github.com/hyperledger-identus/sdk-rust/issues/179) with an exact
fallback, while SDK-owned storage, formatting, errors and exports satisfy the
current security boundary.

## Evidence commands

- `cargo info ed25519-bip32@0.4.3` recorded exact metadata and features.
- `git diff FETCH_HEAD ed25519-bip32-v0.4.3 -- Cargo.toml src README.md`
  compared Apollo donor `e9d995b` with the published signed tag.
- `cargo +1.85.0 check --locked` passed in a copied candidate package.
- Nix primary Rust 1.98.1 `cargo check --locked` passed on host,
  `wasm32-unknown-unknown`, `aarch64-apple-ios` and
  `aarch64-linux-android`.
- Candidate `cargo tree --locked --edges normal --prefix none` resolved only
  `ed25519-bip32 0.4.3` and `cryptoxide 0.6.0`; SDK integration resolves
  `cryptoxide 0.6.5` and `cargo tree -e features -i cryptoxide` proves that the
  upstream declaration activates all cryptoxide default features.
- `rg` scanned the candidate and resolved dependency for unsafe, native source
  and build scripts; exact output is summarized above.
- `cargo audit --file <candidate>/Cargo.lock` loaded 1,242 RustSec advisories
  and reported no vulnerability for the two-package lockfile.
- Workspace integration, public-API inspection, vectors, dependency policy and
  full Nix gates were deliberately unrun at the research-ready checkpoint;
  implementation results belong to the separate verification receipt.
