# Design

## Capability split

`identus-crypto` remains the sole profile and path semantic owner.
`identus-wasm-crypto` owns generated opaque derivation handles and projection
of public material. The private TypeScript package owns strict ingress,
immutable public values, lifecycle wrappers, observation, target loading,
selection, and rollback. TypeScript implements no path parser, derivation
algorithm, or canonical key parser.

The existing separate crypto artifact remains the correct boundary: derivation
handles compose with its secret lifecycle without entering the public-material
DID artifact. Core Rust completes before independently selected binding jobs.

## Selected profiles

`secp256k1-bip32-hardened-v1` accepts 16-to-64-byte seeds and hardened private
paths only. Its public projection is the 33-byte compressed SEC1 key. It makes
no general soft/private or public BIP-32 claim.

`ed25519-slip10-hardened-v1` accepts 16-to-64-byte seeds and hardened private
paths only. Its public projection is the 32-byte Ed25519 verification key. It
is not presented as SDK-TS/Cardano compatibility.

`ed25519-cardano-v2-v1` accepts exactly the existing 64-byte Apollo-compatible
seed representation. Private handles derive through hard or soft axes; public
handles contain the 32-byte Ed25519 public key plus 32-byte chain code and
derive only through soft axes. Hardened public derivation fails closed.

## Opaque lifecycle and projections

Each generated Rust owner contains a profile-specific extended private or
public key, exposes only permitted derivation and public projection methods,
rejects use after free, and is freed exactly once. TypeScript wrappers are
explicit disposable values with idempotent `dispose`; finalization is defense
in depth. Public values contain copied immutable bytes, profile, and depth only.

Seed import accepts only a mutable `Uint8Array` so the adapter can overwrite
the caller buffer synchronously in `finally`. Rust secret types retain their
zeroizing storage. Errors, declarations, debug text, artifacts, and receipts
contain no seed, private key, chain code, or derived secret bytes.

## Path, resource, and error contract

The canonical path grammar is Rust-owned `m` followed by at most 255 decimal
axes, each optionally suffixed by one apostrophe. The UTF-8 text ceiling is
4,096 bytes. Empty components, partial numbers, signs, whitespace within axes,
values at or above 2^31, and profile-forbidden axes fail before derivation.

Stable errors close over invalid seed, invalid path, forbidden axis, depth
overflow, invalid public material, lifecycle, projection, observer, and binding
contract. Observations include only operation, profile, route, result category,
stable code, depth, and bounded duration.

## Targets and artifacts

The deterministic crypto artifact adds derivation exports without changing the
DID artifact identity contract. Node ESM/CommonJS, browser ESM, React SSR, and
hydrated React client run one neutral derivation assertion body. React contains
no derivation logic. React Native remains explicitly inapplicable. The existing
Rust-core-first impact plan selects the isolated crypto target matrix.

## Rollback

Provider selection occurs before generated module initialization. Once the HD
capability is selected, seed, path, derivation, projection, observer, and
lifecycle failures are terminal. Rollback is a new legacy provider choice
before initialization; no retry, fallback, secret export, or cross-provider
migration occurs automatically.
