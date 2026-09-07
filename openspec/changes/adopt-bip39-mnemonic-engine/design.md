## Context

The existing helper correctly reproduces standard seed vectors for ASCII input
and Apollo's legacy salt variant, but validation checks only word membership.
It therefore accepts zero words, unsupported word counts, and checksum-invalid
sentences. Its hand-written entropy encoder accepts any non-empty byte length
divisible by four rather than BIP-39's five 128–256-bit sizes, and neither the
mnemonic nor standard passphrase is NFKD-normalized.

The exact `bip39 2.2.2` crates.io artifact comes from signed tag `v2.2.2` at
`rust-bitcoin/rust-bip39@d6dbc31678cc507c8cae62b3a059b0b48e866436`.
The direct crate is `no_std`, has no unsafe block, and supports validation,
entropy conversion, normalization and PBKDF2. With default features disabled
and `alloc,zeroize` enabled, the standalone cone is 13 packages and the
incremental workspace cone is seven packages. The dependency's `Mnemonic`
implements word-revealing `Debug`/`Display`, and its convenience `to_seed`
may own a normalized passphrase in an ordinary `String`; neither behavior is
suitable as the SDK boundary.

## Goals / Non-Goals

**Goals:**

- Enforce BIP-39 English entropy, word-count, checksum and NFKD rules.
- Keep injected randomness, stable Identus errors and existing public method
  signatures.
- Keep mnemonic/passphrase/seed intermediates zeroizing where the SDK owns
  them and keep dependency formatters unreachable to consumers.
- Preserve Apollo KMP import behavior without confusing it with standard seed
  derivation.
- Remove duplicated closed-standard mechanics and fixtures only after stronger
  conformance evidence exists.

**Non-Goals:**

- Add other languages, UI recovery workflows, custody/storage, FFI, serde,
  seed persistence, BIP-32 policy, publication or downstream migration.
- Expose `bip39::Mnemonic`, dependency errors, word-count enums, RNG traits or
  formatting behavior.
- Normalize the legacy KMP passphrase and thereby change Apollo-compatible
  bytes without a separate migration decision.

## Decisions

### Decision 1: preserve the Identus facade and strengthen invalid behavior

`MnemonicHelper` and its current signatures remain public. `wordlist()` is
backed by `Language::English.word_list()` and remains an owned vector of static
strings. `to_mnemonic_code` remains infallible for compatibility: exact
128/160/192/224/256-bit entropy returns 12/15/18/21/24 words and every other
length returns an empty vector. The fallible random APIs continue to request
exactly 32 bytes through `SecureRandom` and therefore always produce 24 words.

Validation joins the caller's words into a zeroizing string, NFKD-normalizes
that string, and parses it in the explicit English language. Unsupported word
counts, unknown words and invalid checksums all collapse to the existing
redacted `Error::MnemonicInvalid`; dependency errors never escape.

### Decision 2: use a zeroizing normalization adapter

The implementation calls `Mnemonic::normalize_utf8_cow` but does not call the
dependency's convenience `to_seed`. Borrowed NFKD input is used without a new
allocation. When normalization creates an owned `String`, the SDK immediately
moves it into `Zeroizing<String>`. Standard derivation then calls
`to_seed_normalized`, wraps its `[u8; 64]` result in `Zeroizing`, and returns the
existing caller-owned `Vec<u8>` copy.

This keeps normalization semantics delegated to the focused crate while
closing its non-zeroizing owned-passphrase lifetime at the facade. The parsed
`Mnemonic` is also zeroized on drop by the selected dependency feature. No
dependency value is logged, formatted, serialized, returned or retained.

### Decision 3: isolate the KMP legacy salt

`create_seed_kmp` uses the same strict, NFKD-normalized English mnemonic parser
but retains the exact caller passphrase as PBKDF2 salt with no `"mnemonic"`
prefix and no passphrase normalization. This reproduces Apollo's legacy import
behavior. Local `pbkdf2` and `sha2` use remain compiled only for that opt-in
path; standard BIP-39 seed derivation is fully delegated.

`kmp-compat` explicitly depends on `derivation` and the legacy PBKDF2
dependency so feature-disabled builds do not carry the compatibility code.

### Decision 4: accept the narrow transitive Rust unsafe cone

The direct crate has zero lexical unsafe blocks and no native/build source.
The resolved new packages include Rust unsafe in `bitcoin_hashes`, `arrayvec`
and `unicode-normalization`; this is library implementation code, not a new SDK
unsafe block. Exact versions, checksums, licenses and target builds remain
locked and checked by Nix, RustSec and cargo-deny. No C, C++, assembly or
platform binding enters the graph.

## Risks / Trade-offs

- **Correctness tightening rejects formerly accepted input** → specify this as
  an intentional correctness migration and retain one stable error code.
- **Dependency types can reveal mnemonic words when formatted** → keep them
  transient and private and test generated public API plus SDK diagnostics.
- **Normalization may allocate secret text** → transfer every owned normalized
  string immediately into `Zeroizing` before derivation or error return.
- **KMP and standard passphrases intentionally differ** → keep separate methods,
  features, docs and byte vectors; never share the salt-construction branch.
- **Seven incremental packages add supply-chain surface** → pin exact direct
  version/features, record transitive unsafe and retain one-PR rollback.

## Migration Plan

1. Land issue-linked OpenSpec, focused ADR and exact dependency research.
2. Pass research and constraint readiness before production code.
3. Replace mechanics behind the existing facade and add positive, negative,
   Unicode, KMP and lifecycle conformance tests.
4. Prove public API, feature graph, MSRV/primary/portable targets and full Nix.
5. Archive the change and open a PR after baseline PR #180 merges green.

Rollback reverts the focused #152 PR. Persisted seed/mnemonic formats do not
change; only previously non-conformant input acceptance changes.

## Open Questions

None block implementation. Additional BIP-39 languages require a separate
product/API decision because language detection and localized wordlists are not
part of the current English-only contract.
