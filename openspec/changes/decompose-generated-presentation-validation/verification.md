# Verification evidence

## Identity

- Issue: #450
- Exact protected develop base:
  `f1873f39ad4154a7139553e3d3c8221ac6abecf7`
- Planning commit: `148a25c04c790a33945c35576c9a69b9510a2fe9`
- Preimplementation receipt commit: `ff1417e51f57cf345a55c4a75200bedf8cbb8e02`
- Reviewed implementation head:
  `dbf60d78d210fbe1fb10fb209a57f4f3ec4f26fa`
- Protected-base synchronization: the signed merge commit at the reviewed head
  incorporates the non-overlapping IDR-024 owner repair from protected
  `develop@9288221dfe5288a4fbe78b57ba4d06057d9f181d`.

## Behavioral compatibility

- The complete pre-change `identus-presentations` suite passed: one unit test,
  three error-contract tests, six lifecycle tests with one ignored diagnostic,
  and 26 presentation tests with one ignored diagnostic.
- A preimplementation multi-fault matrix binds the existing first-error order:
  artifact cardinality; exact request/plan binding; aggregate payload budget;
  selection lookup; request-query lookup and format; duplicate binding across
  earlier artifacts; and final plan-order coverage.
- The post-change `identus-presentations` suite passes with the matrix added:
  one unit test, three error-contract tests, six lifecycle tests with one
  ignored diagnostic, and 27 presentation tests with one ignored diagnostic.
- The private validator preserves artifact order, binding order, earlier-slice
  duplicate lookup, and plan order. The public constructor still owns artifact
  cardinality, exact request/plan validation, and retention of the caller's
  owned plan and artifact vector.

## Public, dependency, and resource compatibility

- `cargo-public-api 0.52.0` with the repository compiler and subprocess-scoped
  `RUSTC_BOOTSTRAP=1` reports no removed, changed, or added simplified public
  item between the protected base and implementation head.
- No manifest, feature, lockfile, dependency, unsafe, native, FFI,
  serialization, wire, or proof contract changed.
- No new collection, clone, callback, dynamic dispatch, or trait bound was
  introduced. Existing bounded linear scans and their nested iteration order
  remain unchanged.
- Source-distribution verification passes for all five governed packages.

## Code-health comparison

| Signal | Protected base | Implementation head |
| --- | ---: | ---: |
| `GeneratedPresentation::new` SLOC | 57 | no constructor signal |
| `GeneratedPresentation::new` cognitive complexity | 16 | no constructor signal |
| `GeneratedPresentation::new` cyclomatic complexity | 23 | no constructor signal |
| Unrelated `validate_candidates` signal | 43 / 11 / 16 | 43 / 11 / 16 |
| Presentation module signals | 0 | 0 |

The improvement is one private `GeneratedPresentationValidator<'a>` with
cohesive budget, binding, and coverage phases plus one small public
coordinator. It is not generated code, threshold weakening, a waiver, or
forwarding-only movement. The live report for the reviewed head has source
fingerprint
`aff2b1239416166e84c0d405c056943ae56d7f9b49a9033f32717d8a5ea16b49`,
population projection
`2757e495e1d40accbae481bdb00dba835584f6a9de01c0c4fed2d24b4f9deb80`,
and report digest
`8851739e6db7f833517735c77f9c1a8cd10128c47090511c4716616feb2bac14`.

The canonical baseline cannot pin a feature-branch commit because protected
delivery uses squash merge. The implementation must merge first; a distinct
closeout will generate and pin the report from the durable protected squash.

## Local gates

- Focused presentation tests: passed.
- Workspace tests with all features: passed.
- Strict workspace Clippy with all targets and features: passed.
- Workspace documentation with warnings denied: passed.
- `wasm32-unknown-unknown`, `aarch64-linux-android`, and
  `aarch64-apple-ios` workspace checks: passed.
- Public API comparison: no added, removed, or changed simplified item.
- Source distribution, factory contract, OpenSpec, formatting, diff, and Nix
  flake evaluation gates: passed.
- Canonical Nix `rust-test` gate: passed.
