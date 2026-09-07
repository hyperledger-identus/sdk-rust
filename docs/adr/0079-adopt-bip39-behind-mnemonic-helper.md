# ADR 0079: adopt bip39 behind MnemonicHelper

- **Status:** Accepted
- **Date:** 2026-09-08
- **Issue:** [#152](https://github.com/hyperledger-identus/sdk-rust/issues/152)
- **Research:** [OpenSpec research](../../openspec/changes/adopt-bip39-mnemonic-engine/research.md)
- **Refines:** ADR 0061's approved `bip39` candidate

## Context

The local mnemonic implementation reproduces ASCII seed vectors but accepts
empty, unsupported-count and checksum-invalid mnemonics, accepts non-standard
entropy sizes, and omits BIP-39 NFKD normalization. Those are correctness and
interoperability defects in a security-sensitive standard boundary.

Exact `bip39 2.2.2` provides the closed mechanics, is current and maintained,
has no direct unsafe block or native code, and supports a zeroizing mnemonic
representation. Its dependency type formats words, and its convenience seed
method can own normalized passphrase text without zeroizing that allocation.
The crate is therefore suitable only behind an adapter that owns lifecycle and
diagnostics.

## Decision

Conditionally adopt exact `bip39 2.2.2` with default features disabled and only
`alloc,zeroize` enabled:

1. Preserve `MnemonicHelper`, `SecureRandom`, Identus errors and existing
   collection return types as the complete public boundary.
2. Use the dependency for English wordlist, entropy/checksum validation,
   mnemonic parsing, NFKD normalization and standard seed derivation.
3. Never expose, format, serialize or retain `bip39::Mnemonic` or its errors.
4. Move owned normalized text into `Zeroizing<String>` and call
   `to_seed_normalized`; wrap its fixed seed result in `Zeroizing` before the
   intentional caller-owned vector copy.
5. Preserve the KMP salt variant separately behind `kmp-compat`, sharing strict
   mnemonic validation but retaining Apollo's exact unnormalized passphrase
   bytes and unprefixed PBKDF2 salt.
6. Keep the infallible entropy API source-compatible: invalid lengths return an
   empty vector, while exact standard lengths map to their required word counts.
7. Remove the duplicated SDK English wordlist and standard PBKDF2 mechanics
   after all existing and new conformance tests pass.
8. Pin tag/artifact provenance, exact features, dependency cone, Rust
   1.85/1.98, target, advisory, license, unsafe and public-API evidence.

## Consequences

- Every consumer receives correct checksum, size and NFKD behavior.
- Invalid inputs previously accepted become the existing redacted mnemonic
  error; this is an intentional correctness migration without an API break.
- Seven packages enter the current workspace lock, all private to the feature;
  transitive pure-Rust unsafe is accepted as recorded evidence.
- Injected SDK randomness remains the only entropy source and multilingual,
  serde and crate-RNG features remain disabled.
- Dependency replacement remains local because no third-party type crosses the
  facade.

## Alternatives rejected

Keeping or extending the handwritten implementation repeats high-risk standard
mechanics and leaves demonstrated validation gaps. `tiny-bip39`, `coins-bip39`
and broader wallet suites add older or more coupled surfaces without improving
the owned facade. Exposing dependency types would make future replacement and
diagnostic hardening breaking changes.

## Verification and rollback

Acceptance requires all published English vectors, exact entropy/word-count
boundaries, checksum negatives, independent Unicode NFKD seed evidence, KMP
vectors, stable redacted errors, zeroizing lifetime checks, private public API,
feature graph evidence, Rust 1.85 and 1.98, supported portable targets, audit,
deny, full Nix/factory and distinct review.

Rollback reverts the issue #152 PR. Reconsider on advisory, maintenance loss,
target regression, vector drift, public-boundary leak or a narrower maintained
implementation with equivalent lifecycle guarantees.
