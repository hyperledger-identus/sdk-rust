# Verification evidence

## Identity

- Issue: #452
- Exact protected develop base:
  `3de0a6eac2b12c02ec80f9d0e17bded26648d506`
- Planning commit: `227388e98c9082e707575e8a40ab65ab3c2cf4fb`
- Preimplementation receipt commit: `5d34e49`
- Reviewed implementation head:
  `f4599a8fabd9c3567801d5c2f982f267c9758302`
- Protected-base synchronization: the signed merge at the reviewed head brings
  in the completed issue #450 canonical evidence and archive without touching
  the DID implementation or characterization.

## Behavioral compatibility

- The complete pre-change DID URL dereferencing suite passed: 12 tests passed
  and one release-only diagnostic was ignored.
- A preimplementation combined-fault matrix binds the publicly observable
  priority: malformed query and parameter failures remain `invalidDidUrl`
  before cross-field `invalidOptions`; selector-only invalidity remains
  `invalidOptions`; and every preparation failure leaves the resolver call
  count at zero.
- The post-change suite passes with that matrix added: 13 tests passed and one
  release-only diagnostic was ignored.
- Existing recording-resolver tests continue to prove exact typed projection,
  single-decoding/literal-plus behavior, decoded-name duplicate rejection,
  sorted application, extension retention, selector semantics, and exactly one
  resolver call for accepted requests.
- The private state clones and moves the same option, extension, selector, and
  typed values in the same phase order. Complete query parsing still precedes
  parameter application; `ResolutionOptions` construction still precedes both
  cross-field checks.

## Public, dependency, and resource compatibility

- `cargo-public-api 0.52.0` with the repository compiler and subprocess-scoped
  `RUSTC_BOOTSTRAP=1` reports no removed, changed, or added simplified public
  item between the protected base and implementation head.
- No manifest, feature, lockfile, dependency, unsafe, native, FFI,
  serialization, wire, W3C algorithm, or public error contract changed.
- No collection, clone, callback, dynamic dispatch, trait bound, decoding pass,
  or allocation class was added. The owner replaces existing locals with one
  private aggregate and a fieldless private dispatch enum.
- DID URL, parameter-text, typed scalar, relative-reference, and extension JSON
  limits remain the authoritative work and memory boundaries.
- Source-distribution verification passes for all five governed packages.

## Code-health comparison

| Signal | Protected base | Implementation head |
| --- | ---: | ---: |
| `PreparedRequest::new` SLOC | 127 | no constructor signal |
| `PreparedRequest::new` cognitive complexity | 18 | no constructor signal |
| `PreparedRequest::new` cyclomatic complexity | 43 | no constructor signal |
| Unrelated `dereference_services` signal | 66 / 8 / 20 | 66 / 8 / 20 |
| DID dereference module signals | 0 | 0 |

The improvement is one private `PreparedRequestBuilder` with cohesive
initialization, complete-query application, resolution/resource parameter, and
finalization phases. It is not generated code, threshold weakening, a waiver,
or a helper per conditional. The live report for the reviewed head has source
fingerprint
`e6ab173f16fd83a95661c1436c69886defd0c8dce6f7a86ce5505bfa0f14d593`,
population projection
`2757e495e1d40accbae481bdb00dba835584f6a9de01c0c4fed2d24b4f9deb80`,
and report digest
`990b65d7c1e77e070ad38beb4534be7d06cb3d41e7bff35ed3d4713b559120eb`.

The canonical baseline cannot pin a feature-branch commit because protected
delivery uses squash merge. The implementation must merge first; a distinct
closeout will generate and pin the report from the durable protected squash.

## Local gates

- Focused and complete `identus-did` tests: passed.
- Workspace tests with all features: passed.
- Strict workspace Clippy with all targets and features: passed.
- Workspace documentation with warnings denied: passed.
- `wasm32-unknown-unknown`, `aarch64-linux-android`, and
  `aarch64-apple-ios` workspace checks: passed.
- Public API comparison: no added, removed, or changed simplified item.
- Source distribution, factory contract, OpenSpec, formatting, diff, and Nix
  flake evaluation gates: passed.
- Canonical Nix `rust-test` gate: passed.
