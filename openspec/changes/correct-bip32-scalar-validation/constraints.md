# Constraint and limitation impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/153
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001` remains effective: BIP-32 raw derivation is generic cryptographic
mechanics without wallet, account, chain or product policy. `SDK-COMPAT-002`
and `SDK-COMPAT-004` remain Rust 1.85.0 and primary Rust 1.98.1. Reusing the
existing graph introduces no new MSRV claim.

`SDK-SEC-001` remains effective because no unsafe block or native dependency is
added. `SDK-SEC-002` governs seed, HMAC, private-key, scalar and chain-code
temporaries. `SDK-DELIVERY-001` is satisfied by issue #153 and this lifecycle.

## Introduced or changed constraints

No repository-wide constraint value changes. The directed decision requires
exact BIP-32 scalar validation through the existing `k256` primitive and
prohibits modular reduction of master or child values. It records
`bip32 0.5.3` as not adopted for this boundary; no BIP-32 framework type,
extended-key serializer, Base58Check, RIPEMD, public derivation or non-hardened
behavior is authorized.

## Introduced or changed limitations

Seed inputs shorter than 16 bytes or longer than 64 bytes, invalid master
scalars, child `IL >= n`, zero child results and depth overflow become errors.
A zero child tweak remains valid when its resulting key is nonzero. The facade
continues to derive only an explicitly requested hardened index and returns an
error rather than automatically selecting the next index.

The SDK retains a small local BIP-32 HMAC orchestration because no assessed
crate matches its narrow semantics and cohesion requirements. Compile and
source review cannot guarantee physical erasure; caller copies of public raw
fields remain caller-owned.

## Consumer and product impact

Consumers retain source-compatible `HDKey` signatures, fields, Apollo outputs
and official vector results. Only invalid or previously underspecified inputs
change behavior. No persisted encoding, binding, UI, custody, account, Cardano
or downstream repository behavior changes.

## Activation and rollback

The behavior activates only when issue #153's PR merges into `develop` after
the BIP-39 predecessor and every hosted gate are green. Reverting the focused
PR restores the previous implementation; no persisted data migration is
required.

## Evidence

Research records normative semantics, exact candidate provenance, feature and
dependency cones, maintenance, licenses, advisory result, unsafe/native scan,
target probes and the zero-tweak mismatch. Implementation requires official
vectors, synthetic scalar/seed/depth cases, redacted errors, zeroizing/public
API evidence, Rust 1.85/1.98, portable targets, deny, audit, full Nix/factory
and distinct correctness/security review.
