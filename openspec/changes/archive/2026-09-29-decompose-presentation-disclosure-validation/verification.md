# Verification evidence

## Identity

- Issue: #444
- Exact protected develop base:
  `e15314797408b5d7904b5075081dadb144254276`
- Planning commit: `3d71983ab4d8a475bb9d1887a4925fc808241ce1`
- Preimplementation receipt commit: `307883c`
- Reviewed implementation head:
  `738a7d993a8fbd415729bad492fe5e14dc032630`

## Behavioral compatibility

- The complete pre-change `identus-presentations` suite passed: one unit test,
  three error-contract tests, six lifecycle tests with one ignored diagnostic,
  and 25 presentation tests with one ignored diagnostic.
- A preimplementation multi-fault matrix binds the existing first-error order:
  selection cardinality; exact-request candidate validation; duplicate selection
  identity; query then candidate lookup; requested path, intent, and candidate
  availability; required claims; and final query coverage in request order.
- The post-change `identus-presentations` suite passes with the matrix added:
  one unit test, three error-contract tests, six lifecycle tests with one
  ignored diagnostic, and 26 presentation tests with one ignored diagnostic.
- The private validator preserves the existing ordered slices and uses borrowed
  request, candidate-set, and selection references. The public constructor still
  owns cardinality preflight, exact-request revalidation, request cloning, and
  retention of the caller's selection vector.

## Public, dependency, and resource compatibility

- `cargo-public-api 0.52.0` with the repository compiler and subprocess-scoped
  `RUSTC_BOOTSTRAP=1` reports no removed, changed, or added simplified public
  item between the protected base and implementation head.
- No manifest, feature, lockfile, dependency, unsafe, native, FFI,
  serialization, wire, or proof contract changed.
- No new collection, clone, callback, dynamic dispatch, or trait bound was
  introduced. Existing bounded linear scans and their iteration order remain
  unchanged.
- Source-distribution verification passes for all five governed packages.

## Code-health comparison

| Signal | Protected base | Implementation head |
| --- | ---: | ---: |
| `PresentationDisclosurePlan::new` SLOC | 73 | no constructor signal |
| `PresentationDisclosurePlan::new` cognitive complexity | 23 | no constructor signal |
| `PresentationDisclosurePlan::new` cyclomatic complexity | 27 | no constructor signal |
| Unrelated `validate_candidates` signal | 43 / 11 / 16 | 43 / 11 / 16 |
| Presentation module signals | 0 | 0 |

The improvement is one private `DisclosurePlanValidator<'a>` with cohesive
phase methods and one small public coordinator. It is not generated code,
threshold weakening, a waiver, or forwarding-only movement. The live report
for the reviewed head has source fingerprint
`aff2b1239416166e84c0d405c056943ae56d7f9b49a9033f32717d8a5ea16b49`,
population projection
`2757e495e1d40accbae481bdb00dba835584f6a9de01c0c4fed2d24b4f9deb80`,
and report digest
`86e881083d16ec6ed91959d08e9aafe787744dc45c38dea478b7e8ed3fb6e7b4`.

The canonical baseline cannot pin a feature-branch commit because protected
delivery uses squash merge. The implementation must merge first; a distinct
closeout will generate and pin the report from the durable protected squash.

## Protected implementation evidence

- PR #445 passed DCO, pull-request policy, every file-hygiene check, and the
  exact-head fast gate in 7m48s before guarded merge.
- The guarded squash merge produced protected
  `develop@a33b48ead4bb1c37d63f26775c0f47c77564b4fe`.
- The canonical v2 report generated from that exact protected revision has
  source fingerprint
  `aff2b1239416166e84c0d405c056943ae56d7f9b49a9033f32717d8a5ea16b49`,
  population projection
  `2757e495e1d40accbae481bdb00dba835584f6a9de01c0c4fed2d24b4f9deb80`,
  and report digest
  `f922df6c36ad6340e82d7a90f982a70d5958d9952b1d30c43905887f7e60a5eb`.
- The refreshed report contains no `PresentationDisclosurePlan::new` function
  signal and no presentation module signal. Every unrelated hotspot and
  disposition remains unchanged.
- PR #445 required no post-CI push, rerun, or retry.

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
