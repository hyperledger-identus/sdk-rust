# Design

## Capability split

`identus-crypto` remains the sole semantic owner. A new
`identus-wasm-crypto` outer-boundary artifact owns generated opaque handles and
system entropy composition. The private TypeScript package owns strict ingress,
immutable public projection, lifecycle wrappers, observation, target loading,
selection, and rollback. TypeScript implements no cryptographic algorithm or
canonical key parser.

The dependency rulebook admits exactly one reviewed same-layer composition
edge, `identus-wasm-crypto -> identus-adapters-entropy`, because the generated
outer-boundary artifact is the composition root for the existing production
entropy adapter. Every other outer-boundary-to-outer-boundary dependency stays
denied, and a conformance regression test locks both sides of that boundary.

Keeping crypto separate from `identus-wasm-did` prevents secret-bearing APIs
from entering the public-material DID artifact and lets CI and artifacts evolve
independently. The package composes both generated modules behind capability
providers without exposing generated classes.

## Selected profiles

The Ed25519 profile supports system generation, exact 32-byte private import,
exact 32-byte public import/export, derived public keys, signing, and strict
verification. Signatures are exactly 64 bytes.

The X25519 profile supports system generation, exact 32-byte private import,
exact 32-byte public import/export, derived public keys, and shared-secret
derivation. Public input must be canonical and contributory; an all-zero result
fails closed. Private export and implicit curve conversion are absent.

## Opaque private lifecycle

Generated Rust owners contain typed private keys, expose only purpose-specific
methods, reject use after free, and are freed exactly once. TypeScript wrappers
are explicit `Disposable` values with idempotent `dispose`; a registered
finalizer is defense in depth, not lifecycle authority. Public values are
immutable copied bytes. Observers see only operation, curve, result category,
stable error code, and bounded duration.

Private import accepts only a mutable `Uint8Array` so the adapter can overwrite
the caller buffer synchronously in `finally`. Rust secret types retain their
existing zeroizing storage. Errors, declarations, debug text, artifacts, and
receipts contain no secret, message, signature, or shared-secret bytes.

## Resource and error contract

Message work is caller-budgeted but the facade imposes an explicit maximum
message length before crossing WASM. Keys and signatures have fixed lengths.
Every generated return is shape-checked, copied, and freed. Stable errors close
over invalid input, unsupported lifecycle, invalid signature, weak or
non-contributory peer, entropy failure, observer failure, and binding contract.
Verification returns a boolean only for a well-shaped cryptographic mismatch;
malformed input remains an error.

## Targets and artifacts

The deterministic artifact bundle contains independently hashed DID and crypto
web/node modules. Browser, Node ESM/CommonJS, React SSR, and hydrated React
client run one neutral key assertion body. React contains no key logic. React
Native is explicitly inapplicable. The fail-closed impact plan selects crypto
targets without changing the protected Rust-core-first aggregation contract.

## Rollback

Provider selection occurs before either generated module initializes. Once a
key capability is selected, every initialization, entropy, import, operation,
projection, observer, and lifecycle failure is terminal. Rollback is a new
provider choice before initialization; no legacy fallback, retry, key copying,
or cross-provider migration occurs automatically.
