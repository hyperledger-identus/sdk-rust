## 1. Contract and dependency decision

- [x] 1.1 Correct issue #152 provenance and current Rust policy before code
- [x] 1.2 Complete proposal, crypto delta, design, research, constraints and ADR
- [x] 1.3 Pass strict OpenSpec, research-readiness and constraint-readiness gates

## 2. Private BIP-39 integration

- [x] 2.1 Add exact private dependency with only `alloc,zeroize` features
- [x] 2.2 Replace local English wordlist, entropy and checksum mechanics behind
      the existing `MnemonicHelper` facade
- [x] 2.3 Normalize standard mnemonic/passphrase input with zeroizing owned
      temporaries and stable Identus error mapping
- [x] 2.4 Preserve the separately gated Apollo KMP salt path byte-for-byte and
      keep its PBKDF2 dependency out of the standard-only graph
- [x] 2.5 Remove superseded local wordlist and standard PBKDF2 mechanics

## 3. Conformance and boundary evidence

- [x] 3.1 Retain all 24 published English vectors and add every allowed entropy
      and word-count boundary
- [x] 3.2 Add invalid empty/count/checksum/entropy negative cases
- [x] 3.3 Add composed/decomposed Unicode passphrase equivalence and exact seed
- [x] 3.4 Prove KMP vectors, redacted errors, zeroizing types, dependency-private
      public API and minimal/default/KMP feature graphs

## 4. Validation and delivery

- [x] 4.1 Run focused tests, formatting, Clippy, docs, Rust 1.85, Rust 1.98,
      supported portable targets, deny, audit and complete Nix/factory gates
- [x] 4.2 Perform distinct correctness/security review and record findings
- [x] 4.3 Rebase onto `develop` after PR #180 merges, then rerun the focused gate
- [x] 4.4 Sync canonical crypto spec, archive safely and prepare the signed/DCO
      issue-linked PR packet; hosted CI/review and green-only merge remain PR evidence
