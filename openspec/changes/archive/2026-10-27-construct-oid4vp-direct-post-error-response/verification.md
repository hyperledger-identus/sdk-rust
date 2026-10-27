# Verification receipt

Verification date: 2026-10-27
Issue: #532
Develop base: `273063e8dbca6c3b631206bdd9914f66cc40aefa`
Planning commit: `899745dd04c4fdadab4eba603c1255206e804595`
Preimplementation receipt commit: `212301ce0dcaac31f9655a02008ca8e463b15131`
Reviewed head: `645fd5bc9c3ad1b58afd6e0c7d67edd3e1c81fcd`
Implementation commit: `86efc29b517ad4fe53c2a345c6cc47076556509e`

## Focused protocol and documentation evidence

- `cargo test -p identus-oid4vp --all-features`: 57 tests and all doc tests
  passed.
- `cargo clippy -p identus-oid4vp --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc -p identus-oid4vp --all-features
  --no-deps`: passed.
- The deterministic cross-language checker passed six packets, 15 sources and
  32 vectors, including the full invocation/JAR/DCQL/request-lineage error E2E
  packet.
- The 68-record OID4VP error contract, all twelve protocol codes, official
  Final form, exact state, invalid and boundary optional fields, encoding,
  overflow and redaction cases passed.
- No Cargo manifest or lockfile changed; the implementation reuses the existing
  `identus-core`, `fluent-uri` and `zeroize` dependency cone.

## Quality evidence

- The dependency-free benchmark passed: ordinary success 112,345 ns; boundary
  success 396,291 ns; ordinary error 73,537 ns; boundary error 76,562 ns.
- The AddressSanitizer fuzz smoke completed 4,096 executions without a failure
  and exercised both success and error construction branches.
- `scripts/factory check` and `scripts/factory validate` passed with seven
  ledger records, 32 vectors, six quality declarations and 24 obligations.
- `scripts/tests/factory-contract.sh` passed the complete synthetic mutation
  suite after its minimal fixture retained ADR 0180.
- `scripts/check-ssi-upstream-backlog.py` passed all 30 roadmap rows.

## Workspace, compiler and target evidence

The Nix-owned gates completed successfully from the reviewed source tree:

- Rust 1.98.1 `rust-test`: 1,047 workspace tests passed and 23 declared tests
  were skipped.
- Rust 1.89.0 `rust-msrv`: the locked workspace compiled with all targets.
- Rust 1.98.1 `rust-build-wasm32`: all supported packages, including
  `identus-oid4vp`, compiled for `wasm32-unknown-unknown`.

These are compile and local execution claims. Browser/device runtime,
network interoperability, HTTP execution, consumer adoption, publication and
certification are not claimed.

## Review result

Codex CLI reviewed the exact committed base-to-head diff in a read-only
sandbox and reported no actionable defects. The source branch is clean, every
commit is signed and carries DCO sign-off, and protected exact-head CI remains
the final hosted merge gate.
