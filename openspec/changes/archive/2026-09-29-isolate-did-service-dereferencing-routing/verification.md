# Verification evidence

## Identity

- Issue: #461
- Exact protected `develop` base:
  `d27e455901c501d9611abbe0065e6a9b7270dd57`
- Planning commit:
  `4ed8b0d1b08f735ace9cd6a6970511e2caf85ac1`
- Preimplementation receipt commit:
  `ac1efae1840bcbef563da7b239bf265ecb95834d`
- Characterization commit:
  `40d3a84cf6c7db650fa2a2b3a9a5e1aa6740557e`
- Production implementation commit:
  `4fa552f834b5c280527c2594887962e3c176224f`
- Reviewed implementation head after protected-base synchronization:
  `77dcb43ddc1de55868f1181d48f75400c1aee0af`
- Protected implementation squash:
  `cbb8849ab50f39f7b4f8b873a692ede9861b3394`

## Behavioral compatibility

- The complete pre-change DID URL dereferencing suite passed: 13 tests passed
  and one release-only diagnostic was ignored.
- Preimplementation characterization binds selector expansion before media
  routing, empty selection before unsupported-representation handling,
  conjunctive service/type matching, source order, map-endpoint document
  retention, endpoint-map omission in URI output, and fragment-forced endpoint
  output even when a DID document representation is explicitly requested.
- The post-change suite passes with that matrix added: 15 tests passed and one
  release-only diagnostic was ignored.
- Existing traversal, relative-reference, endpoint-order, metadata,
  relationship, method-resource, redaction, concurrency, and resolver-call
  tests remain green.

## Public, dependency, and resource compatibility

- `cargo-public-api 0.52.0` with repository rustdoc JSON reports byte-for-byte
  identical simplified public inventories for `identus-did` at the protected
  base and implementation head: 374,952 bytes each.
- No manifest, feature, lockfile, dependency, unsafe, native, FFI,
  serialization, wire, media, adapter, or public error contract changed.
- The private context borrows the same DID URL, document, prepared request, and
  resolver content type while moving the same content metadata. It performs
  the same service clones and collection allocation and adds no collection,
  clone, callback, dynamic dispatch, trait bound, I/O, or unbounded work path.
- Existing DID URL, selector, document, endpoint, relative-reference, and JSON
  limits remain the authoritative work and memory boundaries.
- Source-distribution verification passes for all five governed packages.

## Code-health comparison

| Signal | Protected base | Reviewed implementation head |
| --- | ---: | ---: |
| `dereference_services` SLOC | 66 | no function signal |
| `dereference_services` cognitive complexity | 8 | no function signal |
| `dereference_services` cyclomatic complexity | 20 | no function signal |
| Replacement context method signals | n/a | 0 |
| `dereference.rs` module signal | 0 | 0 |

The improvement is one private `ServiceDereferencing` context with complete
selection and routing phase owners. Existing endpoint and filtered-document
helpers retain their invariants. It is not generated code, threshold
weakening, a waiver, a generic predicate framework, or a helper per match arm.
The live report for the reviewed head has source fingerprint
`01951d27f9e5fd3ca63d7447965d01cefb7edeac743e2e46034c9d94ec37c675`,
population projection
`2757e495e1d40accbae481bdb00dba835584f6a9de01c0c4fed2d24b4f9deb80`,
and report digest
`37c81b00c3407ddb4eb4bcfbe548b8b6b492c7ff7ade39f5bb0d33585c4acdd0`.

The canonical report regenerated from protected
`develop@cbb8849ab50f39f7b4f8b873a692ede9861b3394` preserves that source
fingerprint and population projection. Its immutable report digest is
`89200c46c04b7bca0d84dbfbce246a9d55e034cdc8462e8bed444e45200587b4`.

## Hosted delivery

- Implementation PR #463 merged through the guarded protected-branch path at
  exact head `0eb6e74f1bf9279672cbe21dcf1f7fa49138d790`.
- DCO, pull-request policy, file hygiene, and the fast Rust lane passed; the
  critical Rust lane completed in 6 minutes 17 seconds with no retry or
  post-CI push.
- Bounded v2 delivery metrics are retained locally and published on PR #463.

## Local gates

- Focused DID URL dereferencing tests: passed.
- Workspace tests with all features: passed.
- Strict workspace Clippy with all targets and features: passed.
- Workspace documentation with warnings denied: passed.
- `wasm32-unknown-unknown`, `aarch64-linux-android`, and
  `aarch64-apple-ios` workspace checks: passed.
- Public API comparison: identical simplified inventories.
- Source distribution, factory contract, OpenSpec, formatting, diff, and Nix
  flake evaluation gates: passed.
- Rust 1.89 MSRV Nix gate: passed.
- Canonical Rust 1.98 Nix nextest gate: 907 passed, 22 skipped.
