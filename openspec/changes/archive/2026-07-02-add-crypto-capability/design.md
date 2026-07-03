## Context

The Identus crypto capability has two existing homes:

- **`apollo/` (KMP)** — the canonical platform capability. Full surface: Ed25519/X25519/secp256k1 keypair generation + signing + verification; `KMMECPoint` + curve validation; `Encodable`; `KMMEllipticCurve` (secp256k1 **and secp256r1**); `Secp256k1Lib` (createPublicKey/derivePrivateKey/sign/verify/compress/uncompress); `SecureRandom`; `derivation` (BIP32 `HDKey`, SLIP-0010 `EdHDKey`, BIP39 `MnemonicHelper` + wordlists + PBKDF2-SHA512); `ConvertEd25519` (ed→x25519 via SHA512+clamp); `hashing.PBKDF2SHA512`.
- **`neoprism/lib/apollo/` (Rust)** — a partial Rust port, crate `identus-apollo`, feature-gated (`ed25519`,`secp256k1`,`x25519`,`hash`,`hex`,`base64`,`jwk`,`serde`,`openapi`). Traits `EncodeVec`/`EncodeArray<N>`/`Verifiable`, `EncodeJwk` + `Jwk`, `Sha256Digest`, `HexStr`, `Base64UrlStrNoPad`. **Ed25519 and X25519 are verify/public-key only** (no private key, no generation, no signing/DH). secp256k1 has full sign+verify including the **JVM-compat verification** (raw verify → normalize-s verify → bitcoin-transcode verify) pinned by a 50-vector suite, preserved for cross-node (JVM PRISM) compatibility. **Gaps vs KMP**: no derivation (BIP32/SLIP-0010/BIP39), no SecureRandom API, no ed→x25519 conversion, no P-256 (secp256r1), no PBKDF2. neoprism's `did-prism` consumes this crate for SSI operation key encoding + JWK + SHA256 hashing.

