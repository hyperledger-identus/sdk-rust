# Verification evidence

## Identity

- Issue: #467
- Exact protected `develop` base:
  `6e4183a8e02ef0f2ed9984279d91e0660a36125f`
- Planning commit:
  `2333d3981813cce3fa2c70330c7c93177ada4241`
- Preimplementation receipt commit:
  `ed7b782c9fe656b1576ba6968be3a9cf73c886b2`
- Characterization commit:
  `99e5c5029ae666b5c44fad0418bb563076aabacc`
- Production implementation commit:
  `4036eed333be8a463f31674904ce9a36fe843c53`
- Reviewed head after protected-baseline synchronization:
  `10c084c265d12bdcc0e37644a3bdcf1c5d73a6d2`
- Synchronized protected `develop` baseline:
  `3b5c91d384756fd0ef1ee36ef3e66f051f5950ba`
- Exact implementation PR head:
  `ff57bba89f172788ad6c656a04ff1697891b981e`
- Protected implementation squash:
  `87fe079e26b2f7026bf89b2aef11d99617236545`

## Behavioral compatibility

- The complete pre-change Authorization Request suite passed: 7 tests.
- Preimplementation characterization binds existing-query collision/count
  failures before Authorization Details and final-size failures, and binds
  Authorization Details overflow before final URI overflow.
- The post-change focused suite passes: 8 tests. Existing exact URI bytes,
  managed parameter order, endpoint-query preservation, conditional locations,
  issuer state, resource boundaries, static errors, redaction, and request
  lineage remain green.

## Public, dependency, and resource compatibility

- `cargo-public-api 0.52.0` reports byte-for-byte identical simplified public
  inventories for `identus-oid4vci` at protected `develop` and the reviewed
  source head: 272,060 bytes each with digest
  `e03d241781d6d1fbe0d2335054c53a396e7b2b4eeba4ea3a6f88577388c616a4`.
- No manifest, feature, lockfile, dependency, unsafe, native, FFI,
  serialization, wire, public error, or protocol capability changed.
- The private owner borrows the same predecessor and endpoint/query/state
  values, owns the same zeroizing Authorization Details, retains the same one
  fixed-capacity parameter vector, and allocates the final zeroizing URI once
  at its checked exact length.
- Existing endpoint query count/name/value, Authorization Details, final URI,
  and predecessor limits remain the authoritative work and memory boundaries.
- Source-distribution verification passes for all five governed packages.

## Code-health comparison

| Signal | Protected base | Reviewed source head |
| --- | ---: | ---: |
| `try_into_authorization_request` SLOC / cognitive / cyclomatic | 60 / 7 / 18 | no function signal |
| Replacement assembly method signals | n/a | 0 |
| `authorization_request.rs` module signal | 0 | 0 |

The improvement is one private request-assembly owner aligned to the consuming
transition, not forwarding wrappers or one helper per parameter. The live
reviewed report has source fingerprint
`447498639b8c2a7c4fd8b75b7d624cc5a046d984d9852f41133bbfa1ee2932ff`,
population projection
`cfc603629e9ba2b2e33f9feb3d18b262a0618080ebfaaeebae892438598d2e53`,
and report digest
`cafa5504c7a68d9aff5da6b55b729412aff097b27dd072112553b34ad61ffffe`.

The canonical report regenerated from protected
`develop@87fe079e26b2f7026bf89b2aef11d99617236545` preserves that source
fingerprint and population projection. Its immutable report digest is
`6567e914eaf4f8687a2c1919d2db83e56ed82db3f65c375661042905c1ecb9a6`.

## Hosted delivery

- Implementation PR #469 merged through the guarded protected-branch path at
  exact head `ff57bba89f172788ad6c656a04ff1697891b981e`.
- DCO, pull-request policy, file hygiene, and the fast Rust lane passed; the
  critical Rust lane completed in 8 minutes 29 seconds with no retry or
  post-CI push.
- Bounded v2 delivery metrics are retained locally and published on PR #469.

## Local gates

- Focused Authorization Request tests: passed.
- Workspace tests with all features: passed.
- Strict workspace Clippy with all targets and features: passed.
- Workspace documentation with warnings denied: passed.
- `wasm32-unknown-unknown`, `aarch64-linux-android`, and
  `aarch64-apple-ios` Nix checks: passed.
- Public API comparison: identical simplified inventories.
- Source distribution, factory contract, OpenSpec, formatting, diff, and Nix
  flake evaluation gates: passed.
- Rust 1.89 MSRV Nix gate: passed.
- Canonical Rust 1.98 Nix nextest gate: passed.
