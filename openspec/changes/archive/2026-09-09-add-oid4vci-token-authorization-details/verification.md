# Verification receipt

Verification status: passed locally
Verification date: 2026-09-09
Base: develop@f28b88010b46e2dd6c3e9aaec2f087d515399ed6
Implementation head: f61a109

## Behavior receipt

- `cargo test -p identus-oid4vci` passed all crate tests, including eight new
  Authorization Details tests.
- The new suite covers the reconstructed Final example, mixed extensions,
  explicit-transition compatibility, required shapes, exact/one-over entries,
  identifiers and decoded UTF-8 bytes, decoded duplicates within/across
  entries, and success/error redaction.
- `cargo test --workspace --all-features --quiet` passed all workspace tests.
- Focused strict Clippy and rustdoc-with-warnings-denied passed for
  `identus-oid4vci`.

## Repository receipt

- `scripts/factory research-ready`, `constraints-ready` and `check` passed.
- `/nix/var/nix/profiles/default/bin/nix flake check` passed all 29 compatible
  aarch64-darwin checks, including Rust build/test/Clippy/docs/format, minimal
  and KMP profiles, WASM/iOS/Android compile checks, policy, dependency,
  license and advisory gates. Nix reported x86_64-linux as locally
  incompatible; hosted Linux is the merge authority.
- `git diff --check` passed for the complete base-to-head diff.
- All three implementation-history commits have good GPG signatures and DCO
  trailers.

## Supplemental observation

The non-canonical command `cargo clippy --workspace --all-targets
--all-features -- -D warnings` reaches existing test/configuration code not
selected by the repository Clippy gate and reports Rust 1.98 lints in four
unchanged DID/conformance files. The mandatory Nix Clippy derivations and the
focused changed-crate all-target Clippy both pass; no unrelated source was
mutated in this change.

## Exclusions

Hosted Linux CI/review, HTTP, Authorization Code execution, access-token or
issuer verification, metadata matching, identifier selection, Credential
Request construction, downstream adoption, publication, release and runtime
device/browser behavior remain unrun or out of scope.
