# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/611
Constraint blockers: none

## Existing entries affected

- `SDK-ARCH-001` remains effective: Rust owns portable cryptographic semantics
  and target adapters depend inward.
- `SDK-SEC-003` remains effective: messages, signatures, public material, and
  imported secrets are bounded before semantic use.
- `SDK-COMPAT-002`, `SDK-COMPAT-004`, and `SDK-COMPAT-005` remain effective for
  Rust 1.99.1, strict language mapping, and target evidence.
- `SDK-LIM-002`, `SDK-LIM-003`, and `SDK-LIM-009` remain effective for private
  bindings, non-certification, and isolated CI evidence.
- The crypto spec, secret-storage boundary, TypeScript portability contract,
  TS1 SC-01 through SC-09, and issue #611 remain controlling.

## Introduced or changed constraints

Private keys generated or imported through the facade remain in Rust-owned
WASM objects until explicit disposal or generated-owner finalization. Public
bytes may be copied out. Private bytes are never exported, logged, observed,
serialized into errors, returned by debugging, or retained in TypeScript DTOs.

Import is an explicit caller-owned custody boundary. The facade copies exactly
32 bytes into Rust and synchronously overwrites the supplied mutable
`Uint8Array` before returning or throwing. Callers remain responsible for any
other aliases or prior copies. Generation uses the reviewed system entropy
adapter and has no caller-supplied randomness fallback.

Ed25519 owns generation, private import, public import/export, signing, and
strict verification. X25519 owns generation, private import, public
import/export, and contributory shared-secret derivation. Every operation is
curve- and purpose-specific; no generic algorithm string or implicit
Ed25519-to-X25519 conversion exists.

## Introduced or changed limitations

This is a private, opt-in, non-default Browser/Node/React-web facade. React
Native requires separately qualified native bindings. WASM memory cannot
promise physical zeroization against every engine copy, snapshot, allocator,
or compromised host; the contract proves logical ownership, drop-time
zeroization of Rust secret types, input overwrite, and absence from public
values and diagnostics.

No HD derivation, mnemonic, seed, secp256k1, P-256, private export, keystore,
KMS, hardware key, permission, persistence, wallet, publication, certification,
production, release, or mdoc claim is included.

## Activation and rollback

Activation requires issue-bound planning, exact donor evidence, focused and
complete local gates, secret-handling review, signed+DCO commits, green hosted
CI, and protected merge. Rollback selects the existing legacy provider before
initialization or removes the unreleased key capability; no automatic fallback
after key creation or import is permitted.

## Consumer and product impact

Existing private package APIs and the external SDK-TS route remain unchanged.
The additive provider is opt-in and selected before initialization. Consumers
that adopt it gain purpose-specific opaque key operations but keep all custody,
permission, persistence, backup, recovery, user-consent, and product policy.

## Evidence

Issue #611; base `d291a446ece0e932f7f00cbe19da34b72b36bb17`;
RFC 8032; RFC 7748; the canonical crypto spec; Apollo parity and resource
registries; SDK-TS revision `4bf86ebf69d5e96616a148e4c973f831f95fa38e`;
exact Cargo/npm locks; generated declaration/artifact identities; focused
secret-handling review; target receipts; and protected exact-head CI are the
atomic evidence owners.