`sdk-rust`'s `identus-core` (from `add-core-error-conventions`) provides the redaction-safe `IdentusError` (`&'static str` only; `Display` renders `"{code}: {public_message}"`), `ErrorCode`, `ErrorKind` (incl. `Crypto`), `CapabilityId`, `IdentusResult`, and documents a two-surface bridging convention (`to_identus_error()` / `parse_with_core_error()`) that **no crate has adopted yet**.

`crate-ring-layout` places `identus-crypto` in the domain-primitives layer, depending only on `identus-core`, enforced by the conformance guard.

## Goals / Non-Goals

**Goals:**
- Make `identus-crypto` the canonical Rust reference implementation that TS/Swift/KMP bindings are ported *from*.
- Port neoprism's Rust crypto in, completed to full sign/verify/generate for all curves (Ed25519, X25519, secp256k1, secp256r1).
- Cover the full KMP `apollo` capability surface: JWK, hashing, encodings, secure random, derivation (BIP32/SLIP-0010/BIP39), ed→x25519 conversion, P-256.
- Be the first adopter of the `core-error-conventions` two-surface error-bridging pattern.
- Preserve the secp256k1 JVM-compat verification contract (cross-node compatibility).
- Move the entropy adapter to the outer-boundary (`identus-adapters-entropy`), keeping `identus-crypto` a wasm-clean domain crate that defines only the `SecureRandom` port and takes it via dependency injection.

**Non-Goals:**
- Migrating neoprism to depend on `identus-crypto`. That is a separate future change; this change only ports *in* and does not touch neoprism.
- The dependency-declaration convention. That is `add-workspace-dependency-conventions`, assumed landed first. The `deny.toml` policy is `nix-tooling`'s current configuration; any tightening is a separate future change.
- BBS+ or any credential-specific crypto. Not in the KMP `apollo` surface; belongs to a credentials-layer change if needed.
- A FFI/UniFFI binding surface. That is `add-bindings` (outer-boundary layer); this change only produces the Rust API.
- The `getrandom`-backed wasm `SecureRandom` adapter and the `wasm` cargo feature. These are deferred to a **future TypeScript/wasm binding change** (distinct from `add-bindings`, which targets native FFI/UniFFI for Swift/KMP). They land in `identus-adapters-entropy` (the outer-boundary adapter crate created by `add-adapter-crate-layer`), alongside the `ring` adapter; `SecureRandom` remains a port in `identus-crypto` because that second backend is known to be needed.

## Decisions

### Decision 1: Port the source in — `identus-crypto` owns the Rust source (canonical home)

`identus-crypto` owns the Rust crypto source, ported from neoprism's `lib/apollo` and completed. neoprism's crate is the *source of the port*, **not** a dependency. The SDK's Rust crypto crate becomes the canonical reference that TS/Swift/KMP bindings port from.

**Rationale**: the user's framing — "a new canonical crypto capability for the identus sdk where the sdk will be ported/binding to different platform" — requires the SDK crate to *be* the reference, not a consumer of another repo's crate. Depending on neoprism would make neoprism the canonical home and leak its `derive_more` error model; the SDK would not be the port source for bindings. Owning the source gives full control of the error model and the binding surface, at the cost of duplication until a future change migrates neoprism.

**Alternatives considered**:
- *Depend on `identus-apollo` as an external path dep*: rejected; neoprism keeps owning the impl, its `derive_more` error model leaks, and the SDK is not the canonical source.
- *Port in + immediately migrate neoprism in this change*: rejected; scope creep across two repos. Migration is its own change.

### Decision 2: Two-surface error bridging — crypto is the first adopter

`crypto::Error` is a rich, idiomatic (thiserror-style) enum carrying runtime detail for logs — e.g. `InvalidKeySize { expected: usize, actual: usize, key_type: &'static str }`, `KeyParsing { source }`. `to_identus_error()` maps each variant to a redaction-safe `IdentusError` with a stable `ErrorCode` (catalogue below), `ErrorKind::Crypto` (or `InvalidInput`/`VerificationFailed` where semantically apt), `CapabilityId("crypto")`, and a `&'static str` `public_message` carrying **no** runtime detail. `IdentusError`'s `Display` renders only `"{code}: {public_message}"`, so `actual`/`expected`/`key_type` never cross the surface.

Stable `ErrorCode` catalogue (matched by conformance fixtures and bindings):
- `crypto.invalid_key_size`
- `crypto.key_parsing`
- `crypto.signature_invalid` (`ErrorKind::VerificationFailed`)
- `crypto.unsupported_curve`
- `crypto.derivation_failed`
- `crypto.mnemonic_invalid`
- `crypto.secure_random_failure`

**Rationale**: `IdentusError` carries only `&'static str` (structural redaction safety, per `core-error-conventions`); runtime-derived values like `actual: usize` *cannot* be stored in it. The two-surface pattern keeps the Rust-idiomatic API ergonomic (rich errors for local debugging) while the cross-language surface stays stable and leak-free. `crypto` is the documented "first adopter" of the convention.

**Alternatives considered**:
- *Drop the rich local error, expose only `IdentusError`*: rejected; loses useful debugging detail and ergonomic `?`-propagation of source errors.
- *Store runtime detail in `IdentusError`*: impossible by construction (`&'static str` only); would violate `core-error-conventions`.

### Decision 3: Feature-gated, default = all features

Cargo features per algorithm (`ed25519`,`x25519`,`secp256k1`,`secp256r1`,`hash`,`hex`,`base64`,`jwk`,`derivation`), mirroring neoprism's gating style. `default = ["ed25519","x25519","secp256k1","secp256r1","hash","hex","base64","jwk","derivation"]` — all on. There is **no `securerandom` feature**: the `SecureRandom` port trait is zero-dependency and is always available (it is referenced by every curve's `generate` and by `MnemonicHelper::create_random_mnemonics`). The `ring` adapter and any `wasm`/`getrandom` feature live in `identus-adapters-entropy`, not here.

**Rationale**: matches neoprism's gating (familiar, lets a minimal binding opt out), while all-on-by-default means the canonical build and every binding's default path compile the full surface — avoiding the feature-matrix-drift risk (the default *is* the whole capability). Because `ring` no longer lives in this crate, the all-on default is now also **wasm-safe** (the earlier wasm-tension between "default = all" and "crypto builds on wasm32" is resolved — see Decision 8).

**Alternatives considered**:
- *Always-on, no features*: rejected; loses the opt-out path for a minimal binding.
- *Feature-gated, default = minimal*: rejected; bindings would diverge on which features they enable.

### Decision 4: Preserve secp256k1 JVM-compat verification as a contract

`secp256k1::PublicKey::verify` SHALL try, in order: raw verify → normalize-s verify → bitcoin-transcode verify, returning `true` if any succeeds. Ported verbatim from neoprism (which ports it from the JVM `apollo` `Secp256k1Lib`). Pinned by the 50-vector JVM-compat suite ported from neoprism's `secp256k1_compat.rs`.

