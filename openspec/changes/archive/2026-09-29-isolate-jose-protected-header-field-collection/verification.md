# Verification evidence

## Identity

- Issue: #470
- Exact protected `develop` base:
  `3b5c91d384756fd0ef1ee36ef3e66f051f5950ba`
- Planning commit:
  `45f86b33e27afc7bdbebccbf6aff6c57cbbe7f08`
- Preimplementation receipt commit:
  `48af0fdeca3a109ae1fb3b0bf32c71dbfcd723ef`
- Characterization commit:
  `e912d54655fb71beb70c20cb150b755898df0895`
- Production implementation commit:
  `f8c33c01c6d903ab776d024d7752d14bb947d76b`
- Equivalence evidence commit:
  `fa31f9d8a351e191c3451442e0dc2265118947d5`
- Reviewed head after protected-baseline synchronization:
  `39ee586cf11af5480a8c81e5ad6ba0181e405d9e`
- Synchronized protected `develop` baseline:
  `90f1c5f0dd51ca4d49615acbb2a4e85a1a079c7e`

## Behavioral compatibility

- The complete pre-change compact JWS conformance suite passed: 12 tests and
  one ignored release diagnostic.
- Preimplementation characterization binds duplicate, unknown, and invalid
  known-value failures in source order; collection failures before map
  finalization; missing `alg` before ambiguous key references; and ambiguity
  after otherwise successful collection.
- The post-change suite passes: 13 tests and one ignored release diagnostic.
  Existing RFC bytes, closed-header rejection, exact limits, exclusive key
  references, redaction, deterministic round trips, and serialization remain
  green.

## Public, dependency, and resource compatibility

- `cargo-public-api 0.52.0` reports identical simplified public inventories
  for `identus-jose` at protected `develop` and the reviewed source, with
  digest `8dea38ec82eddfddd1e4b721989c932e3e94087001a41ae4ad0af9bcd8b7e5bf`.
- No manifest, feature, lockfile, dependency, unsafe, native, FFI, wire,
  serialization, public error, or protocol capability changed.
- The visitor still allocates each property name once and iterates the map once.
  The private enum is stack-only; the collector retains the same seven optional
  values; and the generic unique-value decoder replaces three identical typed
  branches without a collection, clone, dynamic dispatch, synchronization, or
  I/O path.
- Existing header, string, certificate-count, trust-chain-count, nested JWK,
  and compact-token limits remain authoritative.
- Source-distribution verification passes for all five governed packages.

## Code-health comparison

| Signal | Protected base | Reviewed source head |
| --- | ---: | ---: |
| `RawProtectedHeaderVisitor::visit_map` SLOC / cognitive / cyclomatic | 67 / 13 / 31 | no function signal |
| Replacement classifier/collector method signals | n/a | 0 |
| `header.rs` module signal | 0 | 0 |

The improvement is one private closed vocabulary plus one partial-state owner,
not a forwarding chain or helper-per-field split. The unrelated 28 / 4 / 16
`ProtectedHeader::serialize` signal remains visible and unchanged. The live
reviewed report has source fingerprint
`de1895419b3f20d05fcd41daad2f53cd6a873e5e5234f4348c8aedbd097c8428`,
population projection
`cfc603629e9ba2b2e33f9feb3d18b262a0618080ebfaaeebae892438598d2e53`,
and report digest
`ce3268592ed967919b7f0975762d45856460f31fd152dabaeaf80ddbbfa6fd4d`.

Canonical evidence will be regenerated and rebound after the implementation is
squash-merged to protected `develop`.

## Local gates

- Focused compact JWS conformance tests: passed.
- Workspace tests with all features: passed.
- Strict workspace Clippy with all targets and features: passed.
- Workspace documentation with warnings denied: passed.
- `wasm32-unknown-unknown`, `aarch64-linux-android`, and
  `aarch64-apple-ios` Nix checks: passed.
- Public API comparison: identical simplified inventories.
- Source distribution, factory contract, OpenSpec, formatting, diff, and Nix
  flake evaluation gates: passed.
- Rust 1.89 MSRV Nix gate: passed.
- Canonical Rust 1.98 Nix etalon gate: passed.
