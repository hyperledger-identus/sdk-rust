## Context

`add-crypto-capability` (now complete) ported `MnemonicHelper` from the KMP
`apollo` crate. During verification, two divergences from the KMP port origin
were found in the **seed derivation** path:

1. **Salt prefix (a confirmed KMP bug).** BIP-39 defines
   `salt = "mnemonic" + passphrase`. KMP `apollo` passes `passphrase` directly,
   omitting the prefix. `identus-crypto` shipped with the **correct** standard
   salt. Result: KMP/`cloud-agent`-created wallets cannot be reproduced from
   the same mnemonic by `identus-crypto`.

2. **Default passphrase (a KMP quirk, half-inherited).** KMP defaults
   `passphrase` to `"AtalaPrism"`; BIP-39 specifies `""`. `cloud-agent` calls
   `createSeed(words, "")` explicitly and sidesteps the default, but
   `identus-crypto`'s `create_random_seed` convenience inherited `"AtalaPrism"`
   **while keeping the correct salt prefix**, yielding a hybrid salt
   `"mnemonicAtalaPrism"` that interops with nothing.

The full reasoning and the locked decisions are captured in the pre-change
exploration `openspec/explorations/bip39-salt-kmp-compat.md`. This design
implements those locked decisions; it does not re-open them.

A companion exploration, `ed25519-derivation-canonical-standard.md`, covers a
**separate** KMP divergence (ed25519-bip32 vs SLIP-0010 — a canonical-standard
choice, not a bug). The two share the `kmp-compat` Cargo feature but are
shipped as independent changes. This change introduces the feature as an empty
gate; the ed25519 change later extends it with `dep:ed25519-bip32`.

## Goals / Non-Goals

**Goals:**

- Keep the standard BIP-39 salt (`"mnemonic" + passphrase`) as the canonical
  default — new SDK wallets remain portable *out* of the PRISM ecosystem to
  any standard BIP-39 wallet.
- Provide a KMP-interop escape hatch (`create_seed_kmp`) so legacy PRISM
  wallets (KMP `apollo`, `cloud-agent`) can be imported by stating the source
  wallet's passphrase explicitly.
- Eliminate the `"mnemonicAtalaPrism"` hybrid by dropping the `"AtalaPrism"`
  default passphrase in favor of the standard `""` default.
- Keep the KMP quirk opt-in and invisible by default via a `kmp-compat` Cargo
  feature.
- Reuse the existing validation + PBKDF2 core; no new error variant, no new
  `ErrorCode`, no new external dependency.

**Non-Goals:**

- No migration/re-keying helper. Re-deriving different keys from the same
  mnemonic is a wallet/identity lifecycle decision (an `identus-wallet`
  concern per `add-crypto-capability` Decision 9), not a crypto primitive.
- No non-English wordlist support (inherited `add-crypto-capability` scope;
  deferred to a future change). A non-English mnemonic is rejected by both
  variants until that change lands.
