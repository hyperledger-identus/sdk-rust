# Verification receipt

## Identity and provenance

- Delivery issue: `#152`; parent dependency decision: `#151`.
- Exact base: `7c84621018eb5e9e9f68ab0abc57d60aa27b55b3`
  (`origin/develop` after issue #177 merged).
- Reviewed implementation head: `8caaad44a2b2e0c5d3e1030965fc52ec112b64ac`.
- Upstream `bip39` tag commit: `d6dbc31678cc507c8cae62b3a059b0b48e866436`;
  artifact checksum:
  `90dbd31c98227229239363921e60fcf5e558e43ec69094d46fc4996f08d1d5bc`.
- Apollo reference: `ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c`.
- Primary compiler: Rust `1.98.1`; effective workspace MSRV: Rust `1.85.0`.

## Local gates

- Focused derivation suites passed 32/32 with `kmp-compat` and 28/28 with
  standards derivation only; scoped all-target Clippy passed with warnings
  denied.
- `nix flake check --print-build-logs` passed every aarch64-Darwin-compatible
  check for implementation head `8caaad4`. The primary workspace nextest suite
  passed 600/600 with 22 skipped diagnostics; the KMP crypto lane passed
  106/106. Primary, effective-MSRV, etalon, WASM, Android, iOS, rustdoc,
  formatting, Clippy, audit, deny and repository-policy lanes passed.
- Current online `cargo audit` loaded 1,242 advisories and found no
  vulnerability. `cargo deny check bans licenses sources`, all-feature rustdoc,
  strict OpenSpec validation, research readiness, constraint readiness and
  `git diff --check` passed.
- The Nix offline audit emitted non-fatal missing-yank metadata while its
  advisory and derivation gate completed successfully.

## Conformance, API and dependency receipt

- All 24 published English BIP-39 vectors, every allowed entropy/word count,
  invalid entropy/count/checksum/word cases and composed/decomposed Unicode
  passphrases pass. Existing Apollo/cloud-agent KMP vectors remain byte exact,
  and a Unicode test proves that legacy KMP passphrase bytes are not silently
  normalized.
- `cargo-public-api 0.52.0` found no `bip39` dependency type in the public API.
  Its diff reports only the reviewed removal of the duplicate public
  `wordlist` module/constant; every `MnemonicHelper` method and
  `derivation::mnemonic::wordlist()` are unchanged.
- No-default crypto contains neither `bip39` nor `pbkdf2`; standards derivation
  contains `bip39` and no SDK `pbkdf2`; `kmp-compat` contains both. The workspace
  lock gains exactly seven packages.
- Artifact SHA-256: implementation
  `119d962cee1dc98516d550686fa4baac7c97f4accdab0108d197e826c0ffa79f`;
  tests `765dacde68d1999f7eb3ff9e15b5c46ba5c99e730aa1184de8e8419514f20a80`;
  research `bc084d68cf2bb64bdde160d02238b4db67f1d6fb7d17c30f9c5e56a0e37f8535`;
  lockfile `5eca413ab59f652dc5fd2c3ec041247e828bce7cb30e2ef8c26b9cc0de079dd7`.

## Residuals and review result

- Returned `Vec<String>` mnemonic words and `Vec<u8>` seed bytes are
  caller-owned and are not automatically erased. This is the existing facade;
  changing it requires a separate public-API decision.
- Compile gates prove target compatibility, not runtime platform certification
  or guaranteed physical-memory erasure.
- The fresh correctness/security review found no unresolved blocker. The
  implementation is ready for guarded archive and exact-head hosted CI; merge
  remains prohibited until every required PR check is green.
