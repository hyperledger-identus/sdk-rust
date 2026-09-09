# Verification receipt

Verification status: passed locally
Verification date: 2026-09-09
Base: develop@49c5288713f9979747c56f0307cf6ed12c785422
Implementation head: d60ffc7a8ecadc526a639249ddcfe1057c2683ad
Reviewed test head: 37c70e9935457eef0da6c31cd1b99ad75b44e06a

## Behavior receipt

- `cargo test -p identus-oid4vci`, all-features and no-default-features passed.
- The ten-test Deferred Credential Request suite proves the Final example,
  exact POST/media type/endpoint/body, JSON escaping, positive limits,
  exact/one-under complete-body bounds, default worst-case escaping, larger
  response-policy isolation, endpoint omission, repeated construction, static
  error codes and diagnostic redaction.
- Focused strict Clippy and rustdoc-with-warnings-denied passed.
- `cargo test --workspace` passed every workspace test at the implementation
  head.

## Repository receipt

- `scripts/factory research-ready`, `constraints-ready`, preflight receipt
  validation, `./bootstrap.sh --check`, formatting and `git diff --check`
  passed.
- `/nix/var/nix/profiles/default/bin/nix flake check` passed all 29 compatible
  aarch64-darwin checks at the implementation head, including Rust build/test,
  strict Clippy/docs/format, Rust 1.98.1, minimal/KMP/entropy profiles,
  WASM/iOS/Android compilation, factory, dependency, license and advisory
  gates. Nix reported x86_64-linux as locally incompatible; hosted Linux is the
  merge authority.
- The review-only follow-up changes tests; its focused ten-test suite, strict
  Clippy, formatting and diff check pass.
- Every commit has a good GPG signature and DCO trailer.

## Dependency and source receipt

No Cargo manifest, lockfile, crate, feature, dependency, unsafe, donor or
downstream change exists. The Final example and edge cases were independently
constructed from the pinned OpenID Foundation document.

## Exclusions

Hosted exact-head Linux CI/review, HTTP/TLS execution, access-token lifecycle,
interval scheduling, retry/replay/invalidation, encryption, error response
handling, response correlation, storage, downstream adoption, publication,
release and device/browser runtime behavior remain unrun or out of scope.