- No ed25519-bip32 derivation (the companion change's scope).
- No change to the canonical `create_seed(mnemonics, passphrase)` signature.

## Decisions

### Decision 1: Standard BIP-39 salt stays canonical; KMP salt is opt-in

**Choice:** `create_seed` keeps `salt = "mnemonic" + passphrase"` unchanged.
The KMP salt (`passphrase` with no prefix) is exposed only via
`create_seed_kmp`, behind `kmp-compat`.

**Rationale:** The KMP omission is a confirmed bug (evidence in the
exploration: KMP's own test passes `"mnemonic"` *as the passphrase* to mimic
the standard empty-passphrase salt). Enshrining it as the default would
re-import the root cause into the reference implementation and lock new
wallets into the PRISM quirk. New SDK wallets must be portable *out* of the
PRISM ecosystem.

**Alternatives considered:**
- *Make KMP salt the default.* Rejected — locks new wallets into a bug,
  forecloses portability to standard BIP-39 wallets (Trezor, Ledger, Bitcoin,
  Cardano Daedalus/Yoroi).
- *Salt-mode enum on `create_seed`.* Rejected — puts the KMP quirk on the
  canonical surface, visible at every call site. The two-function shape keeps
  the quirk opt-in and invisible by default; the `_kmp` suffix is intentionally
  loud ("escape hatch, prefer standard") at the call sites that do use it.

### Decision 2: Default passphrase becomes `""` (drops `"AtalaPrism"`)

**Choice:** `DEFAULT_PASSPHRASE` changes from `"AtalaPrism"` to `""`.
`create_random_seed` therefore derives with salt `"mnemonic"` (the standard
empty-passphrase salt).

**Rationale:** The current `"AtalaPrism"` + standard-salt combination produces
`"mnemonicAtalaPrism"`, which interops with nothing — not KMP `"AtalaPrism"`,
not `cloud-agent` `""`, not standard `"mnemonic"`. Dropping it is a strict
improvement. `add-crypto-capability` has no downstream dependents yet, so this
is the cheapest moment to fix it.

**Alternatives considered:**
- *Keep `"AtalaPrism"` default.* Rejected — preserves a hybrid that interops
  with nothing.
- *Remove `create_random_seed` entirely.* Rejected — the convenience is useful;
  the fix is the default value, not the function's existence.

### Decision 3: `create_seed_kmp` requires an explicit passphrase (no default)

**Choice:** `create_seed_kmp(mnemonics, passphrase)` has no default passphrase.
The caller must state the source wallet's passphrase (`""` for cloud-agent,
`"AtalaPrism"` for KMP-default, or the user's value).

**Rationale:** The passphrase composes with the salt to determine the seed. A
wrong default silently derives wrong keys. Legacy import is exactly the
context where the caller knows (or must discover) the source wallet's
passphrase; a silent default here is a footgun, not a convenience.

**Alternatives considered:**
- *Default `create_seed_kmp` passphrase to `""`.* Rejected — `""` is only
  correct for `cloud-agent`-origin wallets; KMP-default wallets use
  `"AtalaPrism"`. A default would be right for one legacy population and
  silently wrong for the other.

### Decision 4: Both variants validate the mnemonic

**Choice:** Both `create_seed` and `create_seed_kmp` call
`is_valid_mnemonic_code` and error with `Error::MnemonicInvalid` on failure.

**Rationale:** A typo'd 24-word phrase copied from an old wallet is exactly
  when validation matters most. The silent-key-derivation hazard is identical
  (arguably worse) for legacy import.

**Alternatives considered:**
- *Skip validation in `create_seed_kmp` to mirror KMP exactly.* Rejected — KMP
  does validate in `createSeed`; the validation is not part of the divergence.
  Skipping it would import a *second* KMP behavior we don't want.

### Decision 5: Reuse `crypto.mnemonic_invalid`; no new error code

**Choice:** No new `Error` variant, no new `ErrorCode` catalogue entry. Both
variants return the existing `Error::MnemonicInvalid` / `crypto.mnemonic_invalid`.

**Rationale:** The failure ("mnemonic is invalid") is semantic and identical
regardless of which salt was about to be used; the caller already knows which
variant they called. The stable `ErrorCode` catalogue stays semantic, not
per-function.

### Decision 6: `kmp-compat` feature introduced as an empty gate

**Choice:** `crates/crypto/Cargo.toml` gains `kmp-compat = []` (off by
default, not in `default`). `create_seed_kmp` is `#[cfg(feature = "kmp-compat")]`.

**Rationale:** `create_seed_kmp` needs no new dependency — it only drops the
salt prefix, reusing existing `pbkdf2`/`hmac`/`sha2`. The feature exists purely
to keep the KMP-interop surface opt-in and invisible by default, and to be
shared with the future ed25519-bip32 change (which will extend it to
`kmp-compat = ["dep:ed25519-bip32"]`).

**Alternatives considered:**
- *No feature gate; `create_seed_kmp` always compiled.* Rejected — puts the
  KMP quirk on the default surface; callers see it in autocompletion/docs even
  if they never need legacy import.
- *Wait and introduce `kmp-compat` only with the ed25519 change.* Rejected —
  couples this confirmed-bug fix to an unresolved design question. The two
  divergences are explicitly different in kind.

### Decision 7: Shared validation/PBKDF2 core, no duplication

**Choice:** Both functions funnel through one private helper that performs
validation + PBKDF2; the only per-variant input is the salt string
(`format!("{SALT_PREFIX}{passphrase}")` vs `passphrase.to_string()`).

**Rationale:** The two variants differ in exactly one byte string. A shared
core keeps the validation guarantee (Decision 4) structurally guaranteed
rather than reimplemented twice.

### Decision 8: `kmp-compat` is exercised by a distinct CI check, not `--all-features`

**Choice:** The Nix flake gains two parallel checks — `rust-test-kmp-compat`
and `rust-clippy-kmp-compat` — built with `cargoBuildFeatures = [ "kmp-compat" ]`.
The existing default-feature `rust-test`/`rust-clippy` checks are kept unchanged.
`--all-features` is explicitly rejected.

**Rationale:** `nix flake check` previously ran clippy/nextest with default
features only, so the `#[cfg(feature = "kmp-compat")]` surface
(`create_seed_kmp` + its tests) was never compiled in CI — tasks 7.2/7.4 were
satisfied manually but unenforced. `--all-features` is the wrong fix: the
crane checks are workspace-wide, so it would also enable `ring` and
`deterministic` on `identus-adapters-entropy` (`ring` is deliberately kept
out of the crypto surface), and it would collapse the two required scenarios
("default surface excludes the KMP surface" and "opt-in surface builds") into
one everything-on job, losing the `kmp_create_seed_is_absent_without_feature`
coverage. `--features kmp-compat` is a no-op on workspace members that don't
define the feature, so it stays scoped to `identus-crypto`. Two distinct
checks form the 2-cell matrix the spec describes.

The "absent without feature" half is enforced by a `trybuild` `compile_fail`
case (`tests/ui/kmp_create_seed_absent.rs`, which calls `create_seed_kmp`)
driven by a `#[cfg(not(feature = "kmp-compat"))]` test in `derivation.rs`. A
plain `#[test]` cannot assert non-existence: referencing `create_seed_kmp`
would fail to *compile* when the gate is correctly kept, while not referencing
it passes vacuously. `compile_fail` observes the non-compilation as a passing
assertion, and the `not(feature)` gate on the driver skips it in the
`kmp-compat` job (where the case would compile and `compile_fail` would report
unexpected success). This mirrors the `trybuild` pattern already used in
`crates/derive`. `trybuild` runs as an ordinary `#[test]`, so `cargoNextest`
(the runner the nix checks use) exercises it.

**Alternatives considered:**
- *`--all-features` on the existing checks.* Rejected — pulls `ring`/`deterministic`,
  loses default-surface coverage, and couples the crypto opt-in to an
  unrelated adapter feature axis.
- *A single check with `--features kmp-compat` replacing the default.* Rejected —
  drops the "absent without feature" scenario, which the default-feature check
  is the only thing enforcing at test time.

## Risks / Trade-offs

- **[BREAKING: `create_random_seed` default passphrase changes]** → Mitigation:
  `add-crypto-capability` has no downstream dependents; the previous default
  produced a non-portable hybrid. The "break" fixes a bug. Documented
  explicitly in the proposal and spec.
- **[Callers using `create_seed_kmp` with the wrong passphrase silently derive
  wrong keys]** → Mitigation: `create_seed_kmp` has *no* default passphrase
  (Decision 3); the caller must state it. The `_kmp` suffix and the
  `kmp-compat` feature gate make the escape-hatch nature obvious at every call
  site.
- **[Feature shared with a not-yet-proposed companion change]** → Mitigation:
  introduced as an empty gate here; the ed25519 change extends it later. No
  coupling to the ed25519 design's open question. If the ed25519 change never
  lands, `kmp-compat = []` remains a valid, useful opt-in for the salt variant
  alone.
- **[Legacy wallets with user-set passphrases are only reachable if the user
  remembers the passphrase]** → Mitigation: this is inherent to BIP-39 (a
  passphrase is a memory secret, not a stored value). `create_seed_kmp`
  exposes the capability; recovering the passphrase is a wallet UX concern,
  not a crypto-primitive one.

## Migration Plan

No code migration is required (no downstream dependents). For users importing
legacy PRISM wallets:

| Legacy wallet source | SDK call |
|----------------------|----------|
| `cloud-agent` (passphrase `""`) | `create_seed_kmp(words, "")` |
| KMP-default (passphrase `"AtalaPrism"`) | `create_seed_kmp(words, "AtalaPrism")` |
| KMP with user-set passphrase | `create_seed_kmp(words, <user pp>)` |
| New SDK wallet | `create_random_seed(rng)` (standard salt, `""` passphrase) |
| Standard import (any standard BIP-39 wallet) | `create_seed(words, pp)` |

Rollback: revert the change. Since there are no dependents, rollback is clean.

## Open Questions

None. All design questions were locked during the exploration grilling session
(see `openspec/explorations/bip39-salt-kmp-compat.md`, "Decisions locked").