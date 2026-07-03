## Why

`add-crypto-capability` shipped `MnemonicHelper::create_seed` with the
**standard BIP-39 salt** (`"mnemonic" + passphrase`), verified against
published BIP-39 test vectors. The KMP `apollo` port origin it was ported
from **omits the `"mnemonic"` prefix** — a confirmed bug, not an ecosystem
convention. The consequence: a wallet created by KMP `apollo` or `cloud-agent`
(which calls `MnemonicHelper.createSeed`) **cannot be reproduced by
`identus-crypto`** from the same mnemonic — different salt → different 64-byte
seed → unrelated key trees. There is no passphrase value that makes the two
agree, because KMP always omits the prefix.

A second, related deviation compounds the hazard: `identus-crypto` currently
inherits KMP's `"AtalaPrism"` default passphrase while keeping the *correct*
standard salt prefix, producing a **hybrid** salt `"mnemonicAtalaPrism"` that
interops with nothing — not KMP `"AtalaPrism"`, not cloud-agent `""`, not
standard `"mnemonic"`. This must be fixed now, while `add-crypto-capability`
has no downstream dependents yet, so the fix is free.

## What Changes

- **BREAKING**: Drop the `"AtalaPrism"` default passphrase from
  `MnemonicHelper::create_random_seed`; the canonical default becomes `""`
  (standard BIP-39). The current `"AtalaPrism"` + standard-salt hybrid
  interops with nothing, so this is a strict improvement with no real
  consumers to migrate.
- Add `MnemonicHelper::create_seed_kmp(mnemonics, passphrase)`, an opt-in
  KMP-interop escape hatch that uses the **KMP salt** (`passphrase` with no
  `"mnemonic"` prefix) for one-way legacy PRISM wallet import. The passphrase
  is **required** (no default) — the caller must state the source wallet's
  passphrase explicitly (cloud-agent: `""`; KMP-default: `"AtalaPrism"`; or
  the user's value).
- `create_seed_kmp` is gated behind a new **`kmp-compat`** Cargo feature,
  introduced here as an empty gate (no new dependency — it only drops the
  salt prefix, reusing existing `pbkdf2`/`hmac`/`sha2`). The feature is
  **opt-in and off by default**, so the KMP quirk stays invisible on the
  canonical surface. The feature is shared with the future
  ed25519-bip32 interop change, which will later extend it with
  `dep:ed25519-bip32`.
- Both `create_seed` and `create_seed_kmp` validate the mnemonic via
  `is_valid_mnemonic_code` → `Error::MnemonicInvalid` (no new error variant,
  no new `ErrorCode` catalogue entry — the failure is semantic and identical
  regardless of which salt was about to be used).
- No migration/re-keying helper. "Migrating" a legacy wallet to the standard
  salt means re-deriving *different* keys (a new identity/DID) from the same
  mnemonic — a wallet/identity lifecycle decision, explicitly an
  `identus-wallet` concern, not a crypto-primitive operation.

## Capabilities

### New Capabilities

_None._

### Modified Capabilities

- `crypto`: The "BIP39 mnemonic helper" requirement changes — the canonical
  default passphrase becomes `""` (dropping the non-standard `"AtalaPrism"`
  default), and a new `create_seed_kmp` interop function is added behind the
  `kmp-compat` feature. The "Feature-gated with all-on default" requirement
  changes — a new opt-in `kmp-compat` feature is introduced (off by default,
  not in `default`).

## Impact

- **Code**: `crates/crypto/src/derivation/mnemonic.rs` (default passphrase
  constant, new `create_seed_kmp` fn, shared validation/PBKDF2 core); the
  `is_valid_mnemonic_code` → `Error::MnemonicInvalid` path is reused
  unchanged.
- **Cargo**: `crates/crypto/Cargo.toml` gains a new `kmp-compat` feature
  (initially `kmp-compat = []`, an empty gate). No new external dependency.
- **APIs**: One breaking signature-behavior change (`create_random_seed`
  default passphrase `"AtalaPrism"` → `""`); one new opt-in function
  (`create_seed_kmp`). No change to the canonical `create_seed` signature.
- **Error surface**: Unchanged — reuses `crypto.mnemonic_invalid`.
- **Interop**: After this change, all five field salts are reachable
  (standard `"mnemonic"`, standard `"mnemonic"+pp`, legacy cloud-agent `""`,
  legacy KMP-default `"AtalaPrism"`, legacy user-passphrase). The canonical
  default (new SDK wallet) is maximally portable to any standard BIP-39
  wallet; every legacy import requires the caller to state the source
  wallet's passphrase explicitly.
- **Companion change**: The `kmp-compat` feature is shared with the
  still-exploring `ed25519-derivation-canonical-standard` work. This change
  introduces the feature as an empty gate; the ed25519 change later extends
  it with `dep:ed25519-bip32`. The two divergences are different in kind and
  shipped separately.