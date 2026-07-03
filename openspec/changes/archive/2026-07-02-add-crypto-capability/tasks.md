## 1. Workspace dependency entries + feature map (Phase 1 — foundation)

> **Assumes `add-adapter-crate-layer` landed**: `identus-adapters-entropy` skeleton exists (crate + `ring` workspace entry + `LAYER_RULES` membership) and the ring-layout admits adapter-family crates. This change fills that crate with the `ring` adapter (Phase 3) and defines the `SecureRandom` port in `identus-crypto`.

- [x] 1.1 Add external crate entries to root `Cargo.toml` `[workspace.dependencies]` (per `add-workspace-dependency-conventions`): `ed25519-dalek` 2, `k256` 0.13 (+`arithmetic`,`ecdsa`), `p256` 0.13 (+`arithmetic`,`ecdsa`), `x25519-dalek` 2, `sha2` 0.10, `hmac` 0.12, `pbkdf2` 0.12, `base64` 0.22, `hex` 0.4. (**No `ring`** — `ring` is added to `[workspace.dependencies]` by `add-adapter-crate-layer`; `identus-crypto` does not depend on `ring`. `getrandom` is **not** added in this change — the `getrandom`-backed wasm `SecureRandom` adapter is deferred to the future TS/wasm binding change, landing in `identus-adapters-entropy`.)
- [x] 1.2 Update `crates/crypto/Cargo.toml`: reference each via `<dep>.workspace = true`; add feature map (`ed25519`,`x25519`,`secp256k1`,`secp256r1`,`hash`,`hex`,`base64`,`jwk`,`derivation`) with `default = all` (**no `securerandom` feature** — the `SecureRandom` port trait is zero-dependency and always available; **no `ring` dependency**). Map feature → dep edges (e.g. `ed25519 = ["jwk","dep:ed25519-dalek"]`). A `wasm` feature (and the `getrandom` dep) is **not** added in this change — both live in `identus-adapters-entropy`, deferred to the future TS/wasm binding change.
- [x] 1.3 Verify `cargo build -p identus-crypto` resolves the full default-feature graph (no `ring`); verify `cargo build -p identus-crypto --target wasm32-unknown-unknown` succeeds with default features

## 2. Error bridging + encoding primitives (Phase 1 cont.)

