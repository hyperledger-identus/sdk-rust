## 1. Contract and decision record

- [x] 1.1 Create issue #67 under #9 / `IDR-004` and record the exact base.
- [x] 1.2 Specify the fallible caller-owned entropy contract and constructor migration.
- [x] 1.3 Record the API/security decision in ADR 0021.
- [x] 1.4 Complete pre-implementation semantic, security, and API review.

## 2. Port and adapters

- [ ] 2.1 Replace `generate_seed` with fallible `fill_bytes`.
- [ ] 2.2 Make getrandom backend failures return the stable crypto error.
- [ ] 2.3 Update the deterministic test adapter to fill caller-owned slices.
- [ ] 2.4 Test success, deterministic behavior, zero-length input, and provider failure.

## 3. Crypto consumers

- [ ] 3.1 Make Ed25519 and X25519 generation fallible.
- [ ] 3.2 Make secp256k1 and P-256 generation fallible with bounded exhaustion.
- [ ] 3.3 Make random mnemonic/seed creation fallible.
- [ ] 3.4 Update all in-tree callers and add failure/exhaustion regression tests.

## 4. Verification and delivery

- [ ] 4.1 Pass focused crate tests and feature combinations.
- [ ] 4.2 Pass workspace format, test, clippy, docs, OpenSpec and factory gates.
- [ ] 4.3 Pass native and browser-WASM entropy adapter build checks.
- [ ] 4.4 Complete a distinct post-implementation review.
- [ ] 4.5 Archive the change, open an issue-linked signed/DCO PR to `develop`,
  pass hosted CI, merge, close #67, update #9, and sync `develop`.
