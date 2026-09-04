## 1. Contract and decision record

- [x] 1.1 Create issue #69 under #9 / `IDR-004` and record the exact base.
- [x] 1.2 Specify redacted formatting, owned-buffer erasure, limits, and compatibility.
- [x] 1.3 Record the security decision in ADR 0022.
- [x] 1.4 Complete pre-implementation semantic, security, and API review.

## 2. Long-lived derivation secrets

- [x] 2.1 Add the direct workspace-governed `zeroize` dependency.
- [x] 2.2 Redact `HDKey` and `EdHDKey` debug formatting.
- [x] 2.3 Implement explicit and drop-time zeroization for both HD key types.
- [x] 2.4 Test trait availability, redaction, erasure, and unchanged derivation vectors.

## 3. Secret intermediates

- [x] 3.1 Zeroize caller-owned curve-generation entropy after construction attempts.
- [x] 3.2 Zeroize mnemonic entropy and PBKDF2 result intermediates.
- [x] 3.3 Zeroize HD HMAC input and output intermediates.
- [x] 3.4 Preserve current public APIs, features, errors, algorithms, and byte outputs.

## 4. Verification and delivery

- [x] 4.1 Pass focused crate tests and relevant feature combinations.
- [x] 4.2 Pass workspace format, test, clippy, docs, OpenSpec, and factory gates.
- [x] 4.3 Pass declared MSRV, native, mobile, WASM, supply-chain, and Nix gates.
- [x] 4.4 Complete a distinct post-implementation security/API review.
- [x] 4.5 Produce ready/receipt, synchronize canonical specs, archive the change,
  and prepare the issue-linked signed/DCO PR; hosted CI, merge, issue closure,
  parent update, and `develop` sync remain authoritative GitHub evidence.
- [x] 4.6 Resolve the hosted review finding by guarding internally consumed
  random-mnemonic word strings and rerun focused gates.
