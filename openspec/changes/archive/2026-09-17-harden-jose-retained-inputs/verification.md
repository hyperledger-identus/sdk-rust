# Verification

- **Date:** 2026-09-17
- **Develop base:** `4c2942fc11124348f5a330334294a91e221306eb`
- **Planning head:** `d31a159652b3c9194e30facd3dcd363c67a43e34`
- **Preimplementation receipt head:** `7bcd34ac974b20381d8f0c6c1019d6c3fc3cdcaf`
- **Reviewed implementation head:** `7db37f44d24c9d10710cfd4ee35b3ad060f0cda5`

## Functional and invariant evidence

- `cargo test -p identus-jose --all-features`: passed.
- `cargo test -p identus-jose --no-default-features`: passed.
- `cargo test -p identus-oid4vci --all-features`: passed.
- `cargo test --workspace --all-features`: passed.
- `cargo test --workspace --no-default-features`: passed.
- `cargo clippy -p identus-jose --all-targets --all-features -- -D warnings`:
  passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `cargo fmt --all -- --check`, `git diff --check`, and
  `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`: passed.

The focused tests cover exact and one-over `kid` and client-ID lengths; one,
eight, empty, ninth, malformed, and oversized `x5c` inputs; accepted wire
round trips; tighter-limit reuse; and redacted diagnostics. Rustdoc
`compile_fail` examples prove that raw `String` and `Vec<String>` enum payloads
are no longer constructible outside the crate.

## Public API and supply chain

Rustdoc JSON was generated with the pinned Rust 1.98.1 toolchain and inspected
with `cargo-public-api 0.52.0`. The resulting `identus-jose` report has SHA-256
`a3313b719b276bfb214fef71c9de6d770df46d13c2604db39a0a37ea5d44cfb0`.
It exposes the three opaque value types, their fallible constructors and
borrowed accessors, while their tuple fields remain private.

`cargo-cyclonedx 0.5.9` generated a CycloneDX 1.5 SBOM for `identus-jose` with
77 components and SHA-256
`a14b1f0599bf6bf74308709a803cb1ffe2128369c6e4da7b3c86bd7b0457b129`.
The manifest and lockfile are unchanged. `cargo deny check` passed all advisory,
ban, license, and source policies; it reported only the existing unmatched
allowance and duplicate-`syn` warnings.

## Targets, factory, and routed slow evidence

- `cargo check -p identus-jose --all-features --target
  wasm32-unknown-unknown`: passed.
- The equivalent checks for `aarch64-apple-ios` and
  `aarch64-linux-android`: passed.
- `nix flake check --no-write-lock-file`: the first run built the Rust, target,
  Clippy, documentation, test, audit, and factory derivations, then correctly
  rejected non-canonical formatting in the new `archive-intent.toml`.
  `taplo fmt` corrected the artifact and
  `nix build .#checks.aarch64-darwin.lint-toml --no-link` passed. A second full
  source-hash rebuild was deliberately stopped after that exact failed gate was
  repaired; protected Linux `fast` remains the merge authority and the complete
  native closure belongs to the slow line.
- `./scripts/factory preflight harden-jose-retained-inputs --issue 299
  --validate-receipt`, `./scripts/factory check`, and strict OpenSpec
  validation: passed.

`./scripts/fuzz-jws.sh smoke` was attempted locally but the pinned stable shell
cannot accept libFuzzer's nightly-only `-Zsanitizer` flag. This is not promoted
as passing evidence. The exact diff plan routes `fuzz-conformance` to the
weekly/manual slow line; the committed JWS corpus and ordinary conformance tests
remain green in the fast evidence above.

The exact-diff production-ready plan selects required PR status `fast` and
recommends `fuzz-conformance`, `portable-targets`, and `security` for the slow
line. The local portable and dependency-policy portions passed; protected Linux
CI remains the merge authority.

## Consumer and decomposition evidence

The four named consumers were rechecked read-only after implementation. Their
heads and pre-existing status are unchanged from research: Oxid
`183664aeca500c25d6d27a22fa402b4d40c649d3`, Lace ID Portal
`804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`, Midnight Identity
`3e1672baf0a237d2360d393a124611d6452b8192`, and NeoPRISM
`d6ad1ecade80757f08da4f9101d14c2fb1a4d02b`. No consumer was mutated.

The reviewed implementation plan counted 28 paths and 966 changed text lines;
the final archive and canonical spec deltas bring the PR plan to 33 paths and
1,201 changed text lines, so a decomposition note is required. The change
intentionally remains one invariant-closing slice across
three cohesive bands: governance/planning evidence, the `identus-jose` API and
tests, and mechanical migration of two `identus-oid4vci` test callers. Splitting
those bands would temporarily make the public API, canonical limitation, or
workspace tests disagree. The excess over the 1,000-line guidance is entirely
the required review/verification archive plus generated canonical spec deltas;
there is no production consumer mutation or additional implementation scope.

## Harness evidence

The pinned Pi supervisor completed one session with 33 turns, 76 tool calls,
433,623 input tokens, 21,148 output tokens, 2,732,800 cache-read tokens, and
zero cache-write tokens. Its handoff envelope was rejected only because the
worker emitted an invalid completion timestamp. The code and evidence were
therefore reviewed and verified independently; the rejected handoff is retained
as factory telemetry debt rather than represented as a successful supervisor
handoff.
