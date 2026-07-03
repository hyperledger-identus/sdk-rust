# wallet-key-management

> **Status: Exploring** — not yet fleshed out. This is a pre-change exploration;
> it is not tracked by `openspec` as a change or spec, and is not apply-able.
> When ready, promote to `openspec/changes/` via `openspec new change`.

## Why

`add-crypto-capability` deliberately scopes `identus-crypto` to **primitive
crypto operations on key material** (bytes-in, bytes-out, neoprism-`apollo`-style),
with `SecureRandom` as its only infrastructure port. Its Decision 9 records that
**key management** — `KeyHandle`, `KeyStore`, `SecretResolver`, non-exportable
signing, and hardware/`KMS`-bound signers — is out of scope for crypto and
belongs in `identus-wallet`. That deferral creates a recorded architectural gap:
until a wallet-layer change lands, the workspace has primitive crypto but no
port against which the key-management adapters (Secure Enclave, Android
Keystore, HSM/KMS, WebCrypto, OS keychain) can be written.

This exploration captures the shape of that deferred layer so the thinking
isn't lost — it is **not** a change proposal yet. Several structural forks
remain open (see Open Questions) and the seed's drafts are placeholders, so
promoting prematurely would lock in answers we haven't earned.

## What (sketch)

The seed (`sdk-rust-seed` branch) already drafts the seam across two crates.
The change would flesh out and reconcile those drafts.

### The two-tier hexagonal split (crypto ↔ wallet)

```
   identus-crypto (domain-primitives)          identus-wallet (orchestration)
   ─────────────────────────────────          ──────────────────────────────
   PRIMITIVE OPS on key MATERIAL (bytes):     KEY-MANAGEMENT seam (Depth 3):
     Ed25519/X25519/secp256k1/P-256 sign/      KeyHandle { id, purpose, exportable }
       verify/generate  (concrete)             ResolvedSecret { key_id, purpose }
     derivation, conversion, JWK, hashing       (descriptor, NOT raw bytes)
     SecureRandom  (the only infra PORT)       trait KeyStore {
                                                   generate_key(...)->KeyHandle
                                                   import_test_key(...,exportable)
                                                   sign(handle,msg)->Signature
                                                   delete_key(...) }
                                                trait SecretResolver {
                                                   resolve_secret(...)->ResolvedSecret }
                                                trait SecureStore { put/get/delete/... }
                                                trait BackupStore { export/import_snapshot }
                                                trait EntropySource { fill }

   consumed BY wallet when it HAS bytes      ↑ builds ON TOP of crypto's primitives:
   (software keys → call crypto.sign).        software key → wallet resolves handle to
                                              bytes internally → calls crypto sign.
                                              HSM key → wallet adapter talks to the
                                              device directly; crypto not involved.
                          ↑
                          │ implement both tiers' ports
                          ▼
   identus-adapters (outer-boundary):
     InMemorySecureStorageAdapter  (deterministic, test-only — seed's draft)
     [future] Secure Enclave / Android Keystore / HSM-KMS / WebCrypto / OS keychain
```

### What the change would do (sketch)

- Promote the seed's `identus-wallet` ports (`KeyStore`, `SecretResolver`,
  `SecureStore`, `BackupStore`, `EntropySource`) and value types (`KeyHandle`,
  `ResolvedSecret`, `SecretClass`, `StorageRecord`, `BackupSnapshot`,
  `Signature`) from stubs to a real, redaction-safe port layer.
- Define the **non-exportable key contract** per `adr-secure-storage.md`:
  raw private keys must not cross public Rust/UniFFI/WASM/Swift/Kotlin
  boundaries unless the adapter marks the key `exportable` and the caller
  requests explicit export. `SecretResolver` returns descriptors, not bytes.
