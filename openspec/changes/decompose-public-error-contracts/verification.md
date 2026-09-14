# Verification receipt

## Identity and provenance

- Umbrella issue: `#271`; delivery issue: `#278`.
- Exact develop base: `353030a7f263b9a1fba9deac0312ed228e61d761`.
- Planning-contract head: `7d20510124103ef7b18c916376bfbc3d97d1cd87`.
- Reviewed implementation and hardening head:
  `16687bc41d437a4d7d2911ae4e72710d7185933e`.
- Compiler: repository-pinned Rust `1.98.1`.
- Planning/stable golden: 47 rows with SHA-256
  `6148a00b22bdb8551d4df9369c7a9fcf819e1cf1c2654edc8654feef82227c7c`.
- All commits through the reviewed head have valid local GPG signatures and
  DCO sign-offs.

## Compatibility and architecture evidence

- `cargo-public-api 0.52.0` compared `identus-credentials` at the exact base
  and reviewed head with blanket and auto-trait implementations omitted. It
  reported no removed, changed, or added public items. The tool ran with the
  pinned compiler and `RUSTC_BOOTSTRAP=1` solely because rustdoc JSON remains
  an unstable compiler interface.
- `Cargo.toml`, `crates/credentials/Cargo.toml`, and `Cargo.lock` have an empty
  base/head diff. The dependency inventory remains `identus-core`,
  `identus-derive`, and `zeroize`; the crate still declares no features.
- No public enum, constant, method signature/constness, re-export, Serde/FFI
  surface, unsafe/native code, runtime input, protocol behavior, issue #7
  behavior, or issue #168 behavior changed.
- Both error enums route without wildcards. One macro-owned variant inventory
  per enum generates both its router and test inventory, so a new unmapped
  variant is a compile error and cannot be hidden by a separately maintained
  test list.
- The 47 stable decisions are split across five crate-private catalogue modules
  by envelope, metadata, status, verification-report, and verification-runtime
  invariants. No shared workspace error layer or public introspection surface
  was introduced.

## Behavioral and golden evidence

- Default, `--no-default-features`, and `--all-features` credentials test runs
  each passed 60 tests with four explicitly ignored manual throughput
  diagnostics, plus zero doctests.
- The dedicated contract suite passed three integration checks and one unit
  inventory check. It compares every one of the 44 `CredentialError` and three
  `CredentialVerificationError` variants with the immutable pre-refactor
  oracle, including local/public display differences, code, kind, capability,
  visibility inventory, const use, redaction, and empty source behavior.
- `scripts/check-error-golden.py` validates byte identity, fixed provenance and
  the immutable SHA-256. `scripts/tests/error-golden.py` passed 17 mutation
  cases covering coordinated and single-copy drift, schema/provenance drift,
  missing/ambiguous active and archive states, and leaf/intermediate symlinks.
- The checked-in Rust integration test embeds only the stable crate fixture;
  it does not read an active or archived OpenSpec path at runtime. Nix retains
  exactly that CSV in cleaned Rust sources and excludes the planning copy.

## Quality and target evidence

- `cargo clippy --locked -p identus-credentials --all-targets --all-features
  -- -D warnings`, crate rustdoc, workspace formatting, and `git diff --check`
  passed at the reviewed head.
- Direct pinned-environment checks of `identus-credentials` passed for
  `wasm32-unknown-unknown`, `aarch64-linux-android`, and
  `aarch64-apple-ios`. These are compile checks, not runtime/device or binding
  claims.
- The Nix workspace Rust test gate passed 707/707 tests with 22 skipped
  diagnostics. Strict all-target/all-feature Clippy, rustdoc, source-contract,
  WASM, Android, iOS and factory derivations passed. The existing generic
  target derivations do not name `identus-credentials`; therefore the direct
  package checks above, rather than those generic target derivations, are the
  credentials-specific target evidence.
- On Darwin, Nix may print the repository's known `find | while` fixup
  segmentation diagnostic after a successful archived-target build; the
  affected derivations returned exit status zero. This is not reported as a
  failed Rust gate.

## Maintainability measurements

The line measures are physical source lines over the two original owning files
and the new production catalogue modules; test-only blocks are excluded where
noted. They are review diagnostics, not a line-count budget.

| Measure | Base | Reviewed head | Result |
| --- | ---: | ---: | --- |
| Independently maintained variant projection/display decisions | 94 | 47 | 50% fewer decision sites |
| Projection/display mapping lines | 266 | 79 | 70.3% reduction |
| Largest error-mapping function/router | 175 | 52 | 70.3% reduction |
| Relevant production-module physical lines | 701 | 875 | +174 lines |

The total line increase is intentional and disclosed: named domain catalogues,
the private record, exhaustive routing, and compile-time inventory cost more
physical lines while halving behavioral decision sites and shrinking the
largest review unit. This pilot justifies independent evaluation of later
crates; it does not yet justify a shared macro or framework.

## Review and residual boundaries

Two independent exact-head review passes challenged the fixture's independence
and found active/archive ambiguity plus leaf and parent symlink gaps. All were
fixed before approval. The final architecture and adversarial reviews are
clear at `16687bc41d437a4d7d2911ae4e72710d7185933e`.

This slice does not create a wire error schema, retryability model, structured
metadata, localization, binding API, consumer adoption, or a rule for
data-bearing/source-bearing errors. Presentations, JOSE, and OID4VCI remain
separate issue-linked decisions under #279, #280, and #277.
