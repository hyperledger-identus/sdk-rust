## Why

`MnemonicHelper` currently treats any collection of known English words as a
valid mnemonic, even an empty collection or a sentence with an invalid BIP-39
checksum. Its local entropy encoder also accepts non-standard sizes, and seed
derivation does not apply the standard NFKD normalization. Issue #152 makes
this Apollo-parity cleanup explicit so every sdk-rust consumer receives the
same standards-correct mnemonic mechanics.

## What Changes

- Adopt exact `bip39 2.2.2` as a private implementation dependency with only
  `alloc` and `zeroize` features.
- Preserve the existing `MnemonicHelper`, `SecureRandom`, `Vec<String>` and
  Identus error boundary while replacing local wordlist, checksum, entropy and
  standard seed mechanics.
- Accept only BIP-39 entropy sizes and word counts and reject invalid checksums.
- Apply NFKD normalization to mnemonic and standard passphrase input while
  retaining zeroizing ownership of temporary secret text and seed bytes.
- Preserve Apollo's opt-in `kmp-compat` salt behavior as an explicitly separate
  legacy path with byte-exact fixtures.
- Remove the duplicated 2,048-word list and standard PBKDF2 implementation once
  conformance and public-API evidence pass.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `crypto`: strengthen the existing BIP-39 helper contract and replace its
  internal mechanics without exposing dependency types.

## Impact

- **Issue:** #152, child of dependency-first cleanup #151 and crypto epic #9.
- **API:** existing signatures remain; values previously accepted despite
  invalid word count/checksum become `crypto.mnemonic_invalid`.
- **Dependencies:** seven packages are new to the current workspace lock; the
  standalone feature cone contains 13 packages and no native source.
- **Security:** dependency mnemonic formatting remains private; normalized
  passphrase and seed temporaries stay in SDK-controlled zeroizing storage.
- **Consumers:** receive standards-correct BIP-39 behavior with no downstream
  repository edit in this change.
- **Rollback:** revert one focused dependency/implementation/spec PR.