- Wire real signing: replace the seed's dummy `InMemorySecureStorageAdapter::
  sign` (which returns `vec![accumulator; 64]`, not real Ed25519) with a
  software adapter that resolves a handle to bytes and delegates to
  `identus-crypto`'s primitive operations.
- Land the platform adapter *traits* (and likely a first in-memory + a
  SQLite adapter) in `identus-adapters`; Secure Enclave / Keystore / HSM /
  KMS / WebCrypto adapters are feature-gated and may land incrementally.

## Open Questions (to resolve before promoting to a change)

- **Signer-port shape (Model B vs wallet-resolves-then-calls-crypto).** Does
  the wallet layer need a bound-to-key `Signer` *port* (an `HsmSigner {
  device_ref }` instance implements `Signer`), or does `KeyStore::sign(handle,
  msg)` own the dispatch and call `identus-crypto` primitives only when it
  holds bytes? The former bakes key-identity into a port; the latter keeps
  primitive crypto out of the key-management abstraction. Lean: the latter —
  crypto stays bytes-only; the HSM adapter implements `KeyStore::sign` by
  talking to the device, never touching crypto. Confirm before scoping.
- **`EntropySource` (wallet) vs `SecureRandom` (crypto) — reconcile or bridge?**
  The seed drafts `EntropySource` in `identus-wallet` (injected into
  `KeyStore::generate_key`) while `add-crypto-capability` puts `SecureRandom`
  in `identus-crypto`. Same concept, two homes. Options: (a) wallet's
  `EntropySource` is a re-export/alias of crypto's `SecureRandom`; (b) wallet
  defines `EntropySource` and crypto's `SecureRandom` adapts it; (c) one is
  dropped. Needs a single-source decision.
- **Derivation for non-exportable keys (capability profiles).** BIP32/SLIP-0010
  derivation needs the parent private key *bytes*; an HSM that won't export
  them cannot do BIP32 in-software. Does the change (a) bound software-side
  derivation to exportable keys and document a capability profile per key
  backing, (b) make `Derivation` a port the HSM may-not-implement, or (c)
  restrict the wallet derivation surface to seeds/`RecoveryMaterial` feeding
  `KeyStore::generate_key` (the seed's `SecretClass::RecoveryMaterial` hint)?
  This is a real HSM limitation, not a design bug — but the choice shapes the
  API.
- **Scope: wallet-only vs wallet + adapters in one change.** Does this change
  land only the wallet ports (+ in-memory/SQLite adapters in `identus-adapters`),
  or also the first hardware adapters? Lean: wallet ports + deterministic +
  in-memory + SQLite; hardware adapters are follow-on changes.
- **Sequencing vs `add-crypto-capability`.** This change consumes crypto's
  primitive operations, so it lands after. Confirm there's no intermediate
  consumer that needs the ports sooner.
- **`SecretClass` / `StorageRecord` metadata surface.** The seed drafts a rich
  metadata model (`namespace`, `owner_crate`, `secret_class`, `version`,
  `encryption_profile`, `migration_label`). Confirm whether to port it
  verbatim or trim before the port layer stabilizes (once wrappers port from
  it, it's hard to change).

## Notes

- Grounded in the `sdk-rust-seed` branch: `crates/wallet/src/lib.rs` (the
  port drafts + value types), `crates/adapters/src/lib.rs` (the dummy
  `InMemorySecureStorageAdapter` + `DeterministicEntropy`), and
  `docs/architecture/adr-secure-storage.md` + `adr-crate-layout.md` (the
  recorded two-tier intent).
- The seed's `crypto` doc-comment over-claimed "signer ports, and
  hardware/`KMS` adapter traits" as crypto's remit; `add-crypto-capability`
  Decision 9 moves that remit to wallet. This change is where those ports
  actually land.
- `adr-secure-storage.md` already records the decision ("the storage boundary
  is owned by `identus-wallet` and `identus-adapters`") and the platform
  adapter table (Browser/Node/iOS/Android/JVM/Server) — this change is the
  execution of that ADR, not a new decision.
- Dependency direction is already ring-legal: `identus-adapters` →
  `identus-wallet` → `identus-crypto` is inward; `identus-conformance`'s
  guard already enforces it.
- Not ready to be a change: the Signer-port-shape and EntropySource-reconcile
  forks (above) materially change the API surface and shouldn't be guessed.
  Revisit after `add-crypto-capability` lands its `SecureRandom` port, since
  that crystallizes the entropy-port home question.