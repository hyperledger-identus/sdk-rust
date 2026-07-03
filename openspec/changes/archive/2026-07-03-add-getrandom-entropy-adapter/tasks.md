## 1. Decision gate — Q3 (ring disposition)

- [x] 1.1 Investigate consumer impact: confirm `identus-adapters-entropy` is not consumed outside the workspace with `features = ["ring"]` under a published-API stability commitment (check crates.io publish status, any downstreampinned `ring` feature, semver expectations). Record the finding in `design.md` under D4 as the resolved Q3 outcome.
- [x] 1.2 Decide: clean cut (remove `ring`) vs deprecate-then-remove (keep `ring` buildable + `#[deprecated]`). If the gate flips to deprecate, amend the `adapters-entropy` spec delta (`Removed ring adapter` requirement → retain a deprecated `RingSystemRandomAdapter` requirement) and adjust tasks 3.x / 4.x to keep `ring` building. If clean cut, proceed with tasks as written.

## 2. Workspace dependency wiring

- [x] 2.1 Add `getrandom` to the root `Cargo.toml` `[workspace.dependencies]` with the version pinned per `workspace-dependency-conventions` (match `sdk-ts`'s `externals/anoncreds/wasm` usage: `getrandom = "0.2"`).
- [x] 2.2 If Q3 = clean cut: remove the `ring = "0.17"` line from `[workspace.dependencies]` (no remaining consumer). If Q3 = deprecate: keep it.

## 3. Adapter implementation (adapters-entropy)

- [x] 3.1 Update `crates/adapters-entropy/Cargo.toml`: add `getrandom = { workspace = true, optional = true, features = ["js"] }`; add `getrandom = [ "dep:getrandom" ]` feature; keep `default = []`.
- [x] 3.2 If Q3 = clean cut: remove the `ring` optional dependency and the `ring = [ "dep:ring" ]` feature line. If Q3 = deprecate: keep them and add `#[deprecated(note = "use the getrandom feature and GetrandomSystemRandomAdapter; ring does not build on wasm32")]` on `RingSystemRandomAdapter`.
- [x] 3.3 Implement `GetrandomSystemRandomAdapter` in `crates/adapters-entropy/src/lib.rs`: `#[cfg(feature = "getrandom")]`, `derive(Debug, Default, Clone, Copy)`, `impl SecureRandom for GetrandomSystemRandomAdapter` calling `getrandom::getrandom` to fill `num_bytes`. Match the existing `RingSystemRandomAdapter` panic contract (must not fail on supported targets).
- [x] 3.4 If Q3 = clean cut: delete `RingSystemRandomAdapter` and its `impl`. If Q3 = deprecate: leave it (deprecated).
- [x] 3.5 Update the crate's module doc-comment to reflect reality: remove the "deferred" framing for the getrandom adapter; describe `GetrandomSystemRandomAdapter` as the cross-platform (native + browser WASM) adapter and note WASI is out of scope.

## 4. Workspace build + lint verification

- [x] 4.1 `cargo build --workspace` passes with default features.
- [x] 4.2 `cargo build -p identus-adapters-entropy --features getrandom` passes on the native host.
- [x] 4.3 `cargo build -p identus-adapters-entropy --features getrandom --target wasm32-unknown-unknown` passes (browser WASM; the `js` feature resolves entropy to `crypto.getRandomValues()`).
- [x] 4.4 `cargo clippy --workspace --all-targets -- -D warnings` passes.
- [x] 4.5 `cargo fmt --check` passes.
- [x] 4.6 If Q3 = clean cut: confirm `rg -n 'ring' crates/ Cargo.toml` finds no remaining references in the entropy crate or workspace dep map (crypto crate's `ring`-absence comments may remain as historical notes — verify they still read correctly).
- [x] 4.7 `cargo build -p identus-crypto --target wasm32-unknown-unknown` passes with default features (verifies the `Wasm-clean by default` MODIFIED scenario survives the workspace `ring` removal; crypto never depended on `ring`, so this is a regression guard, not a new behavior).

## 5. Tests

- [x] 5.1 Add a test (behind `cfg(test)` + the `getrandom` feature, or a dev-dependency harness) asserting `GetrandomSystemRandomAdapter.generate_seed(32)` returns exactly 32 bytes and is non-zero.
- [x] 5.2 Add a `wasm32-unknown-unknown` build-smoke test or CI step confirming the `getrandom` feature compiles for browser WASM (mirror `sdk-ts`'s `wasm-pack --target=web` expectation, or a plain `cargo build --target wasm32-unknown-unknown` smoke).
- [x] 5.3 Confirm the existing `DeterministicRandomAdapter` tests still pass unchanged.

## 6. Spec sync and OpenSpec housekeeping

- [x] 6.1 After task 1 resolves: if the gate flipped to deprecate, amend `specs/adapters-entropy/spec.md` and `specs/crypto/spec.md` deltas so the `ring` adapter requirement is retained (deprecated) rather than removed. **Note:** the crypto delta now contains TWO MODIFIED requirements (`SecureRandom — the single infrastructure port` and `Wasm-clean by default`). Under the deprecate path the `ring` adapter stays in `identus-adapters-entropy` (deprecated), so the adapter crate remains wasm-incompatible via `ring`; the `Wasm-clean by default` MODIFIED entry must therefore be re-amended to restore the localization contrast ("only the adapter crate is wasm-incompatible" is true again under deprecate), rather than the clean-cut wording that drops it. Re-run `openspec validate add-getrandom-entropy-adapter --type change`.
- [x] 6.2 Final `openspec validate add-getrandom-entropy-adapter --type change` passes; `openspec status --change "add-getrandom-entropy-adapter"` shows all artifacts done.
