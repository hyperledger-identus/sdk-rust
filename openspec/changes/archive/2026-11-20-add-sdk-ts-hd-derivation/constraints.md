# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/612
Constraint blockers: none

## Existing entries affected

- `SDK-ARCH-001` remains effective: Rust owns portable derivation semantics and
  target adapters depend inward.
- `SDK-SEC-003` remains effective: seed, path, and generated output boundaries
  are bounded before semantic or cryptographic work.
- `SDK-COMPAT-002`, `SDK-COMPAT-004`, and `SDK-COMPAT-005` remain effective for
  Rust 1.99.1, strict language mapping, and target evidence.
- `SDK-LIM-002`, `SDK-LIM-003`, and `SDK-LIM-009` remain effective for private
  bindings, non-certification, and isolated CI evidence.
- The crypto spec, TypeScript portability contract, TS1 SC-01 through SC-09,
  issue #612, and the merged key-operations boundary remain controlling.

## Introduced or changed constraints

Every derivation request selects one immutable named profile. A profile cannot
silently change master-key construction, hardened/soft rules, private/public
derivation behavior, key shape, or path semantics.

Seed ingress accepts an exact profile-valid mutable `Uint8Array`, adopts a Rust
copy synchronously, and overwrites the supplied buffer before returning or
throwing. Derived private and chain-code material remains inside Rust-owned
WASM objects until explicit disposal or generated-owner finalization. It is
never exported, logged, observed, serialized into errors, returned by debug
formatting, or retained in TypeScript DTOs.

Path text is canonical, bounded, and parsed by Rust. Secp256k1 BIP-32 and
Ed25519 SLIP-0010 accept hardened private axes only. Cardano V2 accepts hard or
soft private axes and soft-only public axes. No implicit profile conversion or
fallback exists.

## Introduced or changed limitations

This is a private, opt-in, non-default Browser/Node/React-web facade. React
Native requires separately qualified native bindings. WASM cannot promise
physical erasure of every engine copy or snapshot; the contract proves logical
ownership, Rust drop-time zeroization, caller-input overwrite, and exclusion
from public values and diagnostics.

No mnemonic generation/validation, BIP-39 seed creation, X25519 conversion,
general soft secp256k1 derivation, secp256k1 public derivation, private export,
keystore, KMS, hardware key, persistence, wallet, publication, certification,
production, release, or mdoc claim is included.

## Activation and rollback

Activation requires issue-bound planning, exact donor and normative vectors,
focused and complete local gates, secret/profile review, signed+DCO commits,
green hosted CI, and protected merge. Rollback selects the legacy host route
before initialization or removes the unreleased capability; no automatic
fallback after seed import or derivation is permitted.

## Consumer and product impact

Existing private package APIs and the external SDK-TS route remain unchanged.
The additive provider is opt-in and selected before initialization. Consumers
gain named deterministic profiles and opaque derived keys while retaining all
mnemonic acquisition, recovery-material sourcing, permission, persistence,
backup, user-consent, and product policy.

## Evidence

Issue #612; base `48b3c1dc34969016fe8fbf7bba1a68756b312d7a`;
BIP-0032; SLIP-0010; Cardano/IOG Ed25519-BIP32 V2; the canonical crypto spec;
Apollo parity and resource registries; SDK-TS revision
`4bf86ebf69d5e96616a148e4c973f831f95fa38e`; exact Cargo/npm locks;
generated declaration/artifact identities; focused secret/profile review;
target receipts; and protected exact-head CI are the atomic evidence owners.
