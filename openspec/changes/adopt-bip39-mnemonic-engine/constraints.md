# Constraint and limitation impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/152
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001` remains effective because BIP-39 is reusable cryptographic
mechanics with no wallet, chain or product policy. `SDK-COMPAT-002` and
`SDK-COMPAT-004` remain Rust 1.85.0 and primary Rust 1.98.1; the selected
resolution must pass both because the direct crate declares no machine-readable
Rust floor.

`SDK-SEC-001` remains effective: no SDK unsafe block is proposed, while exact
transitive Rust unsafe is recorded rather than represented as absent.
`SDK-SEC-002` governs mnemonic, passphrase and seed temporaries and prohibits
dependency formatting or ordinary owned normalization buffers at the facade.
`SDK-DELIVERY-001` is satisfied by issue #152 and this lifecycle.

## Introduced or changed constraints

No repository-wide constraint value changes. The directed decision permits
exact `bip39 2.2.2` with only `alloc,zeroize` inside `identus-crypto` while the
dependency remains private, English-only, non-RNG, non-serde, and protected by
the specified normalization/lifecycle adapter. It does not authorize public
dependency types, other language features, crate RNG, serde, or use outside the
mnemonic implementation.

## Introduced or changed limitations

Previously accepted invalid word counts and checksums become errors. The
infallible entropy conversion retains its existing empty-vector failure shape
rather than adding a breaking `Result`. The KMP compatibility passphrase stays
byte-exact and intentionally is not normalized; it is an import escape hatch,
not a second standards-compliant BIP-39 API.

The graph gains seven pure-Rust packages and transitive unsafe implementation
code. Compile checks do not prove runtime platform certification or guaranteed
physical memory erasure. Returned `Vec<String>` mnemonic words and `Vec<u8>`
seed bytes are deliberately caller-owned; callers control their eventual
storage and erasure.

## Consumer and product impact

Consumers retain source-compatible method signatures and official ASCII vector
results. They gain correct NFKD behavior and fail-closed invalid input handling.
No UI, recovery ceremony, persistence, custody, binding, downstream repository
or release behavior changes.

## Activation and rollback

The dependency and stricter behavior activate only when the issue #152 PR
merges to `develop`, after baseline PR #180 is integrated and every hosted gate
is green. Reverting that focused PR restores the former implementation; no
persisted format migration is required.

## Evidence

Research records exact tag/artifact provenance, candidate features, standalone
and incremental cones, licenses, advisory result, unsafe/native scan, formatter
hazards and lifecycle design. Implementation requires official and Unicode
vectors, negative cases, KMP parity, public API/feature checks, Rust 1.85/1.98,
portable targets, full factory/Nix and distinct security review.