**Rationale**: signatures produced by the JVM PRISM node (bouncycastle/bitcoinj) are not always verifiable by a vanilla `k256` verify; the normalize-s + bitcoin-transcode fallbacks are a cross-node compatibility contract. Dropping it would break verification of JVM-signed PRISM operations.

**Alternatives considered**:
- *Vanilla `k256` verify only*: rejected; breaks JVM-signed-operation verification.

### Decision 5: Complete the sign/generate side for all curves — with injected entropy

neoprism exposed `PublicKey`-only for Ed25519/X25519. The canonical crate exposes `PublicKey` + `PrivateKey` + `KeyPair` for **all** curves (Ed25519, X25519, secp256k1, secp256r1), with `generate` / `sign` / `verify` (and `derive_shared` for X25519 DH), so bindings get a symmetric surface across curves. Because the `SecureRandom` adapter lives in the outer-boundary `identus-adapters-entropy` crate (Decision 8), the domain crate cannot reach a default entropy source, so `generate` SHALL take an injected `&mut impl SecureRandom` (`Ed25519KeyPair::generate(rng: &mut impl SecureRandom)`); `MnemonicHelper::create_random_mnemonics(rng)` likewise. `sign`/`verify`/`derive_shared`/`create_seed` are entropy-free and keep their natural signatures.

**Rationale**: a canonical capability that bindings port from must expose the full lifecycle; verify-only is a resolver's concern, not an SDK's. Injection (rather than a zero-arg `generate()`) is the hexagonal shape: the domain depends on the *port*, the composition root injects the *adapter*. It also matches the port origin — KMP `apollo`'s `SecureRandomInterface` is already injected into `KeyGenerator`/`MnemonicHelper`, not held as a global default.

**Alternatives considered**:
- *Port neoprism verbatim (verify-only Ed/X)*: rejected; an SDK cannot only verify.
- *Zero-arg `generate()` with a co-located `ring` default*: rejected; that couples the domain to `ring`'s wasm limitation and puts an adapter in the domain (see Decision 8).
- *A global/thread-local default `SecureRandom` registry*: rejected; hidden mutability is a poor default for a crypto crate; explicit injection is testable and wasm-friendly.

### Decision 6: Dependency choice — audited Rust crates

External crates chosen for audit posture: `ed25519-dalek` 2, `k256` 0.13, `p256` 0.13 (RustCrypto), `x25519-dalek` 2 (dalek-rs), `sha2` 0.10 + `hmac` 0.12 + `pbkdf2` 0.12 (RustCrypto), `base64` 0.22, `hex` 0.4. These are the most-audited Rust crypto crates; `rust-audit` catches known RustSec advisories in them. **`ring` is not an `identus-crypto` dependency** — the `ring`-backed entropy adapter lives in `identus-adapters-entropy` (outer-boundary), declared there behind a `ring` feature; the workspace-level `ring` entry is added by `add-adapter-crate-layer`. Hashing and derivation use pure `sha2`/`hmac`/`pbkdf2` (wasm-safe, no `ring`). A `getrandom` backend for the `SecureRandom` wasm adapter is **not** added in this change — it is deferred to the future TS/wasm binding change, landing in `identus-adapters-entropy`.

**Rationale**: a crypto crate's supply chain is part of its trust model; choosing audited crates is the first layer, and the existing `cargo-deny` + `rust-audit` checks (per `add-nix-tooling`) are the second. Keeping `ring` out of the domain keeps the domain's supply chain wasm-clean.

**Rationale**: a crypto crate's supply chain is part of its trust model; choosing audited crates is the first layer, and the existing `cargo-deny` + `rust-audit` checks (per `add-nix-tooling`) are the second.

### Decision 7: Idiomatic Rust snake_case API names

The public Rust method names follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/naming.html) (snake_case for methods/functions, PascalCase for types), **not** the KMP Kotlin source names. So `derive_child`, `init_from_seed`, `create_random_mnemonics`, `create_seed`, `is_valid_mnemonic_code`; type names (`HDKey`, `EdHDKey`, `MnemonicHelper`, `CurvePoint`, `KeyPair`) stay PascalCase. The KMP `derivation.HDKey` / `derivation.EdHDKey` / `MnemonicHelper` remain the *port origin* (referenced in the spec), but the Rust API surface is idiomatic.

