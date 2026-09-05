# Verification receipt

- **Change:** `add-wallet-storage-conformance`
- **Issue:** #91 under #20 / `IDR-010`
- **Develop merge base:** `db83fbc1d7dbe66f7c7a09bbc13a7f606559e67d`
- **Specification commit:** `45079122a7f7a63c1b562b994e2c0719d9181d74`
- **Implementation commit:** `d6ff338061a81d394e0fe4af04ae47c9951d03f2`
- **Commit policy:** both commits carry valid local GPG signatures and DCO
  trailers

## Focused behavior

- `cargo test -p identus-wallet-conformance`: 5 passed, 1 ignored diagnostic.
- `cargo test -p identus-conformance`: 21 passed.
- Negative evidence: replacement revision reuse and repeated pagination cursor
  are rejected; non-Debug canary values do not enter diagnostics.
- Dynamic evidence: all five production storage trait entry points pass;
  `SecretStore` is also exercised through a fully specified trait object.

## Performance

`cargo test -p identus-wallet-conformance release_exact_suite_throughput
--release -- --ignored --nocapture` completed 160,000 port calls in
30.265542 ms, approximately 5,286,540 calls/s on this host. This is diagnostic
evidence only and carries no machine-dependent threshold.

## Native and factory gates

- Workspace tests passed with all features and with no default features.
- Strict all-target Clippy and warning-denied workspace documentation passed.
- Bootstrap inventory passed with 15 packages; its 20 mutation tests passed.
- SSI upstream backlog passed all 30 rows.
- Factory structure and strict OpenSpec validation passed.
- `cargo tree -p identus-wallet-conformance --depth 1` shows only
  `identus-wallet` as a direct dependency.

## Pinned Nix gate

The repository-pinned `nix flake check --print-build-logs` passed all 28
compatible `aarch64-darwin` checks after applying the required Taplo formatting.
Evidence includes Rust 1.85 MSRV, workspace/all-feature builds, strict Clippy,
documentation, WASM, Android, iOS, source/license/advisory checks, factory
contracts and the principal release nextest suite: 362 passed, 17 diagnostics
skipped.

## Scope truth

The SDK now supplies reusable behavioral checks, not a production adapter.
No consumer or donor repository was modified. `IDR-010` remains `specified`
until two independent downstream receipts exist, including one encrypted
adapter.

## Hosted review hardening

PR #92 identified that target-specific runtime dependency tables were not part
of the verification-leaf dependency set. The shared collector now applies the
same layer and leaf checks to top-level and target-specific runtime edges,
deduplicates repeated edges, and continues to exclude development/build
dependencies. A focused regression test proves the boundary before merge.
