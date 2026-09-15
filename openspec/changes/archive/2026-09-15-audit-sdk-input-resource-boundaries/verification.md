# Verification evidence

- **Date:** 2026-09-16
- **Issue:** #168
- **Develop base:** `66ec2b9b3a7ec35cf21ecc52cdca5bebed0b4d0d`
- **Environment:** aarch64-darwin, repository-pinned Nix and Rust 1.98.1
- **Result:** passed

## Focused and workspace evidence

- `scripts/check-input-resource-boundaries.py .`: 27 boundary families passed,
  including distinct DID method-registry, cache-policy, and cache-adapter rows.
- `scripts/tests/input-resource-boundaries.py`: all structural mutation cases
  passed locally and inside the isolated Nix factory derivation.
- BIP-39 derivation tests: 35 default and 40 `kmp-compat` tests passed, including
  exact/one-over word, entropy, and passphrase cases plus published vectors.
- JWK all-feature integration tests: 23 passed, including exact/one-over member,
  depth, node and text budgets, serde/native parity, redaction, and iterative
  rejection cleanup for a depth-32,768 native extension tree.
- `cargo test --workspace --all-features`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `cargo fmt --all -- --check`, Taplo format check, and `git diff --check`:
  passed.
- `scripts/factory check audit-sdk-input-resource-boundaries`, the complete
  factory contract, OpenSpec validation, archive preservation, constraints,
  source distribution, Apollo parity, support policy, code health, and error
  goldens: passed.

## Candidate and reproducible matrix

The corrected pinned `crypto-candidate` app completed in 66.814 seconds with
Rust 1.98.1,
all four configured feature profiles, three unpublished crate archives, SBOMs,
and public-API evidence. The baseline delta contains only the intended additive
BIP-39 and JWK limit constants; no public error variant or dependency edge was
added.

`nix flake check --fallback` passed all 31 compatible aarch64-darwin checks:
factory isolation, text/TOML/Nix lint, source contract, format, builds,
strict Clippy variants, default/minimal/KMP/entropy tests, Rust 1.98.1 policy,
WASM, Android aarch64, iOS aarch64, etalon, rustdoc, cargo-deny, and
cargo-audit. Nix reported x86_64-linux as incompatible with this local host;
protected Ubuntu CI supplies that independent gate before merge.

## Iteration evidence

The first isolated factory attempt correctly omitted an untracked checker from
the Git-backed flake source. After staging the candidate, Taplo identified the
new inventory formatting, and the formatted form exposed a whitespace-coupled
mutation helper. Both issues were corrected; the final staged tree passed the
isolated factory derivation and the full compatible Nix closure.

Hosted review then identified two inventory omissions and one native rejection
cleanup hazard. The correction adds three DID inventory rows and an internal
iterative JWK extension-map guard. All focused and full checks above were rerun
on that corrected tree before publication. Each hosted finding has a resolution
reply linked to commit `0900b2f`, and all three review threads are resolved.

A subsequent hosted finding identified the unbounded `Multihash` compatibility
placeholder. The checker vocabulary, inventory, ADR, limitation index, and
canonical specification now disclose it explicitly without changing its public
API or pretending an outer layer enforces a typed limit. The focused checker,
mutation suite, complete factory contract, and all 31 compatible
`nix flake check --fallback` checks pass after this correction.

A later hosted finding identified recursive drop of hostile-depth owned DID
JSON after native validation fails. The inventory, ADR, limitation index, and
canonical specification now disclose the complete native family and direct
consumers to bounded slice parsing or a pre-entry depth bound. The focused
checker, constraints, factory, and formatting are rerun after this documentation
correction; protected exact-head CI remains required before integration.

## Repository boundary

No downstream repository was edited, switched, copied from, or built. No
publication, tag, support-tier activation, consumer migration, or `main`
promotion is part of this change.