- [x] 2.1 `src/error.rs`: idiomatic `crypto::Error` (thiserror-style) with variants `InvalidKeySize{expected,actual,key_type}`, `KeyParsing{source}`, `SignatureInvalid`, `UnsupportedCurve`, `DerivationFailed`, `MnemonicInvalid`, `SecureRandomFailure`; `to_identus_error()` mapping to the stable `ErrorCode` catalogue + `CapabilityId("crypto")`; ensure `IdentusError` `Display` carries no runtime detail
- [x] 2.2 `src/enc.rs`: `EncodeVec`, `EncodeArray<const N>`, `Verifiable` traits (ported from neoprism)
- [x] 2.3 `src/base64.rs` + `src/hex.rs`: `Base64UrlStrNoPad`, `HexStr` (ported from neoprism)
- [x] 2.4 `src/hash.rs`: `Sha256Digest`, `Sha512Digest` (pure `sha2` — wasm-safe, no `ring`)
- [x] 2.5 `src/jwk.rs`: `Jwk` struct + `EncodeJwk` trait (ported from neoprism)
- [x] 2.6 `src/lib.rs`: module wiring; keep `COMPONENT`; re-exports; **reconcile the module doc-comment to the primitive-operations remit** (drop the seed's "signer ports, and hardware/`KMS` adapter traits" — key management is an `identus-wallet` concern per Decision 9); describe crypto as primitive operations on key material with `SecureRandom` as the single infra port

## 3. Core curves — full sign/verify/generate (Phase 2)

- [x] 3.1 `src/crypto/ed25519.rs`: `Ed25519PublicKey` + `Ed25519PrivateKey` + `Ed25519KeyPair` (`generate(rng: &mut impl SecureRandom)`, `sign`, `verify`, `EncodeVec`/`EncodeArray<32>`/`Verifiable`/`EncodeJwk`) — completes neoprism's verify-only port
- [x] 3.2 `src/crypto/x25519.rs`: `X25519PublicKey` + `X25519PrivateKey` + `X25519KeyPair` (`generate(rng: &mut impl SecureRandom)`, `derive_shared` DH, `EncodeVec`/`EncodeArray<32>`/`EncodeJwk`) — completes neoprism's pubkey-only port
- [x] 3.3 `src/crypto/secp256k1.rs`: `Secp256k1PublicKey` (compressed/uncompressed/`CurvePoint`) + `Secp256k1PrivateKey` + `Secp256k1KeyPair` (`generate(rng: &mut impl SecureRandom)`, `sign` DER, `verify` with JVM-compat normalize-s + bitcoin-transcode, `EncodeJwk`) — ported from neoprism
- [x] 3.4 `src/crypto/secp256r1.rs`: `P256PublicKey` + `P256PrivateKey` + `P256KeyPair` (`generate(rng: &mut impl SecureRandom)`, `sign`, `verify`, `EncodeJwk`) — new (parity gap vs neoprism)
- [x] 3.5 Tests: ed25519/x25519/secp256r1 unit tests (each `generate` injects a small in-test deterministic `SecureRandom` impl — `identus-crypto` cannot dev-depend on the outer-boundary adapter crate); secp256k1 unit tests + the 50-vector JVM-compat suite (ported from neoprism `secp256k1_compat.rs`); JWK roundtrip tests for all curves

## 4. Derivation, secure random, conversion (Phase 3)

- [x] 4.1 `src/securerandom.rs`: `SecureRandom` **port trait only** (the crate's only infra port), `generate_seed(n)` (port of KMP `SecureRandom`/`SecureRandomInterface`). **No `ring` impl in `identus-crypto`** — the `ring`-backed adapter lives in `identus-adapters-entropy` (see 4.1b). `SecureRandom` stays a port because a known second backend (`getrandom`/wasm) exists; the `getrandom`-backed wasm adapter is deferred to the future TS/wasm binding change, landing in `identus-adapters-entropy`.
- [x] 4.1b `crates/adapters-entropy/src/` (behind the `ring` feature created by `add-adapter-crate-layer`): implement the `ring`-backed `SecureRandom` adapter (`ring::rand::SystemRandom`) implementing `identus_crypto::SecureRandom` — this is the adapter that `*KeyPair::generate` / `MnemonicHelper::create_random_mnemonics` consume via injection at the composition root. (A deterministic test adapter may land behind a `deterministic` feature for cross-crate consumers.)
- [x] 4.2 `src/derivation/hdkey.rs`: BIP32 `HDKey` for secp256k1 — `derive(path)`, `derive_child`, hardened derivation, `init_from_seed` (port of KMP `derivation.HDKey`)
- [x] 4.3 `src/derivation/edhdkey.rs`: SLIP-0010 `EdHDKey` for ed25519 — `derive(path)`, `derive_child`, `init_from_seed` (port of KMP `derivation.EdHDKey`)
- [x] 4.4 `src/derivation/mnemonic.rs` + wordlist: BIP39 `MnemonicHelper` — `create_random_mnemonics(rng: &mut impl SecureRandom)`, `create_seed` (PBKDF2-SHA512), `is_valid_mnemonic_code`, English wordlist (port of KMP `MnemonicHelper`); `DerivationPath`/`DerivationAxis` helpers
- [x] 4.5 `src/convert.rs`: `ConvertEd25519` — ed25519 → x25519 private key conversion (SHA512 + clamp) (port of KMP `ConvertEd25519`)
- [x] 4.6 Tests: BIP32 known vectors, SLIP-0010 ed25519 known vectors, BIP39 known vectors (mnemonic creation injects a deterministic rng), ed→x25519 conversion vectors

## 5. Conformance + hardening (Phase 4)

- [x] 5.1 Error-bridging tests: every `crypto::Error` variant → `to_identus_error()` yields the catalogue `ErrorCode` + `CapabilityId("crypto")`; assert `IdentusError` `Display` contains no runtime detail (`actual`/`expected`/`key_type`)
- [x] 5.2 Layer guard green: `identus-crypto` depends only on `identus-core` (+ workspace-level external crates, which the guard treats as external per `add-workspace-dependency-conventions`); `crates/crypto/Cargo.toml` SHALL NOT list `ring` or any `identus-adapters-*` crate
- [x] 5.3 `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test --workspace` pass (API names are snake_case per design Decision 7; no `#![allow(non_snake_case)]` is permitted, so any camelCase regression fails this gate)
- [x] 5.4 `nix flake check` green under the existing `deny.toml` (per `add-nix-tooling`: `all-features = false`, `multiple-versions = "warn"`) and `cargo audit` via crane `cargoAudit`
- [x] 5.5 Record the wasm-backend decision in `design.md` (Decision 8 + Non-Goals): `SecureRandom` is a port in `identus-crypto`; the `ring` adapter lives in `identus-adapters-entropy` (filled here); the `getrandom`-backed wasm adapter + `wasm` feature land in `identus-adapters-entropy` in the future TS/wasm binding change (distinct from `add-bindings`/UniFFI). No spike required — the decision is recorded. (Hashing needs no spike — pure `sha2` is wasm-safe.)

## 6. OpenSpec artifacts

- [x] 6.1 Finalize `proposal.md`, `design.md`, `specs/crypto/spec.md`, `tasks.md`
- [x] 6.2 Resolve the Open Questions in `design.md` (`default-features` per dep, mnemonic wordlist scope) during apply — the `SecureRandom` wasm-backend question is now a recorded decision (port in `identus-crypto`; `ring` adapter in `identus-adapters-entropy`; `getrandom`/wasm adapter deferred there to the future TS/wasm binding change), not an open question
- [x] 6.3 Run `openspec validate add-crypto-capability --strict` and resolve any findings