# Verification evidence

## Identity

- Issue: #459
- Exact protected `develop` base:
  `f7a0d58b534a0af2a5cd135fa2bab71f77195991`
- Planning commit:
  `9904e8acd1d44afdeef4220313ed54fd6be0058f`
- Preimplementation receipt commit:
  `50b53a6cf0d5084626eb18ea7c01f42f543a37e3`
- Characterization commit:
  `f486daad2306b4aa743f0f103016e6fcab6f47e3`
- Reviewed implementation head:
  `b95db35942a33678068b1005ef2530958bb36819`

## Behavioral compatibility

- The complete pre-change DID registration suite passed: 19 tests passed and
  one release-only diagnostic was ignored.
- Preimplementation characterization binds all eight terminal/non-terminal
  state/job-presence shapes and exact combined-fault priority across job
  method, shape, handle cardinality, DID/document identity, public-document
  policy, action/wait continuation, wait bound, document metadata identity,
  and extension policy.
- The post-change suite passes with that characterization added: 21 tests
  passed and one release-only diagnostic was ignored.
- Existing adapter, request-correlation, redaction, resource-bound, hostile-
  cleanup, concurrency, and Prism/Midnight lifecycle-shape tests remain green.

## Public, dependency, and resource compatibility

- `cargo-public-api 0.52.0` with repository rustdoc JSON reports byte-for-byte
  identical simplified public inventories for `identus-did` at the protected
  base and reviewed implementation head: 120,106 bytes each.
- No manifest, feature, lockfile, dependency, unsafe, native, FFI,
  serialization, wire, adapter, or public error contract changed.
- The private validator borrows the same method, job, state, and metadata. It
  adds no collection, clone, allocation, callback, dynamic dispatch, trait
  bound, I/O, or unbounded work path.
- Handle, public-document, metadata, and extension resource limits remain the
  authoritative work and memory boundaries.
- Source-distribution verification passes for all five governed packages.

## Code-health comparison

| Signal | Protected base | Implementation head |
| --- | ---: | ---: |
| `validate_result` SLOC | 75 | no function signal |
| `validate_result` cognitive complexity | 18 | no function signal |
| `validate_result` cyclomatic complexity | 28 | no function signal |
| Replacement validator method signals | n/a | 0 |
| `registration/result.rs` module signal | 0 | 0 |

The improvement is one private borrowed `RegistrationResultValidator` with
job-method, exhaustive lifecycle, terminal/action/wait, and document-metadata
phase owners. It is not generated code, threshold weakening, a waiver, an
owned mirror, or a helper per conditional. The live report for the reviewed
head has source fingerprint
`d91115abd76ec9f55eb0ce244322b071da966704756db28236be44e54d9046ec`,
population projection
`2757e495e1d40accbae481bdb00dba835584f6a9de01c0c4fed2d24b4f9deb80`,
and report digest
`85e9b5dacc309e10a234638a70d5639c2c1c6fdccfd4b6d0a4ea29b364e0e14e`.

Canonical evidence must be regenerated and rebound after the implementation
is squash-merged to protected `develop`.

## Local gates

- Focused DID registration tests: passed.
- Workspace tests with all features: passed.
- Strict workspace Clippy with all targets and features: passed.
- Workspace documentation with warnings denied: passed.
- `wasm32-unknown-unknown`, `aarch64-linux-android`, and
  `aarch64-apple-ios` workspace checks: passed.
- Public API comparison: identical simplified inventories.
- Source distribution, factory contract, OpenSpec, formatting, diff, and Nix
  flake evaluation gates: passed.
- Rust 1.89 MSRV gate: passed.
- Canonical Rust 1.98 Nix nextest gate: 905 passed, 22 skipped.
