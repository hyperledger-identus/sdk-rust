# Verification receipt

Verification date: 2026-09-29
Issue: #447
Develop base: `a5a54d6fac55912cd79d1f6112311375c96ccac3`
Planning commit: `492e14bdcb019b7d21f9c56258cf5ecd772ef7be`
Preimplementation receipt commit: `1fc2034`
Reviewed implementation commit: `cca83767d0a7bc896a2d43679d5fed3f017afbea`

## Focused evidence

- `cargo test -p identus-oid4vp`: passed all 32 unit, integration and document
  tests, including nine new authorization-routing tests.
- `cargo clippy -p identus-oid4vp --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked -p identus-oid4vp --no-deps`:
  passed.
- The v1 error fixture passed with 50 unique static code/kind/message records,
  including 11 append-only routing categories.
- Positive, absent, unsupported, nonce-grammar, independent-limit, ambiguous
  destination, unsafe URI, deterministic precedence, redaction and legacy
  query-only compatibility vectors passed.

## Workspace and factory evidence

- `cargo test --workspace --all-features`: passed.
- `cargo test --workspace --no-default-features`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps`:
  passed for all 21 packages.
- `scripts/factory check`, strict OpenSpec validation, research readiness,
  material constraint readiness and immutable preflight validation passed.
- Bootstrap inventory passed for 21 packages; 43 input-resource boundary
  records passed.
- `git diff --check` passed and no manifest, lockfile, dependency, feature,
  workflow, unsafe/native or downstream repository change exists.

## Compiler and portable-target evidence

The repository-pinned Nix derivations passed for:

- Rust 1.89.0 MSRV workspace build;
- `wasm32-unknown-unknown` build;
- `aarch64-apple-ios` build;
- `aarch64-linux-android` build.

The primary Rust 1.98 workspace evidence is covered by the focused/workspace
test, Clippy and rustdoc commands above.

## Architecture and code-health evidence

The exact-head code-health audit at `cca8376` reported no new OID4VP function
or module signal. The base-to-implementation slice exceeds the advisory total
file/line threshold because it includes mandatory specification, ADR,
architecture inventory, fixture and negative-test evidence; production logic
remains one cohesive 244-line module plus small private seams. The distinct
exact-diff review records no unresolved finding.

## Unrun and non-claims

Hosted Linux exact-head CI remains mandatory before merge. No HTTP/DNS/TLS
execution, network interoperability, endpoint authorization, verifier trust,
credential verification, consent, presentation generation, response
construction, downstream adoption, release, publication or certification is
claimed. Fuzz/soak execution remains recommended slow evidence rather than a
required gate for this additive bounded state transition.