**Rationale**: the actual Rust port precedent — neoprism's `lib/apollo` — already uses snake_case (`from_slice`, `encode_compressed`, `to_public_key`, `sign`) with no `#![allow(non_snake_case)]`. Reusing Kotlin's camelCase would (a) contradict that precedent, (b) force a crate-level `#![allow(non_snake_case)]` that masks accidental camelCase in unrelated future code, and (c) fail task 5.3's `cargo clippy -- -D warnings` gate, since `non_snake_case` is warn-by-default. "Canonical reference that bindings are ported *from*" does not require lexical name identity: every binding (TS/Swift/KMP) renames at the port boundary to its own conventions regardless, as it already does for every other Rust→binding edge. `clippy -D warnings` is the regression guard — no allow-list is needed or permitted.

**Alternatives considered**:
- *KMP-aligned camelCase + `#![allow(non_snake_case)]`*: rejected; non-idiomatic, contradicts the neoprism Rust precedent, and the blanket allow masks future naming bugs.

### Decision 8: Hexagonal scope at the primitive layer — `SecureRandom` is a port in crypto; the `ring` adapter lives in `identus-adapters-entropy`

`identus-crypto` owns **primitive crypto operations on key material** (bytes-in, bytes-out), neoprism-`apollo`-style: concrete curve types (`Ed25519PublicKey` wrapping `ed25519-dalek`, etc.) plus the encoding/verification trait polymorphism (`EncodeVec`/`EncodeArray`/`Verifiable`/`EncodeJwk`) ported from neoprism. The only infrastructure **port** is `SecureRandom` (entropy): `ring` breaks on `wasm32-unknown-unknown`, and a `getrandom`-backed wasm adapter is a **known second backend** (needed by the future TypeScript/wasm binding) — so `SecureRandom` is a port **defined in `identus-crypto`**, but its **adapters live in the outer-boundary `identus-adapters-entropy` crate** (created by the consumed `add-adapter-crate-layer` change). This change fills the `ring`-backed adapter into that crate; the `getrandom` adapter is deferred to the future binding change (also in `identus-adapters-entropy`). Curve operations, hashing, derivation, and conversion stay **concrete**: `ed25519-dalek`/`k256`/`p256`/`x25519-dalek`/`sha2`/`hmac`/`pbkdf2` have no known second backend, so a `Signer`/`Verifier`/`KeyGenerator` port would have exactly one adapter and document nothing. Hashing uses pure `sha2` (wasm-safe), so no `Digest` port is needed. Because `ring` lives in the outer-boundary adapter crate, `identus-crypto` builds on `wasm32` with **default features** — the wasm limitation is confined to the adapter crate, exactly where the hexagonal ring says platform-specific infrastructure belongs.

**Rationale**: a port is justified when there *is*, or is *known to be needed*, more than one backend — even if only one adapter ships today; a single-adapter port with **no known second backend** is the ceremony to avoid. `SecureRandom` qualifies because `getrandom`/wasm entropy is a known second backend (the future TS binding needs it). The **adapter's home** is the outer-boundary layer (per `crate-ring-layout` / `add-adapter-crate-layer`): the domain defines the port and consumes it via dependency injection; the composition root (binary/binding) wires a concrete adapter. Keeping `ring` out of the domain means the domain crate's portability is not reduced by an adapter's platform limitation — the precise property hexagonal ports exist to preserve.

