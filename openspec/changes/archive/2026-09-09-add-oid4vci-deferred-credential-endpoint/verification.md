# Verification receipt

Verification status: passed locally
Verification date: 2026-09-09
Base: develop@a8fdedc8b39aa93df807a88c7dc4e861cf5367df
Implementation head: 29a9c47

## Behavior receipt

- `cargo test -p identus-oid4vci`, all-features and no-default-features passed.
- The sixteen-test issuer-metadata suite proves exact advertised and omitted
  behavior, invalid types, unsafe URL classes, duplicate rejection, exact and
  one-over endpoint bounds, stable error codes and diagnostic redaction.
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
- Both implementation-history commits have good GPG signatures and DCO
  trailers.

## Supplemental observation

No dependency, feature, manifest, lockfile, unsafe, donor or downstream change
exists. Final examples were independently reconstructed from the pinned
normative document; no donor fixture or code was copied.

## Exclusions

Hosted Linux CI/review, Deferred Credential Request construction, HTTP,
polling/scheduling, token/transaction/issuer trust, encryption, response
correlation, storage, downstream adoption, publication, release and runtime
device/browser behavior remain unrun or out of scope.
