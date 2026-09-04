## 1. Contract and decision record

- [x] 1.1 Create issue #67 under #9 / `IDR-004` and record the exact base.
- [x] 1.2 Specify the fallible caller-owned entropy contract and constructor migration.
- [x] 1.3 Record the API/security decision in ADR 0021.
- [x] 1.4 Complete pre-implementation semantic, security, and API review.

## 2. Port and adapters

- [x] 2.1 Replace `generate_seed` with fallible `fill_bytes`.
- [x] 2.2 Make getrandom backend failures return the stable crypto error.
- [x] 2.3 Update the deterministic test adapter to fill caller-owned slices.
- [x] 2.4 Test success, deterministic behavior, zero-length input, and provider failure.

## 3. Crypto consumers

- [x] 3.1 Make Ed25519 and X25519 generation fallible.
- [x] 3.2 Make secp256k1 and P-256 generation fallible with bounded exhaustion.
- [x] 3.3 Make random mnemonic/seed creation fallible.
- [x] 3.4 Update all in-tree callers and add failure/exhaustion regression tests.

## 4. Verification and delivery

- [x] 4.1 Pass focused crate tests and feature combinations.
- [x] 4.2 Pass workspace format, test, clippy, docs, OpenSpec and factory gates.
- [x] 4.3 Pass native and browser-WASM entropy adapter build checks.
- [x] 4.4 Complete a distinct post-implementation review.
- [x] 4.5 Produce ready/receipt, synchronize canonical specs, archive the change,
  and prepare the issue-linked signed/DCO PR; hosted CI, merge, issue closure,
  parent update, and `develop` sync remain authoritative GitHub delivery evidence.