**Resolved during apply — the `std`-feature tradeoff.** To keep `identus-crypto` wasm-clean by *default* (Decision 8's core claim), the workspace entries for `k256`, `p256`, and `ed25519-dalek` use `default-features = false` and intentionally do **not** enable their `std` feature: those crates' `std` features pull `rand_core/std`, which pulls `getrandom` — and `getrandom` does not compile on `wasm32-unknown-unknown` without its `js` feature (deferred to `identus-adapters-entropy`). Consequence: `elliptic_curve::Error` and `ed25519_dalek::SignatureError` do not implement `std::error::Error` without `std`, so `identus-crypto` wraps them in small local error newtypes (`EcKeyError`, `EdSignatureError`) that implement `std::error::Error` themselves (the underlying types impl `Debug` + `Display` without `std`). This keeps the rich local `crypto::Error` ergonomic while the crate builds on `wasm32` with default features (verified: `cargo build -p identus-crypto --target wasm32-unknown-unknown` succeeds). `x25519-dalek` keeps its defaults plus `static_secrets` (it does not pull `getrandom` via `std`).

**Alternatives considered**:
- *Co-locate the `ring` adapter inside `identus-crypto` (the original `add-crypto-capability` stance)*: rejected; it couples the domain crate to `ring`'s `wasm32` limitation (a full default-feature build on `wasm32` would not build), sets the precedent that adapters live in domain crates, and leaves the most hexagonally-faithful alternative (port in crypto, adapter in `identus-adapters-entropy`) unconsidered. This decision adopts that alternative.
- *Operation ports for every primitive (`Signer`/`Verifier`/`KeyGenerator` + a `SoftwareAdapter`)*: rejected; one adapter per port and no known second backend, pure ceremony. (This "structural" stance is reserved for the key-management layer, not primitive crypto.)
- *A `Digest` port for SHA-2*: rejected; routing hashing through `ring` is unnecessary — pure `sha2` builds on `wasm32`, and there is no known second backend to abstract.
- *Drop `SecureRandom` as a port too, hard-code `ring`*: rejected; `ring` breaks on `wasm32`, so a `getrandom` wasm backend is *known to be needed* — the port is load-bearing even while only the `ring` adapter ships now.

### Decision 9: Key management is out of scope — belongs in `identus-wallet`

This change does **not** define `KeyHandle`, `KeyStore`, `SecretResolver`, non-exportable signing, or hardware/`KMS`-bound signers (the seed's `crypto` doc-comment called these "signer ports, and hardware/`KMS` adapter traits"). That remit is **key management**, not primitive crypto: it abstracts *where the key lives* (in RAM vs an HSM vs a Secure Enclave), which is an orchestration concern the seed places in `identus-wallet` (the `KeyHandle`/`KeyStore`/`SecretResolver`/`EntropySource` ports) with platform adapters in `identus-adapters` (per `adr-secure-storage.md`: "the storage boundary is owned by `identus-wallet` and `identus-adapters`"). Primitive crypto operates on key *material* (bytes); key management decides whether it *has* material (software → calls crypto primitives) or a device reference (HSM → the adapter talks to the device directly, crypto not involved).

**Rationale**: keeping the two seams separate avoids baking key-identity into primitive-crypto ports (a bound-to-key `Signer` would force every binding into a `signer.sign(msg)` indirection with no primitive-level payoff), and matches the seed's two-tier split. The `crates/crypto/src/lib.rs` doc-comment is reconciled at apply time to drop "hardware/`KMS` adapter traits" from crypto's remit.

**Alternatives considered**:
- *Fold key management into this change (define `KeyHandle`/`KeyStore` here)*: rejected; cross-crate scope creep, and the seed already drafts those ports in `identus-wallet`. A separate `add-wallet-key-management` change lands them.
- *A bound-to-key `Signer` port in crypto (Model B)*: rejected; that's the key-management seam wearing a primitive-crypto disguise — it belongs with `KeyHandle` in wallet.

## Risks / Trade-offs

- **[Risk] `ring` does not build on `wasm32-unknown-unknown`** → it lives in the outer-boundary `identus-adapters-entropy` crate (behind a `ring` feature, `default = []`), **not** in `identus-crypto`; hashing uses pure `sha2` (wasm-safe). Consequence: `identus-crypto` builds on `wasm32` with **default features** (no `ring` dependency); the wasm limitation is confined to the adapter crate, and a wasm consumer uses the future `getrandom` adapter (deferred to the TS/wasm binding change) instead. The devshell already provides the `wasm32-unknown-unknown` target.
- **[Trade-off] `generate(rng)`/`create_random_mnemonics(rng)` take an injected `SecureRandom`** → less ergonomic than a zero-arg `generate()`, and every test must construct a (deterministic) rng; accepted as the cost of keeping the entropy adapter out of the domain and keeping crypto wasm-clean by default. It matches KMP `apollo`'s injected `SecureRandomInterface`.
- **[Risk] P-256 is new (not in neoprism's port)** → tested against known P-256 test vectors; `p256` crate is RustCrypto-audited.
- **[Trade-off] duplication with neoprism until migration** → accepted; migration is a separate change.
- **[Risk] duplicate versions may surface transitively among the crypto deps** → under the current `multiple-versions = "warn"` policy (per `add-nix-tooling`) this is a warning, not a failure; if a future supply-chain change tightens to `deny`, a scoped `[bans] skip` with justification is the documented escape hatch.
- **[Trade-off] the rich `crypto::Error` is a second error type to maintain** → accepted; it's the documented cost of the two-surface pattern.

## Migration Plan

0. **Assumes `add-adapter-crate-layer` landed**: `identus-adapters-entropy` skeleton exists (crate + `ring` workspace entry + `LAYER_RULES` membership), and the ring-layout admits adapter-family crates. This change fills the `ring`-backed `SecureRandom` adapter into that crate.
1. Land the foundation/encoding primitives + error-bridging (Phase 1) behind the feature map.
2. Land the core curves with full sign/verify/generate (injected `SecureRandom`) + JVM-compat suite (Phase 2).
3. Land derivation, the `SecureRandom` port (crypto) + the `ring` adapter (in `identus-adapters-entropy`), conversion, P-256 (Phase 3).
4. Conformance + hardening (Phase 4): error-bridging tests, layer guard green, fmt/clippy/test/nix flake check, wasm-clean assertion.
5. Merge to `main` after `add-workspace-dependency-conventions` and `add-adapter-crate-layer`.

**Rollback**: the change is additive to a stub; reverting returns `identus-crypto` to its `COMPONENT`-only stub and empties the `identus-adapters-entropy` `ring` adapter. No other crate depends on crypto's new content yet at merge time (downstream crates adopt it in their own changes).

## Open Questions

_All resolved during apply._

- **`default-features` per dep** — **resolved.** `k256`/`p256` use `default-features = false` with `features = ["arithmetic", "ecdsa"]` (no `std`, to stay wasm-clean per Decision 8's apply note). `ed25519-dalek` uses `default-features = false` with `features = ["fast", "zeroize"]`. `x25519-dalek` keeps its defaults plus `static_secrets`. `sha2`/`hmac`/`pbkdf2`/`base64`/`hex` keep defaults (`pbkdf2` enables `simple`). All declared at workspace level per `workspace-dependency-conventions`.
- **Mnemonic wordlist scope** — **resolved.** English-only now (2048 words embedded as a `const &[&str]` in `derivation/wordlist.rs`); other KMP wordlists (9 languages) deferred to a future change.
- **`SecureRandom` wasm backend** — **resolved (recorded as Decision 8).** `SecureRandom` is a port in `identus-crypto`; the `ring` adapter lives in `identus-adapters-entropy` (filled here); the `getrandom`-backed wasm adapter + `wasm` feature land in `identus-adapters-entropy` in the future TS/wasm binding change (distinct from `add-bindings`/UniFFI). No spike required — pure `sha2` is wasm-safe.

### Divergences from the KMP port origin (resolved during apply)

Two ports are labeled "ported from KMP" but the KMP port origin deviates from the published standard the spec's test-vector scenarios require. In both cases the spec's authoritative algorithm + published-vector scenario wins; the KMP attribution is the error, recorded here.

- **`EdHDKey` (SLIP-0010 vs ed25519-bip32).** The spec/task labels this SLIP-0010, and the spec scenario requires matching published SLIP-0010 ed25519 test vectors. The KMP `derivation.EdHDKey` actually uses the **ed25519-bip32 (Khovratovich)** algorithm via a native uniffi wrapper (64-byte extended keys; `init_from_seed` splits a 64-byte seed into key+chain without an HMAC master step). `identus-crypto` implements **SLIP-0010** (32-byte keys; `init_from_seed` performs the SLIP-0010 master step HMAC-SHA512 keyed `"ed25519 seed"`; hardened-only child derivation where the child key IS the IL half). The implementation matches the published SLIP-0010 ed25519 test vectors (seed `000102...0e0f` → master key `2b4be7f1...`, `m/0'` → `68e0fe46...`). A future change may add the ed25519-bip32 algorithm alongside SLIP-0010 if JVM/PRISM compatibility requires it.
- **`MnemonicHelper::create_seed` (standard BIP39 salt).** The spec scenario requires matching published BIP39 test vectors. The KMP `MnemonicHelper.create_seed` uses the passphrase directly as the PBKDF2 salt (non-standard; omits the `"mnemonic"` prefix). `identus-crypto` uses the **standard BIP39** salt `"mnemonic" + passphrase` (PBKDF2-HMAC-SHA512, 2048 iterations, 64-byte dk), matching the published BIP39 test vectors (`abandon ... about` / `TREZOR` → seed `c55257c3...`).