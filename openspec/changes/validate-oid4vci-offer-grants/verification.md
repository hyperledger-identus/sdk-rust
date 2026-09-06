# Verification receipt

## Candidate

- Issue: #115, child of #7 / #20 / `IDR-023`.
- Base: `origin/develop@bbfbbd77a68a619a616346aa9d3f31285b6effb3`.
- Branch: `codex/oid4vci-offer-grants`.
- Specification commit: `d1c16a6ace0be29d8bd4c4a6e856b6bca9ccf42b`.
- Reviewed implementation commit:
  `23fcbd872fa49d4a16ecfcc623098eda6a55830b`.
- Environment: repository-pinned Nix/Rust toolchains on `aarch64-darwin`.

## Contract evidence

- The two grant values defined by OpenID4VCI 1.0 Final are exposed through a
  consuming transition from the already validated core Credential Offer.
- Absent, empty, unknown, and concurrent known grant alternatives are retained
  without selecting or executing a flow. Every grant value remains object
  shaped and the exact transport JSON stays available.
- Empty `tx_code` means required with effective `numeric` mode. Populated
  requirements enforce the two Final modes, a positive bounded JSON integer,
  and the 300-Unicode-scalar description ceiling.
- Issuer state, Pre-Authorized Code, Authorization Server, and description
  values have independent positive byte policies, zeroizing ownership and
  content-redacted diagnostics. Errors bridge through stable static codes.
- Authorization Server hints accept the shared HTTPS/RFC 8414 identifier shape;
  later issuer-metadata membership and use-condition enforcement are explicitly
  deferred.

## Focused and workspace gates

- `cargo fmt --all -- --check`: passed.
- Grant tests under all features and no default features: 9 passed in each
  lane; core semantic tests: 10 passed; transport tests: 12 passed with one
  manual performance diagnostic ignored.
- Focused strict Clippy and warning-denied rustdoc: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `cargo test --workspace --all-features`: passed.
- `cargo test --workspace --no-default-features`: passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`: passed.
- `./scripts/factory doctor`, `./scripts/factory check`, support-policy,
  inventory, backlog, and `git diff --check` gates: passed.
- `cargo tree -p identus-oid4vci --depth 1` remains `identus-core`,
  `serde_json`, `uriparse`, and `zeroize`; no manifest or lockfile changed.

## Full reproducible matrix

The corrected exact tree passed `nix flake check --print-build-logs` across all
compatible `aarch64-darwin` checks. The matrix included factory and repository
hygiene; Rust 1.85 MSRV/feature lanes; native, browser-WASM, Android AArch64 and
iOS AArch64 builds; strict Clippy, rustdoc and formatting; dependency/license
policy; pinned advisory analysis; and release Nextest profiles. The principal
profile ran 451 tests: 451 passed and 22 manual diagnostics were skipped.

Nix correctly omitted incompatible `x86_64-linux`; hosted CI supplies the
independent Linux matrix. The hermetic advisory lane's offline index cannot
answer current yanked-package state, and the macOS fixup hook may emit its
known non-fatal temporary-directory diagnostic; neither diagnostic failed a
derivation or is represented as fresh online evidence.

## Review and isolation

The fresh exact-diff review in `review.md` resolved two findings before
readiness: object-shape enforcement for unknown grant extensions and removal of
an infallibility assumption from typed Transaction Code conversion. No
unresolved local finding remains.

Oxid and Lace ID Portal were read-only evidence sources. Their final revisions,
pre-existing status entries, and recorded source digests match issue #115's
preflight receipt. No consumer, `main`, release, publication, repository
setting, or chain state changed.

