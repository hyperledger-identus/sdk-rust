# ed25519-derivation-canonical-standard

> **Status: Exploring** — one open question blocks promotion. This is a
> pre-change exploration; it is not tracked by `openspec` as a change or spec,
> and is not apply-able. The companion `bip39-salt-kmp-compat.md` covers the
> *other* KMP divergence (a confirmed bug); this one is **not a bug** — it is a
> canonical-standard choice. The two must not be conflated.

## Why

`add-crypto-capability` shipped `EdHDKey` implementing **SLIP-0010** for
ed25519 hierarchical derivation, verified against published SLIP-0010 test
vectors. The KMP `apollo` port origin it was attributed to uses **ed25519-bip32
(Khovratovich)** — a *different algorithm* — and labeled it "SLIP-0010"
incorrectly. The `add-crypto-capability` design recorded this as a "divergence
from the KMP port origin" and sided with the published SLIP-0010 standard.

The reframing that emerged during verification: **ed25519-bip32 is not a bug —
it is the Cardano ecosystem standard** (CIP-1852). `apollo` is an IOHK (Input
Output) project, and the `ed25519-bip32` + `cryptoxide` crates are the IOHK
Cardano stack. So `add-crypto-capability`'s SLIP-0010 default is a choice of a
*different standard* than the Cardano/PRISM ecosystem actually uses — not a
bug-fix. That raises a question the original change did not answer: **which
standard should be the SDK's canonical ed25519 derivation?**

## Root cause — a labeling error, not an implementation bug

**KMP source:** `apollo/apollo/src/jvmMain/kotlin/org/hyperledger/identus/apollo/derivation/EdHDKey.kt`

```kotlin
import uniffi.ed25519_bip32_wrapper.deriveBytes
import uniffi.ed25519_bip32_wrapper.fromNonextended

actual fun deriveChild(wrappedIndex: BigIntegerWrapper): EdHDKey {
    val derived = deriveBytes(privateKey, chainCode, index)   // ed25519-bip32
    ...
}

actual fun initFromSeed(seed: ByteArray): EdHDKey {
    require(seed.size == 64)                                    // 64-byte seed, split directly
    val keySlice = seed.sliceArray(0 until 32)
    val chainCodeSlice = seed.sliceArray(32 until seed.size)
    val result = fromNonextended(keySlice, chainCodeSlice)      // NO HMAC master step
    ...
}
```

