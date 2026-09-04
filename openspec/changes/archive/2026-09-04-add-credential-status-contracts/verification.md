# Verification receipt

- **Date:** 2026-09-05
- **Issue:** #77, child of #6 / `IDR-007` and program #20
- **Develop base:** `9463f1abfe7d0819c6a5944529fa3c1b8897c1d8`
- **Reviewed implementation:**
  `63ff1284f5cd5f0a7a7ae2bcf26c12359d83fa25`
- **Host:** Apple arm64, Darwin 25.2.0
- **Plain toolchain:** `rustc 1.95.0`, `cargo 1.95.0`
- **Nix:** 2.34.6; configured etalon nightly 2026-03-18 and MSRV 1.85.0

## Functional evidence

- Open method/purpose identifiers retain W3C, Midnight, and unrelated future
  spellings without a central enum.
- Distinct reference, handle, revision, and observed-value wrappers preserve
  exact text or bytes at each boundary and reject empty/invalid/oversized data.
- A 1–16 collection accepts multiple W3C revocation/suspension entries and
  rejects empty, oversized, and exact-duplicate binding sets.
- Freshness accepts an opaque minimum revision and/or maximum duration, rejects
  the empty shape, and selects no clock or comparator.
- Requirements distinguish absent unrestricted allow-lists from invalid empty,
  oversized, and duplicate present lists.
- A complete query rejects bindings excluded by its own requirements.
- W3C text and Midnight binary evidence retain binding/value/revision/time facts
  and reject reversed known time intervals without a lifecycle decision.
- Deterministic boundary/control-character matrices exercise neighboring sizes
  and no-panic constructor behavior.
- Direct and aggregate Debug canary tests prove opaque status data is redacted;
  all 14 status errors bridge to stable static `credential.*` contracts.

Focused result: 15 status tests passed and one manual diagnostic was ignored in
normal correctness runs. The complete credential crate passed 49 tests with
three manual diagnostics ignored.

## Performance evidence

Command:

```text
cargo test --release -p identus-credentials --test status \
  credential_status_construction_throughput_diagnostic -- \
  --ignored --nocapture
```

Observed result: 500,000 representative query/evidence pairs in 177.421625 ms,
or 2,818,146 pairs/second. This is a host observation, not a portable threshold.

## Local quality gates

The following completed successfully:

```text
cargo fmt --all -- --check
cargo check -p identus-credentials --no-default-features
cargo test -p identus-credentials
cargo test --workspace
cargo clippy -p identus-credentials --all-targets -- -D warnings
cargo clippy --workspace --all-targets --all-features -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc -p identus-credentials --no-deps
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps
./scripts/factory validate add-credential-status-contracts
./scripts/factory check
```

The exact dependency tree remains `identus-core` plus existing `zeroize`; no
manifest or lockfile changed. Source inspection found no serde/wire import,
unsafe code, runtime, network, registry, ledger, or downstream dependency.

## Mandatory Nix matrix

`nix flake check --print-build-logs` completed with all 26 checks passed. The
matrix included structural factory/OpenSpec validation, Rustfmt, strict Clippy,
309-workspace-test nextest execution, feature combinations, MSRV 1.85 builds,
the pinned NeoPRISM-etalon toolchain, WASM, Android arm64, iOS arm64, docs,
text/TOML/Nix linting, dependency licenses/sources, and the configured audit.
Nix reported only existing evaluation/license-index/fixup warnings and returned
success; this change introduced no dependency input.

## Donor and consumer isolation

Final read-only receipts match preflight exactly:

- `midnight-identity` `develop@427f8571950c42967a18726cbcbefecc19ef8d79`;
  pre-existing dirty nested `third_party/midnight-did` retained;
  `status.rs` SHA-256
  `984761ada00964197250dc26dddefe1e82d509ed2c14f8f3ada5cb23780c1635`;
  Apache-2.0 `LICENSE` SHA-256
  `c71d239df91726fc519c6eb72d318ec65820627232b2f796219e87dcf35d0ab4`.
- `oxid` `integration@bfe3b481568dc738f0732c2b27548fab8721fd95`;
  pre-existing `.claude/` and `.pi/taskflows/` retained.
- `lace-id-portal` `main@804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`;
  pre-existing `.pi-subagents/`, `.pi/`, and `tmp/` retained.
- `neoprism` `main@d6ad1ecade80757f08da4f9101d14c2fb1a4d02b`;
  clean.
- `apollo` `main@ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c`;
  pre-existing dirty nested `secp256k1-kmp/native/secp256k1` retained.

No donor source was copied verbatim. The implementation is a conceptual,
bounded, format-neutral adaptation under SDK-Rust's Apache-2.0 repository
license; every consumer/donor tree remained untouched.

## Review conclusion

The distinct exact-diff review in `review.md` found no unresolved semantic,
API, security, privacy, performance, portability, dependency, or ownership
issue. The change is ready to synchronize into canonical specs, archive, and
deliver through an issue-linked PR to `develop`.
