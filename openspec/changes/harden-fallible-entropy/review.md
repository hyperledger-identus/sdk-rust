# Pre-implementation semantic, security, and API review

- **Date:** 2026-09-04
- **Issue:** #67, child of #9 / `IDR-004` and #20
- **Develop base:** `0b98a2e9fdd70e8e0792bfcb54a658cf60f22c8d`
- **Result:** contract is implementable with no unresolved blocker

## Findings

1. The current allocation-returning port permits work proportional to an
   arbitrary requested length. A caller-owned slice makes allocation policy
   explicit and keeps current crypto paths fixed at 32 bytes.
2. Entropy is intrinsically fallible. Encoding failure as panic makes library
   consumers and foreign-language bindings unable to recover.
3. The existing `SecureRandomFailure` variant and stable redacted code are the
   correct abstraction. Backend strings and random material must not cross the
   capability boundary.
4. Ed25519/X25519 exact-length panics disappear naturally because the caller
   supplies fixed arrays. EC rejection sampling still requires a bounded
   exhaustion error after sixteen invalid candidates.
5. A backend may partially write before failing. Constructors must return
   immediately without constructing, logging, or exposing output.
6. This is a deliberate breaking correction to unpublished `0.0.0` APIs. No
   compatibility shim is warranted because it would preserve unbounded
   allocation or infallibility.
7. The change belongs in generic sdk-rust crypto and entropy-adapter surfaces;
   it requires no Midnight policy, wallet custody, downstream mutation, new
   dependency, release, repository setting, or `main` change.

Verdict: READY to implement after strict OpenSpec validation.

# Post-implementation semantic, security, and API review

- **Reviewed head:** `d7eeabef6a72e1fae774ade3efff0a317933c64d`
- **Result:** passed with no unresolved finding

The exact `develop...d7eeabe` production diff is limited to the entropy port,
its two adapters, four random key constructors, two random mnemonic/seed
constructors, and their tests. Algorithms, encodings, derivation parameters,
stable error codes, dependencies, crate features, target policy, custody, and
wire/persistence surfaces are unchanged.

Every crypto caller owns a fixed `[u8; 32]` buffer. Provider errors return
before key or mnemonic construction. Ed25519 and X25519 no longer convert an
untrusted vector length with `expect`; secp256k1 and P-256 retain exactly
sixteen bounded scalar attempts and return the redacted existing error on
exhaustion. The getrandom adapter erases backend detail; the deterministic
adapter fills the complete supplied slice and handles zero length.

Regression tests exercise provider failure for all four key families and both
mnemonic entry points, and prove the exact EC retry ceiling. Existing vector,
signature, JWK, COSE, derivation, DID, and conformance suites remain green.
There is no production panic or allocation proportional to an entropy length
argument in the changed path.

Verification passed on the host and in the pinned Nix environment:

- focused all-feature crypto and entropy-adapter tests and strict Clippy;
- workspace all-feature tests, strict Clippy, and warning-denied rustdoc;
- native getrandom, browser `wasm32-unknown-unknown` getrandom, and crypto WASM
  builds;
- OpenSpec/factory validation and all 26 Nix flake checks, including Rust
  1.85 MSRV, Android ARM64, iOS ARM64, supply-chain, feature, and nextest gates.

The Nix run emitted known Darwin fixup-helper segmentation diagnostics while
auditing generated archives and offline cargo-audit index lookup diagnostics;
the derivations continued and the authoritative flake result was `all checks
passed`. Neither diagnostic came from changed code.

No downstream repository, `main`, release, publication, live setting, secret,
or dependency state was changed. Verdict: READY for receipt, archive, signed
issue-linked pull request, and exact-head hosted CI.
