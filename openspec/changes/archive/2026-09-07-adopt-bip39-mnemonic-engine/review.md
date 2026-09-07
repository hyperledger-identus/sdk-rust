# Exact-diff correctness and security review

- **Reviewer:** Codex fresh contract and implementation pass
- **Date:** 2026-09-08
- **Base:** `7c84621018eb5e9e9f68ab0abc57d60aa27b55b3`
  (`origin/develop` after issue #177 merged)
- **Scope:** issue #152, ADR 0079, complete OpenSpec contract, manifests,
  dependency lock, mnemonic facade, conformance tests, feature graphs and
  generated public API
- **Result:** no unresolved blocker; one intentional `0.0.x` public-constant
  removal and the remaining caller-owned secret lifetime are recorded

## Findings

1. **Verified — invalid inputs now fail closed.** The facade delegates entropy,
   word-count, word-membership and checksum rules to exact `bip39 2.2.2`.
   Unsupported entropy remains an empty vector through the existing infallible
   signature; invalid mnemonic input maps only to `Error::MnemonicInvalid`.
2. **Verified — standard and legacy behavior remain separate.** Standard seed
   derivation uses NFKD input and BIP-39's prefixed salt. `kmp-compat` shares
   strict mnemonic parsing but deliberately preserves Apollo's unnormalized,
   unprefixed passphrase bytes. Published, Unicode and donor vectors cover the
   deciding branches.
3. **Verified — secret intermediates have bounded ownership.** Injected entropy,
   joined mnemonic text, newly allocated normalized text, dependency mnemonic
   state and fixed seed output are zeroizing. The dependency's word-revealing
   `Debug`/`Display` and errors never cross the SDK boundary. Returned mnemonic
   and seed vectors remain caller-owned under the pre-existing API contract.
4. **Verified — dependency mechanics remain private.** `cargo-public-api 0.52.0`
   found no `bip39` or dependency type in the all-feature public API. The only
   API diff is removal of the duplicate
   `derivation::wordlist::ENGLISH_WORDLIST` module/constant; the supported
   `derivation::mnemonic::wordlist()` function and every `MnemonicHelper`
   signature are unchanged. This intentional migration is allowed by the
   documented `0.0.x` policy and is now explicit in the ADR and constraints.
5. **Verified — feature cohesion improved.** A no-default build contains neither
   BIP39 nor PBKDF2. `derivation` adds `bip39` without the SDK's PBKDF2 crate;
   only `kmp-compat` adds that legacy PBKDF2 dependency. Crate RNG, serde and
   multilingual features remain disabled.
6. **Accepted residual — seven pure-Rust packages enter the lock.** The exact
   dependency has no direct unsafe or native code, while recorded transitive
   crates contain bounded Rust unsafe. Audit and license/source policy pass;
   this evidence is a supply-chain signal, not a cryptographic audit.
7. **Verified — compatibility and portability are bounded.** All 24 published
   English vectors remain exact; the five entropy sizes, invalid counts and
   checksum, NFKD equivalence, Apollo KMP vectors and redacted errors pass.
   Rust 1.85, Rust 1.98, WASM, Android, iOS, docs, Clippy and full workspace
   tests pass through pinned Nix.
8. **Verified — scope and rollback are focused.** No BIP-32, custody, storage,
   recovery UI, multilingual policy, bindings, consumer repository, release or
   publication behavior changed. Reverting the issue #152 PR restores the
   previous implementation and lock graph without persisted-data migration.

## Decision

The implementation satisfies issue #152 and ADR 0079. The explicitly recorded
`0.0.x` constant migration and caller-owned output lifetime do not block this
bounded adoption. The change is approved for exact-head validation, guarded
OpenSpec archive and an issue-linked PR to `develop`.