This calls the `ed25519-bip32` crate (Khovratovich "Universal Wallets" paper,
the algorithm underlying Cardano's CIP-1852 HD wallet standard) — **not
SLIP-0010**. Different algorithm: 64-byte extended keys, no HMAC-SHA512 master
step, different derivation math. The only error is the *name*: KMP labels this
`EdHDKey` and the spec/task attribution called it SLIP-0010.

```
                  algorithm             key size       init_from_seed
                  ─────────             ────────       ──────────────
KMP apollo:       ed25519-bip32         64-byte ext    split seed into key+chain
                  (Khovratovich/CIP-1852) (IL||IR)     NO HMAC master step

identus-crypto:   SLIP-0010             32-byte        HMAC-SHA512 keyed "ed25519 seed"
                  (shipped by add-crypto-capability)   hardened-only; child key = IL
```

Same seed → different derived private keys at every path. Hard incompatibility.

## The open question — which is the canonical default?

```
┌──────────────────────┬───────────────────────────────┬───────────────────────────────────────┐
│                      │ SLIP-0010 as canonical default │ ed25519-bip32 as canonical default     │
│                      │ (add-crypto-capability's pick) │ (invert: ecosystem-aligned)           │
├──────────────────────┼───────────────────────────────┼───────────────────────────────────────┤
│ Cardano/PRISM interop│ ❌ wrong default; needs _kmp   │ ✅ matches the ecosystem's wallets     │
│ Cross-ecosystem      │ ✅ recognized standard (SLIP)  │ ⚠️ Cardano-specific; less recognizable │
│  recognizability     │                                │   outside the Cardano ecosystem        │
│ "reference to port   │ ✅ published standard          │ ✅ ecosystem standard (but narrower     │
│  from" stance        │                                │   port-from audience)                  │
│ Inverts add-crypto-  │ no (status quo)               │ yes — flips a just-shipped default     │
│  capability?         │                                │                                        │
│ Field usage today    │ cloud-agent uses secp256k1     │ cloud-agent uses secp256k1 BIP32 only; │
│                      │ BIP32 only; EdHDKey unused     │ EdHDKey ed25519-keyed consumer unconfirmed │
└──────────────────────┴───────────────────────────────┴───────────────────────────────────────┘
```

This is **not resolved**. It blocks promotion to a change because it determines
whether `ed25519-bip32` lives behind a `kmp-compat` feature (as the original
exploration assumed) or is the canonical default with SLIP-0010 as the opt-in
(inverting `add-crypto-capability`'s stance).

Two sub-questions a follow-up grilling should resolve:

1. **Is the Identus SDK's "port *from*" audience the Cardano ecosystem** (→
   ed25519-bip32 as default), **or a cross-ecosystem audience** where SLIP-0010
   is more recognizable (→ SLIP-0010 stays default)?
2. **What do existing PRISM wallets in production actually use for ed25519-keyed
   DIDs** — ed25519-bip32, or is `EdHDKey` unused in practice? `cloud-agent` uses
   only secp256k1 `HDKey` (BIP32, not divergent), so the ed25519-bip32 interop
   need may be partly theoretical until an ed25519-keyed PRISM consumer is
   identified.

## Resolved sub-decisions (from the grilling that reframed this question)

These were locked before the reframing redirected focus to the salt issue. They
remain valid **provided** the open question resolves to "SLIP-0010 canonical,
ed25519-bip32 behind `kmp-compat`" (the original assumption). If the open
question resolves the other way (ed25519-bip32 canonical), the feature naming
inverts but the mechanics below still apply.

### Ship both algorithms — locked

Both SLIP-0010 and ed25519-bip32 will be available. The two divergences
compose for any ed25519-keyed KMP wallet: KMP's flow is `createSeed` (KMP
salt) → `EdHDKey.initFromSeed` (ed25519-bip32). To reproduce a KMP-derived
ed25519 DID from a mnemonic, you need **both** `create_seed_kmp` **and**
ed25519-bip32 derivation. Shipping both in one change is correct (the
`create_seed_kmp` side is resolved in `bip39-salt-kmp-compat.md`).

### Implementation source — cargo git dep on `typed-io/rust-ed25519-bip32` — locked

A pure-Rust `ed25519-bip32` crate exists, MIT OR Apache-2.0, depending only on
`cryptoxide` (`#![no_std]`, wasm-clean). Two sources:

- **In the workspace:** `apollo/bip32-ed25519/rust-ed25519-bip32/` (the exact
  crate KMP calls via its uniffi wrapper — `wrapper/src/wrapper.rs` maps
  `deriveBytes`/`fromNonextended` to `XPrv::derive`/`XPrv::from_nonextended_force`).
- **Upstream:** `https://github.com/typed-io/rust-ed25519-bip32.git`, tag
  `ed25519-bip32-v0.4.1` (commit `5f4b418`) — the same version the submodule
  vendors.

**Decision: cargo git dep on the upstream repo**, declared at workspace level
per `add-workspace-dependency-conventions`:
```toml
# root Cargo.toml [workspace.dependencies]
ed25519-bip32 = { git = "https://github.com/typed-io/rust-ed25519-bip32.git", tag = "ed25519-bip32-v0.4.1" }
```
referenced via `ed25519-bip32 = { workspace = true, optional = true }` in
`crates/crypto/Cargo.toml`.

