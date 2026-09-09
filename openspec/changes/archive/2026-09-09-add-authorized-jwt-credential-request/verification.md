# Verification receipt

Verification status: passed locally
Verification date: 2026-09-09
Base: develop@52221cc0e9d2f2cef7c6c1a6eef8e3eddfc98810
Implementation head: a0da98d

## Behavior receipt

- `cargo test -p identus-oid4vci` passed all crate tests, including the three
  additive authorized-dataset cases in the eleven-test request suite.
- The suite proves exact mutually exclusive wire output, checked detail and
  identifier indices, exact offer binding, escaped identifiers, stable proof
  order, shared Bearer/proof/body bounds, redaction and legacy compatibility.
- `cargo test --workspace` passed all workspace tests.
- Focused strict Clippy and rustdoc-with-warnings-denied passed for
  `identus-oid4vci`.

## Repository receipt

- `scripts/factory research-ready`, `constraints-ready` and `check` passed.
- `/nix/var/nix/profiles/default/bin/nix flake check` passed all 29 compatible
  aarch64-darwin checks, including Rust build/test/Clippy/docs/format, minimal
  and KMP profiles, WASM/iOS/Android compile checks, policy, dependency,
  license and advisory gates. Nix reported x86_64-linux as locally
  incompatible; hosted Linux remains the merge authority.
- `git diff --check` passed for the complete base-to-head diff.
- All three implementation-history commits have good GPG signatures and DCO
  trailers.

## Supplemental observation

The non-canonical command `cargo clippy --workspace --all-targets
--all-features -- -D warnings` has known Rust 1.98 findings in unchanged
DID/conformance files. The mandatory Nix Clippy derivations and focused changed-
crate all-target Clippy pass; no unrelated source was mutated.

## Exclusions

Hosted Linux CI/review, HTTP execution, authorization-code flow,
request/response correlation, token/issuer/dataset/credential trust, product
selection, downstream adoption, publication, release and runtime device/browser
behavior remain unrun or out of scope.
