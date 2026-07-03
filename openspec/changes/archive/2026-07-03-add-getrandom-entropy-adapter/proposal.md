## Why

`identus-adapters-entropy` ships a single production entropy adapter backed by
`ring`, which does not compile on `wasm32-unknown-unknown`. The upcoming uniffi
**Kotlin + WASM** bindings therefore have no entropy source on the WASM path,
blocking key generation and mnemonic creation for TS/web consumers. The
`crypto` spec already anticipates a `getrandom`-backed wasm adapter as a known
second backend for the `SecureRandom` port and marks it "deferred to a future
binding change" — this is that change.

## What Changes

- Add a `getrandom`-backed `SecureRandom` adapter
  (`GetrandomSystemRandomAdapter`) to `identus-adapters-entropy`, gated behind
  a new `getrandom` cargo feature. It works across all uniffi targets:
  Kotlin/JVM, Android, native, and browser WASM
  (`wasm32-unknown-unknown`).
- Wire the `getrandom` dependency with the `js` feature (mirroring the
  precedent in `sdk-ts`'s `externals/anoncreds/wasm`), so the browser-WASM path
  resolves to `crypto.getRandomValues()`. The `js` feature is a no-op on
  non-wasm targets, so it can be enabled unconditionally.
- **Decision-gated (see Q3 in `design.md`):** the disposition of the existing
  `ring`-backed adapter (`RingSystemRandomAdapter`) and its `ring` cargo
  feature. Recommended path is a clean cut — remove `ring` and the
  `RingSystemRandomAdapter`, since `ring` is consumed *only* by this crate
  (nothing else in the tree depends on it) and `getrandom` covers every target
  `ring` did, with identical OS-CSPRNG security. Final call is gated behind a
  consumer-impact check (task 1).
- The `DeterministicRandomAdapter` (feature = `deterministic`) is unchanged and
  orthogonal to this decision.

## Capabilities

### New Capabilities

- `adapters-entropy`: The `identus-adapters-entropy` outer-boundary crate's
  `SecureRandom` adapter family — the concrete backends (system-RNG and
  deterministic-test) that satisfy the `identus_crypto::SecureRandom` port.
  No spec existed for this capability; the ring adapter landed without one via
  the consumed `add-adapter-crate-layer` change. This change formalizes it.

### Modified Capabilities

- `crypto`: Two requirements are updated. (1) The `SecureRandom` port
  requirement currently states the `getrandom`-backed wasm adapter is
  "deferred to that future binding change"; this change realizes that
  adapter, so the wording is updated to reflect both backends now live in
  `identus-adapters-entropy` (no longer deferred). No behavior of the port
  itself changes. (2) The `Wasm-clean by default` requirement previously
  localized the wasm limitation to the `identus-adapters-entropy` crate
  ("only the adapter crate is wasm-incompatible") on the assumption that
  the `ring` adapter lived there; under the recommended Q3 clean-cut
  disposition the `ring` feature is removed entirely and the adapter crate
  becomes wasm-clean via `getrandom`, so the requirement is rewritten to
  drop the now-false localization contrast (its contrast scenario is
  removed) while preserving the crypto-builds-on-wasm32 scenario.

## Impact

- **Code:** `crates/adapters-entropy/Cargo.toml` and `src/lib.rs` gain the
  `getrandom` adapter; potentially lose `RingSystemRandomAdapter` and the
  `ring` feature (per Q3). The crate's module doc-comment, which already names
  `GetrandomSystemRandomAdapter` and calls it "deferred," is updated to match
  reality.
- **Dependencies:** Add `getrandom` to the workspace `[workspace.dependencies]`
  and to `adapters-entropy`. Potentially remove `ring` from the workspace
  dependency map and from `adapters-entropy` (per Q3). `identus-crypto`
  already has no `ring` dependency, so it is unaffected.
- **Specs:** New `specs/adapters-entropy/spec.md`; delta to
  `specs/crypto/spec.md` (wording update on the deferred adapter).
- **Consumers:** Any in-tree or published consumer enabling
  `identus-adapters-entropy` with `features = ["ring"]` is affected if Q3
  resolves to a clean cut. The gating check (task 1) surfaces this before any
  breaking edit is made.
- **Targets:** Browser WASM only (`wasm32-unknown-unknown`). WASI
  (`wasm32-wasi`/`wasip1`) is explicitly **out of scope** — confirmed by the
  workspace's `wasm-pack --target=web` pipeline and the absence of any WASI
  target in the workspace or `sdk-ts`.