This was chosen over vendoring-in-tree + reworking onto `sha2`/`hmac`/`ed25519-dalek`
after weighing:
- **License:** `cryptoxide` is `MIT/Apache-2.0` (compatible).
- **Audit posture:** no known RustSec advisories (zero hits in
  `rustsec/advisory-db`), actively maintained (last commit 2026-07-02, ~1.8M
  downloads), `#![no_std]`/wasm-clean. However, **no evidence of a formal
  third-party audit** — it sits a tier below the RustCrypto/dalek crates
  `add-crypto-capability` Decision 6 curated for *audit posture*. The git dep
  therefore introduces a second, less-audited crypto primitive stack
  (`cryptoxide`'s SHA-512/HMAC/ed25519/curve25519) alongside the curated one.
  This is the accepted trade-off of the git-dep choice vs. the
  vendor+rework choice (which would have eliminated `cryptoxide` entirely).
- **API shape:** the crate's API is `XPrv`/`XPub`-shaped (uniffi-wrapper
  ergonomics), so `identus-crypto` still needs a thin adapter to bridge into
  `crypto::Error`, snake_case names, and `SecureRandom` injection — but the
  algorithm core is behind the git dep, not re-implemented.

### Crate layout — `kmp-compat` feature in `identus-crypto` — locked (mechanics)

```toml
# crates/crypto/Cargo.toml
ed25519-bip32 = { workspace = true, optional = true }
[features]
kmp-compat = ["dep:ed25519-bip32"]
```

A sibling crate (`identus-crypto-kmpcompat`) was rejected because the forcing
function that justified exiling `ring` to `identus-adapters-entropy` does **not**
apply here: `ring` is wasm-*incompatible* (had to leave the domain crate to keep
crypto wasm-clean-by-default), whereas `cryptoxide` is wasm-*compatible*, so the
whole `kmp-compat` feature builds on `wasm32-unknown-unknown`. Additionally,
`create_seed_kmp` (the salt variant) needs no new dependency, which would make a
sibling crate awkward (it would exist solely for the cryptoxide-bearing
ed25519-bip32, splitting the "kmp-compat" concept across two crates).

The git dep is `optional` + `dep:`-gated, so default `identus-crypto` stays
`cryptoxide`-free; `--features kmp-compat` pulls it. The layer guard stays green
(the dep is external, not an `identus-*` crate).

> **Caveat from the open question:** if the open question resolves to
> "ed25519-bip32 canonical default", the feature inverts — SLIP-0010 (already
> in-tree, no new dep) becomes the always-on default, and the `kmp-compat`
> feature may instead gate *SLIP-0010 availability as an opt-in* or simply
> disappear (since SLIP-0010 has no extra dep). The mechanics above assume the
> original "SLIP-0010 canonical" framing.

## Non-goals

- No migration/re-keying helper (wallet-layer concern, same as the salt case).
- No BIP32 secp256k1 `HDKey` variant — it is **not divergent** between KMP and
  `identus-crypto` and needs no interop variant.
- No re-implementation of ed25519-bip32 onto `sha2`/`hmac`/`ed25519-dalek`
  (that was the vendor+rework option, rejected in favor of the git dep).

## Notes

- `HDKey` (BIP32 secp256k1) is **not divergent** — only `EdHDKey` and the BIP39
  salt are. `cloud-agent` uses only the non-divergent secp256k1 path.
- The `kmp-compat` feature is shared with `bip39-salt-kmp-compat.md`'s
  `create_seed_kmp`. The two interop pieces compose: a full KMP ed25519-wallet
  import needs `create_seed_kmp` (KMP salt) **and** ed25519-bip32 derivation.
- This exploration emerged from the verification of `add-crypto-capability`.
  The original change's `design.md` records the SLIP-0010-vs-ed25519-bip32
  divergence but frames it as a KMP port-origin error; the reframing here
  (ed25519-bip32 is the Cardano ecosystem standard) means that framing is
  incomplete and the canonical-default question is genuinely open.