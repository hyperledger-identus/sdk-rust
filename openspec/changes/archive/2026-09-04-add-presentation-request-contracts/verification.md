# Verification receipt

- **Component and issue:** `identus-presentations`, #79; parent #20 and
  `IDR-008`
- **Base SHA:** `9b0d10790b9257f94fc9eecd2af99a82c592ca70`
- **Production commit:** `1304ec79d15423bbdddcf3ce52264e57485cbe3d`
- **Compatibility:** additive unreleased experimental API; no external
  dependency/version, feature, wire, persistence, execution, trust-policy,
  chain, protocol, product, or consumer commitment
- **Dependency delta:** one local workspace edge from `identus-presentations`
  to `identus-credentials`; `identus-core` retained

## Focused commands passed

- `cargo fmt --all -- --check`
- `cargo test -p identus-presentations` (14 passed; one manual diagnostic
  ignored)
- `cargo check -p identus-presentations --no-default-features`
- `cargo clippy -p identus-presentations --all-targets -- -D warnings`
- `RUSTDOCFLAGS=-Dwarnings cargo doc -p identus-presentations --no-deps`
- inventory, factory, and whitespace checks

## Performance observation

On an Apple Silicon `aarch64-apple-darwin` host using rustc 1.95.0
(`59807616e`, 2026-04-14), the ignored release diagnostic constructed and
request-validated 250,000 representative presentation request/candidate pairs
in 165.798792 ms, approximately 1,507,852 pairs/second. This is an observation,
not a correctness threshold or cross-host performance promise.

## Full commands passed

- `cargo test --workspace --all-features`
- `cargo test --workspace --no-default-features`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `RUSTDOCFLAGS=-Dwarnings cargo doc --workspace --no-deps`
- `nix flake check --print-build-logs` (all 30 compatible host checks; pinned
  nightly, Rust 1.85 MSRV, native, Android, iOS, WASM, feature, docs, tests,
  lint, architecture, dependency/license/advisory, and factory lanes)

The Nix release workspace lane ran 323 tests successfully with 13 configured
skips. The run emitted the repository's known non-fatal macOS fixup-hook
segmentation diagnostics, MSRV cfg diagnostics, and offline crates.io
yanked-index lookup diagnostics; all governed derivations completed
successfully.

## Review, provenance, and isolation

- Pre-implementation semantic/API/privacy/performance review: no blocker.
- Distinct post-implementation review of
  `develop@9b0d107...1304ec7`: no unresolved finding.
- Oxid's Apache-2.0 request/candidate domain was the primary conceptual seed;
  OpenID4VP 1.0 Final and W3C VC Data Model 2.0 supplied normative context.
- midnight-identity and Lace supplied compatibility shapes. NeoPRISM and
  Apollo supplied architectural and negative-boundary evidence. No donor code
  was copied and no donor or consumer repository was mutated.
- Request wire decoding, DCQL/profile parameters, candidate discovery/ranking,
  consent, trust, proof/holder-binding execution, presentation artifacts and
  receipts, lifecycle, storage, FFI, chain and product behavior remain focused
  follow-up slices.
- Pre-archive factory receipt: branch
  `codex/idr-008a-presentation-core`, head
  `1304ec79d15423bbdddcf3ce52264e57485cbe3d`, merge base
  `9b0d10790b9257f94fc9eecd2af99a82c592ca70`.
- Canonical specifications were synchronized and the change was archived as
  `2026-09-04-add-presentation-request-contracts`. The generated placeholder
  purpose was replaced with the reviewed capability intent.
- Hosted review/CI, merge, effort report, parent update, `develop` sync, and
  cleanup remain delivery evidence.
